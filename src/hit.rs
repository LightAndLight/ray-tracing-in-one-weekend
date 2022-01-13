use crate::{material::Material, ray::Ray, vec3::Vec3};

pub enum Face {
    Front,
    Back,
}

pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub face: Face,
    pub material: Material,
}

pub trait HasHit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

impl<T: HasHit> HasHit for &[T] {
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
}

impl<T: HasHit> HasHit for Vec<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.as_slice().hit(ray, t_min, t_max)
    }
}
