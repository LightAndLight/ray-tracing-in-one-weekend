use crate::{ray::Ray, vec3::Vec3};
use rand::Rng;

pub struct Camera {
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        v_fov: f64,
        up: &Vec3,
        look_from: &Vec3,
        look_at: &Vec3,
        aperture: f64,
        focal_distance: f64,
    ) -> Self {
        debug_assert!(!up.contains_nan(), "up: {:?}", up);
        debug_assert!(!look_from.contains_nan(), "look_from: {:?}", look_from);
        debug_assert!(!look_at.contains_nan(), "look_at: {:?}", look_at);

        let viewport_height = 2.0 * (v_fov.to_radians() / 2.0).tan();
        let viewport_width = viewport_height * aspect_ratio;

        let origin = *look_from;

        let w = (*look_from - *look_at).unit();
        debug_assert!(!w.contains_nan(), "w: {:?}", w);

        if up.unit() == w {
            panic!("up vector and look direction are the same");
        }

        let u = up.cross(w).unit();
        debug_assert!(!u.contains_nan(), "u: {:?}", u);

        let v = w.cross(u).unit();
        debug_assert!(!v.contains_nan(), "v: {:?}", v);

        let horizontal = focal_distance * viewport_width * u;
        let vertical = focal_distance * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focal_distance * w;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        // Compute a random point inside the unit disc
        fn random_in_unit_disc() -> Vec3 {
            let mut rng = rand::thread_rng();
            loop {
                let p = Vec3 {
                    x: rng.gen_range(-1.0..1.0),
                    y: rng.gen_range(-1.0..1.0),
                    z: 0.0,
                };
                if p.norm_squared() >= 1.0 {
                    continue;
                } else {
                    return p;
                }
            }
        }

        let point_on_lens = self.lens_radius * random_in_unit_disc();
        debug_assert!(
            !point_on_lens.contains_nan(),
            "point_on_lens: {:?}",
            point_on_lens
        );

        debug_assert!(!self.u.contains_nan(), "self.u: {:?}", self.u);
        debug_assert!(!self.v.contains_nan(), "self.v: {:?}", self.v);
        let offset = point_on_lens.x * self.u + point_on_lens.y * self.v;
        debug_assert!(!offset.contains_nan(), "offset: {:?}", offset);

        let origin = self.origin + offset;
        debug_assert!(!origin.contains_nan(), "origin: {:?}", origin);

        let direction =
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset;
        debug_assert!(!direction.contains_nan(), "direction: {:?}", direction);

        Ray { origin, direction }
    }
}
