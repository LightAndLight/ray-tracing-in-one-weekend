use crate::{
    bounds::Bounds3,
    hittable::{Hit, Hittable, HittableList},
    ray::Ray,
    vec3::Vec3,
};
use std::sync::Arc;

pub enum Bvh {
    Empty,
    Node(BvhNode),
}

impl From<&[Arc<dyn Hittable + Sync + Send>]> for Bvh {
    fn from(items: &[Arc<dyn Hittable + Sync + Send>]) -> Self {
        if items.is_empty() {
            return Bvh::Empty;
        }

        #[derive(Clone)]
        struct ItemInfo {
            bounds: Bounds3,
            centroid: Vec3,
            item: Arc<dyn Hittable + Sync + Send>,
        }

        let item_infos: Vec<ItemInfo> = items
            .iter()
            .cloned()
            .map(|item| {
                let bounds = item.bounds();
                let centroid = 0.5 * bounds.min() + 0.5 * bounds.max();
                ItemInfo {
                    bounds,
                    centroid,
                    item,
                }
            })
            .collect();

        fn build(item_infos: &[ItemInfo]) -> BvhNode {
            let bounds = item_infos
                .iter()
                .fold(Bounds3::point(Vec3::origin()), |acc, el| {
                    acc.union(&el.bounds)
                });

            if item_infos.len() == 1 {
                BvhNode::Leaf {
                    bounds,
                    items: HittableList::from([item_infos[0].item.clone()]),
                }
            } else {
                let centroid_bounds = {
                    assert!(item_infos.len() > 1);
                    let init = Bounds3::point(item_infos[0].centroid);

                    item_infos[1..]
                        .iter()
                        .fold(init, |acc, el| acc.union(&Bounds3::point(el.centroid)))
                };

                let partition_axis = centroid_bounds.maximum_extent();

                if centroid_bounds.min()[partition_axis] == centroid_bounds.max()[partition_axis] {
                    BvhNode::Leaf {
                        bounds,
                        items: HittableList::from(
                            item_infos
                                .iter()
                                .map(|item_info| item_info.item.clone())
                                .collect::<Vec<Arc<dyn Hittable + Send + Sync>>>()
                                .as_slice(),
                        ),
                    }
                } else {
                    let midpoint = centroid_bounds.centroid();
                    let (left_item_infos, right_item_infos) = item_infos
                        .iter()
                        .cloned()
                        .partition::<Vec<ItemInfo>, _>(|item_info| {
                            item_info.centroid[partition_axis] < midpoint[partition_axis]
                        });

                    assert!(
                        left_item_infos.len() < item_infos.len(),
                        "partition axis: {:?}\nmidpoint: {:?}\nitem centroids: {:?}",
                        partition_axis,
                        midpoint,
                        item_infos
                            .iter()
                            .map(|item_info| item_info.centroid)
                            .collect::<Vec<Vec3>>()
                    );
                    let left = build(&left_item_infos);
                    let right = build(&right_item_infos);

                    BvhNode::branch(left, right)
                }
            }
        }

        Bvh::Node(build(&item_infos))
    }
}

impl Hittable for Bvh {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        match self {
            Bvh::Empty => None,
            Bvh::Node(node) => node.hit_by(ray, t_min, t_max),
        }
    }

    fn bounds(&self) -> Bounds3 {
        match self {
            Bvh::Empty => Bounds3::point(Vec3::origin()),
            Bvh::Node(node) => node.bounds(),
        }
    }
}

pub enum BvhNode {
    Branch {
        bounds: Bounds3,
        left: Arc<BvhNode>,
        right: Arc<BvhNode>,
    },
    Leaf {
        bounds: Bounds3,
        items: HittableList,
    },
}

impl BvhNode {
    fn bounds(&self) -> Bounds3 {
        match self {
            BvhNode::Branch { bounds, .. } => *bounds,
            BvhNode::Leaf { bounds, .. } => *bounds,
        }
    }

    fn branch(left: BvhNode, right: BvhNode) -> Self {
        let bounds = left.bounds().union(&right.bounds());
        BvhNode::Branch {
            bounds,
            left: Arc::new(left),
            right: Arc::new(right),
        }
    }
}

impl Hittable for BvhNode {
    fn hit_by(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        match self {
            BvhNode::Branch {
                bounds,
                left,
                right,
            } => {
                if bounds.hit_by(ray, t_min, t_max) {
                    match left.hit_by(ray, t_min, t_max) {
                        Some(left_hit) => right.hit_by(ray, t_min, left_hit.t).or(Some(left_hit)),
                        None => right.hit_by(ray, t_min, t_max),
                    }
                } else {
                    None
                }
            }
            BvhNode::Leaf { bounds, items } => {
                if bounds.hit_by(ray, t_min, t_max) {
                    items.hit_by(ray, t_min, t_max)
                } else {
                    None
                }
            }
        }
    }

    fn bounds(&self) -> Bounds3 {
        todo!()
    }
}
