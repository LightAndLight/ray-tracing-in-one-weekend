use crate::{bounds::Bounds3, hit::Hit, ray::Ray, vec3::Vec3};
use std::sync::Arc;

pub trait IsObject: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
    fn bounds(&self) -> Bounds3;
}

#[derive(Clone)]
pub struct Object(Arc<dyn IsObject>);

impl Object {
    pub fn new<T: IsObject + 'static>(item: T) -> Self {
        Object(Arc::new(item))
    }
}

impl IsObject for Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.0.hit(ray, t_min, t_max)
    }

    fn bounds(&self) -> Bounds3 {
        self.0.bounds()
    }
}

impl<T: IsObject> IsObject for &[T] {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut result = None;
        let mut closest_so_far = t_max;
        for object in self.iter() {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                result = Some(hit);
            }
        }
        result
    }

    fn bounds(&self) -> Bounds3 {
        if self.is_empty() {
            Bounds3::point(Vec3::ZERO)
        } else {
            let init = self[0].bounds();
            self.iter().fold(init, |acc, el| acc.union(&el.bounds()))
        }
    }
}

impl<T: IsObject> IsObject for Vec<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.as_slice().hit(ray, t_min, t_max)
    }

    fn bounds(&self) -> Bounds3 {
        self.as_slice().bounds()
    }
}
