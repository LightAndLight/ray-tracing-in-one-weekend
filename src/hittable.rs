use crate::{
    bounds::{Bounded, Bounds3},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};
use std::sync::Arc;

pub enum Face {
    Front,
    Back,
}

pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub face: Face,
    pub material: Arc<dyn Material + Sync + Send>,
}

pub trait Hittable: Bounded {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable + Sync + Send>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Sync + Send>) {
        self.objects.push(object)
    }
}

impl From<&[Arc<dyn Hittable + Sync + Send>]> for HittableList {
    fn from(items: &[Arc<dyn Hittable + Sync + Send>]) -> Self {
        HittableList {
            objects: Vec::from(items),
        }
    }
}

impl<const N: usize> From<[Arc<dyn Hittable + Sync + Send>; N]> for HittableList {
    fn from(items: [Arc<dyn Hittable + Sync + Send>; N]) -> Self {
        HittableList {
            objects: Vec::from(items),
        }
    }
}

impl From<Vec<Arc<dyn Hittable + Sync + Send>>> for HittableList {
    fn from(objects: Vec<Arc<dyn Hittable + Sync + Send>>) -> Self {
        HittableList { objects }
    }
}

impl Bounded for HittableList {
    fn bounds(&self) -> Bounds3 {
        self.objects.as_slice().bounds()
    }
}

impl Hittable for HittableList {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut result = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(hit) = object.hit_by(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                result = Some(hit);
            }
        }
        result
    }
}

impl AsRef<[Arc<dyn Hittable + Send + Sync>]> for HittableList {
    fn as_ref(&self) -> &[Arc<dyn Hittable + Send + Sync>] {
        &self.objects
    }
}
