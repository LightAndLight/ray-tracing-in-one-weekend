mod camera;
mod color;
mod hittable;
mod image;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::Color;
use hittable::{Hittable, HittableList};
use image::Image;
use material::{Dielectric, Lambertian, Material, Metal};
use rand::{prelude::ThreadRng, Rng};
use ray::Ray;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, ParallelExtend, ParallelIterator,
};
use sphere::Sphere;
use std::{io, sync::Arc};
use vec3::Vec3;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian {
        albedo: Color {
            r: 0.5,
            g: 0.5,
            b: 0.5,
        },
    });

    world.add(Arc::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        material: ground_material,
    }));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3 {
                x: a as f64 + 0.9 * rng.gen::<f64>(),
                y: 0.2,
                z: b as f64 + 0.9 * rng.gen::<f64>(),
            };
            if (center
                - Vec3 {
                    x: 4.0,
                    y: 0.2,
                    z: 0.0,
                })
            .norm()
                > 0.9
            {
                let sphere_material: Arc<dyn Material + Sync + Send>;

                if choose_mat < 0.8 {
                    let albedo = rng.gen::<Color>() * rng.gen::<Color>();
                    sphere_material = Arc::new(Lambertian { albedo });
                } else if choose_mat < 0.95 {
                    let albedo = rng.gen::<Color>();
                    let fuzziness = rng.gen_range(0.0..0.5);
                    sphere_material = Arc::new(Metal { albedo, fuzziness });
                } else {
                    sphere_material = Arc::new(Dielectric {
                        refractive_index: 1.5,
                    });
                }

                world.add(Arc::new(Sphere {
                    center,
                    radius: 0.2,
                    material: sphere_material,
                }))
            }
        }
    }

    world.add(Arc::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: Arc::new(Dielectric {
            refractive_index: 1.5,
        }),
    }));

    world.add(Arc::new(Sphere {
        center: Vec3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: Arc::new(Lambertian {
            albedo: Color {
                r: 0.4,
                g: 0.2,
                b: 0.1,
            },
        }),
    }));

    world.add(Arc::new(Sphere {
        center: Vec3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: Arc::new(Metal {
            albedo: Color {
                r: 0.7,
                g: 0.6,
                b: 0.5,
            },
            fuzziness: 0.0,
        }),
    }));

    world
}

fn ray_color(rng: &mut ThreadRng, ray: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth == 0 {
        return Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
    }

    if let Some(hit) = world.hit_by(ray, 0.001, f64::INFINITY) {
        let material = hit.material.as_ref();

        match material.scatter(rng, ray, &hit) {
            Some(scatter) => {
                scatter.attenuation * ray_color(rng, &scatter.outgoing, world, depth - 1)
            }
            None => Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            },
        }
    } else {
        let unit_direction = ray.direction.unit();
        let white = Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        };
        let blue = Color {
            r: 0.5,
            g: 0.7,
            b: 1.0,
        };
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * white + t * blue
    }
}

fn main() {
    let look_from = Vec3 {
        x: 13.0,
        y: 2.0,
        z: 3.0,
    };
    let look_at = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let camera = Camera::new(
        3.0 / 2.0,
        20.0,
        &Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        &look_from,
        &look_at,
        0.1,
        10.0,
    );

    let image_width: usize = 1200;
    let image_height: usize = (image_width as f64 / camera.aspect_ratio()) as usize;

    let world = random_scene();

    let samples_per_pixel = 500;
    let samples_per_pixel_f64 = samples_per_pixel as f64;
    let max_depth = 50;
    let world_ref = &world;
    let camera_ref = &camera;
    let x_total = (image_width - 1) as f64;
    let y_total = (image_height - 1) as f64;
    let image = Image {
        width: image_width,
        height: image_height,
        data: {
            let mut data = Vec::with_capacity(image_width * image_height);

            eprintln!("number of cores: {}", num_cpus::get_physical());
            let threads = 2;
            eprintln!("using {} threads", threads);
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build_global()
                .unwrap();

            data.par_extend((0..image_height).into_par_iter().rev().flat_map(|y| {
                // eprint!("\r\x1B[0K");
                // eprint!("rows remaining: {}", y);
                let y = y as f64;

                (0..image_width).into_par_iter().map(move |x| {
                    let mut rng = rand::thread_rng();

                    let mut color = Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                    };
                    let x = x as f64;

                    for _ in 0..samples_per_pixel {
                        let u = (x + rng.gen::<f64>()) / x_total;
                        let v = (y + rng.gen::<f64>()) / y_total;
                        let ray = camera_ref.get_ray(u, v);
                        color += ray_color(&mut rng, &ray, world_ref, max_depth);
                    }

                    (color / samples_per_pixel_f64).sqrt()
                })
            }));

            data
        },
    };

    image.render(&mut io::stdout()).expect("render failed");
}
