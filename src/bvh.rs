use crate::{
    bounds::{Bounds3, HasBounds},
    hit::{HasHit, Hit},
    object::Object,
    ray::Ray,
    vec3::Vec3,
};
use std::sync::Arc;

pub enum Bvh {
    Empty,
    Node(BvhNode),
}

impl From<&[Object]> for Bvh {
    fn from(items: &[Object]) -> Self {
        if items.is_empty() {
            return Bvh::Empty;
        }

        #[derive(Clone, Copy)]
        struct ItemWithInfo {
            bounds: Bounds3,
            centroid: Vec3,
            item: usize,
        }

        let items_with_info: Vec<ItemWithInfo> = items
            .iter()
            .enumerate()
            .map(|(ix, item)| {
                let bounds = item.bounds();
                ItemWithInfo {
                    bounds,
                    centroid: bounds.centroid(),
                    item: ix,
                }
            })
            .collect();

        fn build(items: &[Object], items_with_info: &[ItemWithInfo]) -> BvhNode {
            assert!(!items.is_empty());

            /*
            Every `BvhNode` has an associated bounding box. The bounding box contains all
            of the node's items, so if an array does not intersect the bounding box then it
            does not intersect any of the items.
            */
            let bounds = {
                assert!(!items_with_info.is_empty());
                let init = items_with_info[0].bounds;

                items_with_info[1..]
                    .iter()
                    .fold(init, |bounds, item_with_info| {
                        bounds.union(&item_with_info.bounds)
                    })
            };

            if items_with_info.len() == 1 {
                BvhNode::Leaf {
                    bounds,
                    items: items_with_info
                        .iter()
                        .map(|item_with_info| items[item_with_info.item].clone())
                        .collect::<Vec<_>>(),
                }
            } else {
                let centroid_bounds = {
                    assert!(!items_with_info.is_empty());
                    let init = Bounds3::point(items_with_info[0].centroid);

                    items_with_info[1..]
                        .iter()
                        .fold(init, |centroid_bounds, item_with_info| {
                            centroid_bounds.union(&Bounds3::point(item_with_info.centroid))
                        })
                };

                let partition_axis = centroid_bounds.maximum_extent();

                // The items' centroids coincide, so they cannot be partitioned in space.
                if centroid_bounds.min()[partition_axis] == centroid_bounds.max()[partition_axis] {
                    BvhNode::Leaf {
                        bounds,
                        items: items_with_info
                            .iter()
                            .map(|item_with_info| items[item_with_info.item].clone())
                            .collect::<Vec<_>>(),
                    }
                } else {
                    let midpoint = centroid_bounds.centroid();
                    let (items_with_info_left, items_with_info_right) = items_with_info
                        .iter()
                        .copied()
                        .partition::<Vec<ItemWithInfo>, _>(|item_with_info| {
                            item_with_info.centroid[partition_axis] < midpoint[partition_axis]
                        });

                    assert!(items_with_info_left.len() < items.len());
                    let left = build(items, &items_with_info_left);
                    let right = build(items, &items_with_info_right);

                    BvhNode::branch(left, right)
                }
            }
        }

        Bvh::Node(build(items, &items_with_info))
    }
}

impl HasBounds for Bvh {
    fn bounds(&self) -> Bounds3 {
        match self {
            Bvh::Empty => Bounds3::point(Vec3::origin()),
            Bvh::Node(node) => node.bounds(),
        }
    }
}

impl HasHit for Bvh {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        match self {
            Bvh::Empty => None,
            Bvh::Node(node) => node.hit(ray, t_min, t_max),
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
        items: Vec<Object>,
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

impl HasBounds for BvhNode {
    fn bounds(&self) -> Bounds3 {
        match self {
            BvhNode::Branch { bounds, .. } => *bounds,
            BvhNode::Leaf { bounds, .. } => *bounds,
        }
    }
}

impl HasHit for BvhNode {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        match self {
            BvhNode::Branch {
                bounds,
                left,
                right,
            } => {
                if bounds.hit_by(ray, t_min, t_max) {
                    match left.hit(ray, t_min, t_max) {
                        Some(left_hit) => right.hit(ray, t_min, left_hit.t).or(Some(left_hit)),
                        None => right.hit(ray, t_min, t_max),
                    }
                } else {
                    None
                }
            }
            BvhNode::Leaf { bounds, items } => {
                if bounds.hit_by(ray, t_min, t_max) {
                    items.hit(ray, t_min, t_max)
                } else {
                    None
                }
            }
        }
    }
}
