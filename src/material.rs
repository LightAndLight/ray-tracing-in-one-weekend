use crate::{
    color::Color,
    hittable::{Face, Hit},
    ray::Ray,
    vec3::Vec3,
};
use rand::{prelude::ThreadRng, Rng};

pub struct Scatter {
    pub attenuation: Color,
    pub outgoing: Ray,
}

pub trait Material {
    /// Scatter a `ray` that has `hit` a material.
    fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Option<Scatter>;
}

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    loop {
        let p = Vec3::gen_range(rng, -1.0..1.0);
        if p.norm_squared() >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}

pub struct DiffuseHack {
    pub albedo: Color,
}

impl Material for DiffuseHack {
    fn scatter(&self, rng: &mut ThreadRng, _: &Ray, hit: &Hit) -> Option<Scatter> {
        Some(Scatter {
            attenuation: self.albedo,
            outgoing: Ray {
                origin: hit.point,
                direction: hit.normal + random_in_unit_sphere(rng),
            },
        })
    }
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, rng: &mut ThreadRng, _: &Ray, hit: &Hit) -> Option<Scatter> {
        fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
            loop {
                let p = Vec3::gen_range(rng, -1.0..1.0);
                if p.norm_squared() >= 1.0 {
                    continue;
                } else {
                    return p;
                }
            }
        }

        let direction = {
            let mut direction = hit.normal + random_in_unit_sphere(rng).unit();
            if direction.near_zero() {
                direction = hit.normal;
            }
            direction
        };

        Some(Scatter {
            attenuation: self.albedo,
            outgoing: Ray {
                origin: hit.point,
                direction,
            },
        })
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzziness: f64,
}

impl Material for Metal {
    fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let direction =
            ray.direction.reflect(&hit.normal) + self.fuzziness * random_in_unit_sphere(rng);

        if direction.dot(hit.normal) > 0.0 {
            Some(Scatter {
                attenuation: self.albedo,
                outgoing: Ray {
                    origin: hit.point,
                    direction,
                },
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refractive_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let attenuation = Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        };
        let (outside_refractive_index, inside_refractive_index) = match hit.face {
            Face::Front => (1.0, self.refractive_index),
            Face::Back => (self.refractive_index, 1.0),
        };

        fn reflectance(cos_theta: f64, refractive_index: f64) -> f64 {
            let r_0 = (1.0 - refractive_index) / (1.0 + refractive_index);
            r_0 + (1.0 - r_0) * (1.0 - cos_theta).powi(5)
        }

        let cos_theta = ray.direction.unit().negate().dot(hit.normal);
        let direction = if reflectance(cos_theta, self.refractive_index) > rng.gen::<f64>() {
            ray.direction.reflect(&hit.normal)
        } else {
            match ray.direction.unit().refract(
                &hit.normal,
                outside_refractive_index,
                inside_refractive_index,
            ) {
                Some(direction) => direction,
                None => ray.direction.reflect(&hit.normal),
            }
        };
        let outgoing = Ray {
            origin: hit.point,
            direction,
        };
        Some(Scatter {
            attenuation,
            outgoing,
        })
    }
}
