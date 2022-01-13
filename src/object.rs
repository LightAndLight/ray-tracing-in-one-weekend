use crate::{
    bounds::{Bounds3, HasBounds},
    hit::{HasHit, Hit},
    ray::Ray,
};
use std::sync::Arc;

trait IsObject: HasHit + HasBounds {}

impl<T: HasHit + HasBounds> IsObject for T {}

#[derive(Clone)]
pub struct Object(Arc<dyn IsObject + Sync + Send>);

impl Object {
    pub fn new<T: HasHit + HasBounds + Send + Sync + 'static>(item: T) -> Self {
        Object(Arc::new(item))
    }
}

impl HasBounds for Object {
    fn bounds(&self) -> Bounds3 {
        self.0.bounds()
    }
}

impl HasHit for Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.0.hit(ray, t_min, t_max)
    }
}
