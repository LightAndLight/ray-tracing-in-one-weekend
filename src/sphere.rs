use crate::{
    bounds::{Bounds3, HasBounds},
    hit::{Face, HasHit, Hit},
    material::Material,
    ray::Ray,
    texture,
    vec3::Vec3,
};
use std::f64::consts as f64;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl HasBounds for Sphere {
    fn bounds(&self) -> Bounds3 {
        let corner = Vec3 {
            x: self.radius,
            y: self.radius,
            z: self.radius,
        };
        Bounds3::new(self.center - corner, self.center + corner)
    }
}

impl HasHit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        debug_assert!(
            !(ray.origin.x.is_nan()
                || ray.origin.y.is_nan()
                || ray.origin.z.is_nan()
                || ray.direction.x.is_nan()
                || ray.direction.y.is_nan()
                || ray.direction.z.is_nan()),
            "ray: {:?}",
            ray
        );

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
            debug_assert!(
                !(point.x.is_nan() || point.y.is_nan() || point.z.is_nan()),
                "point: {:?}, ray origin: {:?}, ray direction: {:?}, t: {:?}",
                point,
                ray.origin,
                ray.direction,
                t
            );

            let outward_normal = (point - self.center) / self.radius;
            debug_assert!(
                !(outward_normal.x.is_nan()
                    || outward_normal.y.is_nan()
                    || outward_normal.z.is_nan()),
                "outward_normal: {:?}, point: {:?}, center: {:?}, radius: {:?}",
                outward_normal,
                point,
                self.center,
                self.radius
            );

            let (normal, face) = if ray.direction.dot(outward_normal) < 0.0 {
                (outward_normal, Face::Front)
            } else {
                (-outward_normal, Face::Back)
            };
            debug_assert!(
                !(normal.x.is_nan() || normal.y.is_nan() || normal.z.is_nan()),
                "normal: {:?}",
                normal,
            );

            let phi = (-normal.z).atan2(normal.x) + f64::PI;
            debug_assert!(phi >= 0.0, "phi: {:?}, normal: {:?}", phi, normal);
            debug_assert!(phi < 2.0 * f64::PI, "phi: {:?}", phi);

            let theta = (-normal.y).acos();
            debug_assert!(theta >= 0.0, "theta: {:?}", theta);
            debug_assert!(theta < f64::PI, "theta: {:?}", theta);

            let u = phi / (2.0 * f64::PI);
            debug_assert!(u >= 0.0, "u: {:?}", u);
            debug_assert!(u < 1.0, "u: {:?}", u);

            let v = theta / f64::PI;
            debug_assert!(v >= 0.0, "v: {:?}", v);
            debug_assert!(v < 1.0, "v: {:?}", v);

            let texture_coord = texture::Coord { u, v };

            Some(Hit {
                point,
                normal,
                t,
                face,
                material: self.material.clone(),
                texture_coord,
            })
        }
    }
}
