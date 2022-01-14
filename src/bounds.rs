use crate::{axis::Axis3, interval::Interval, ray::Ray, vec3::Vec3};

#[derive(Clone, Copy)]
pub struct Bounds3 {
    min: Vec3,
    max: Vec3,
}

impl Bounds3 {
    /// Construct a bounding box from two corner points.
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Bounds3 {
            min: Vec3 {
                x: a.x.min(b.x),
                y: a.y.min(b.y),
                z: a.z.min(b.z),
            },
            max: Vec3 {
                x: a.x.max(b.x),
                y: a.y.max(b.y),
                z: a.z.max(b.z),
            },
        }
    }

    /// Get the min-corner of the box.
    pub fn min(&self) -> &Vec3 {
        &self.min
    }

    /// Get the max-corner of the box.
    pub fn max(&self) -> &Vec3 {
        &self.max
    }

    /// Create a zero-volume bounding box at a point.
    pub fn point(v: Vec3) -> Self {
        Bounds3 { min: v, max: v }
    }

    /// Compute the union of two bounding boxes.
    #[must_use]
    pub fn union(&self, other: &Bounds3) -> Self {
        Bounds3 {
            min: Vec3 {
                x: self.min.x.min(other.min.x),
                y: self.min.y.min(other.min.y),
                z: self.min.z.min(other.min.z),
            },
            max: Vec3 {
                x: self.max.x.max(other.max.x),
                y: self.max.y.max(other.max.y),
                z: self.max.z.max(other.max.z),
            },
        }
    }

    /// The vector from the min-corner to the max-corner.
    pub fn diagonal(&self) -> Vec3 {
        self.max - self.min
    }

    /// The dimension in which the bounding box is longest.
    pub fn maximum_extent(&self) -> Axis3 {
        let diagonal = self.diagonal();

        if diagonal.x > diagonal.y && diagonal.x > diagonal.z {
            Axis3::X
        } else if diagonal.y > diagonal.z {
            Axis3::Y
        } else {
            Axis3::Z
        }
    }

    /// The center of the box.
    pub fn centroid(&self) -> Vec3 {
        0.5 * self.min + 0.5 * self.max
    }

    pub fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        /*
        The values of `t` for which the ray is inside the bounding box.

        By the end of this function, the interval will be non-empty iff the ray
        intersects the box.
        */
        let mut t_interval = Interval {
            start: t_min,
            end: t_max,
        };

        for axis in [Axis3::X, Axis3::Y, Axis3::Z] {
            let parallel_to_slab = ray.direction[axis] == 0.0;

            if parallel_to_slab {
                let inside_slab =
                    ray.origin[axis] >= self.min[axis] && ray.origin[axis] <= self.max[axis];

                if inside_slab {
                    continue;
                } else {
                    return false;
                }
            } else {
                /*
                The ray is not parallel to the slab, so it must intersect eventually.

                The two intersection points are used to refine `t_interval`.
                */

                let inverse_ray_direction = 1.0 / ray.direction[axis];

                // origin_axis + t*direction_axis = min_axis
                let t_for_axis_min = (self.min[axis] - ray.origin[axis]) * inverse_ray_direction;
                // origin_axis + t*direction_axis = max_axis
                let t_for_axis_max = (self.max[axis] - ray.origin[axis]) * inverse_ray_direction;

                /*
                Because the ray intesects the slab, there *must* be a non-empty interval
                for `t`. [t_for_axis_min, t_for_axis_max] might be empty, though. For any
                ray that first hits the min-slab, then hits the max-slab, there exists other
                rays (coming from the other side) that hit the slabs at the same points, but
                in the opposite order.
                */
                let t_interval_for_axis = Interval {
                    start: t_for_axis_min.min(t_for_axis_max),
                    end: t_for_axis_min.max(t_for_axis_max),
                };
                t_interval.intersect_mut(&t_interval_for_axis);
            }
        }

        !t_interval.is_empty()
    }
}

pub trait HasBounds {
    fn bounds(&self) -> Bounds3;
}

impl<T: HasBounds> HasBounds for &[T] {
    fn bounds(&self) -> Bounds3 {
        if self.is_empty() {
            Bounds3::point(Vec3::origin())
        } else {
            let init = self[0].bounds();
            self.iter().fold(init, |acc, el| acc.union(&el.bounds()))
        }
    }
}

impl<T: HasBounds> HasBounds for Vec<T> {
    fn bounds(&self) -> Bounds3 {
        self.as_slice().bounds()
    }
}
