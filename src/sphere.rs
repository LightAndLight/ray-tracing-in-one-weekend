use std::sync::Arc;

use crate::{
    bounds::Bounds3,
    hittable::{Face, Hit, Hittable},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material + Sync + Send>,
}

impl Hittable for Sphere {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let a = ray.direction.norm_squared();
        let half_b = ray.direction.dot(ray.origin - self.center);
        let c = (ray.origin - self.center).norm_squared() - self.radius.powi(2);
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let mut t = (-half_b - discriminant.sqrt()) / a;
            if t < t_min || t_max < t {
                t = (-half_b + discriminant.sqrt()) / a;
                if t < t_min || t_max < t {
                    return None;
                }
            }

            let point = ray.at(t);
            let outward_normal = (point - self.center) / self.radius;
            let (normal, face) = if ray.direction.dot(outward_normal) < 0.0 {
                (outward_normal, Face::Front)
            } else {
                (-outward_normal, Face::Back)
            };
            Some(Hit {
                point,
                normal,
                t,
                face,
                material: self.material.clone(),
            })
        }
    }

    fn bounds(&self) -> Bounds3 {
        let corner = Vec3 {
            x: self.radius,
            y: self.radius,
            z: self.radius,
        };
        Bounds3::new(self.center - corner, self.center + corner)
    }
}
