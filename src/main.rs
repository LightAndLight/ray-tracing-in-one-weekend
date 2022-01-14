mod cli;

use clap::Parser;
use cli::{Cli, Dimensions};
use rand::{prelude::ThreadRng, Rng};
use rt_weekend::{
    bvh::Bvh,
    camera::Camera,
    color::Color,
    hit::HasHit,
    image::Image,
    material::{Dielectric, IsMaterial, Lambertian, Material, Metal},
    object::Object,
    ray::Ray,
    sphere::Sphere,
    texture::{self, Texture},
    vec3::Vec3,
};
use std::{io, sync::Arc, thread};

fn random_scene() -> Vec<Object> {
    let mut world = Vec::new();

    let ground_material = Material::new(Lambertian {
        albedo: Texture::new(texture::Constant {
            color: Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
            },
        }),
    });

    world.push(Object::new(Sphere {
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
                let sphere_material: Material;

                if choose_mat < 0.8 {
                    let albedo = Texture::new(texture::Constant {
                        color: rng.gen::<Color>() * rng.gen::<Color>(),
                    });
                    sphere_material = Material::new(Lambertian { albedo });
                } else if choose_mat < 0.95 {
                    let albedo = rng.gen::<Color>();
                    let fuzziness = rng.gen_range(0.0..0.5);
                    sphere_material = Material::new(Metal { albedo, fuzziness });
                } else {
                    sphere_material = Material::new(Dielectric {
                        refractive_index: 1.5,
                    });
                }

                world.push(Object::new(Sphere {
                    center,
                    radius: 0.2,
                    material: sphere_material,
                }))
            }
        }
    }

    /*
    world.push(Object::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 20.0,
            z: 0.0,
        },
        radius: 1.0,
        material: Material::new(Light {
            brightness: 200.0,
            color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
        }),
    }));
    */

    world.push(Object::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: Material::new(Dielectric {
            refractive_index: 1.5,
        }),
    }));

    world.push(Object::new(Sphere {
        center: Vec3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: Material::new(Lambertian {
            albedo: Texture::new(texture::Constant {
                color: Color {
                    r: 0.4,
                    g: 0.2,
                    b: 0.1,
                },
            }),
        }),
    }));

    /*
    world.push(Object::new(Sphere {
        center: Vec3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: Material::new(Metal {
            albedo: Color {
                r: 0.7,
                g: 0.6,
                b: 0.5,
            },
            fuzziness: 0.0,
        }),
    }));
    */

    /*
    world.push(Object::new(Sphere {
        center: Vec3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: Material::new(Lambertian {
            albedo: Texture::new(texture::UV()),
        }),
    }));
    */

    world.push(Object::new(Sphere {
        center: Vec3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: Material::new(Lambertian {
            albedo: Texture::new(texture::Image::new("earth.png")),
        }),
    }));

    world
}

fn ray_color(rng: &mut ThreadRng, ray: &Ray, world: &dyn HasHit, depth: usize) -> Color {
    if depth == 0 {
        return Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
    }

    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        let material = &hit.material;
        let emittance = material.emit();

        match material.scatter(rng, ray, &hit) {
            Some(scatter) => {
                emittance
                    + scatter.attenuation * ray_color(rng, &scatter.outgoing, world, depth - 1)
            }
            None => emittance,
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

fn get_pixel_color(
    rng: &mut ThreadRng,
    camera: &Camera,
    world: &dyn HasHit,
    recursion_depth: usize,
    rays_per_pixel: usize,
    rays_per_pixel_f64: f64,
    x: f64,
    y: f64,
    x_total: f64,
    y_total: f64,
) -> Color {
    let mut color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    let x = x as f64;

    for _ in 0..rays_per_pixel {
        let u = (x + rng.gen::<f64>()) / x_total;
        let v = (y + rng.gen::<f64>()) / y_total;
        let ray = camera.get_ray(u, v);
        color += ray_color(rng, &ray, world, recursion_depth);
    }

    (color / rays_per_pixel_f64).sqrt()
}

fn main() {
    let cli = Cli::parse();

    let num_threads = cli.num_threads.unwrap_or_else(num_cpus::get_physical);
    let rays_per_pixel = cli.rays_per_pixel;
    let recursion_depth = cli.recursion_depth;
    let Dimensions {
        width: image_width,
        height: image_height,
    } = cli.dimensions;
    let aspect_ratio = image_width as f64 / image_height as f64;

    let look_from = Vec3 {
        x: 13.0,
        y: 2.0,
        z: 3.0,
    };
    let look_at = Vec3::origin();
    let up = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let camera = Camera::new(aspect_ratio, 20.0, &up, &look_from, &look_at, 0.1, 10.0);

    let world = Bvh::from(random_scene().as_ref());

    let rays_per_pixel_f64 = rays_per_pixel as f64;
    let world_ref = Arc::new(world);
    let camera_ref = Arc::new(camera);
    let x_total = (image_width - 1) as f64;
    let y_total = (image_height - 1) as f64;

    let data = {
        eprintln!("Using {} threads.", num_threads);

        let outputs_reciever = {
            let (inputs_sender, inputs_reciever) = crossbeam_channel::unbounded::<usize>();
            let (outputs_sender, outputs_reciever) =
                crossbeam_channel::unbounded::<(usize, Vec<Color>)>();

            for _ in 0..num_threads {
                let inputs_reciever = inputs_reciever.clone();
                let outputs_sender = outputs_sender.clone();
                let world_ref = world_ref.clone();
                let camera_ref = camera_ref.clone();

                let _ = thread::spawn(move || {
                    let mut rng = rand::thread_rng();
                    while let Ok(y) = inputs_reciever.recv() {
                        let y_f64 = y as f64;
                        let row = (0..image_width)
                            .map(|x| {
                                get_pixel_color(
                                    &mut rng,
                                    camera_ref.as_ref(),
                                    world_ref.as_ref(),
                                    recursion_depth,
                                    rays_per_pixel,
                                    rays_per_pixel_f64,
                                    x as f64,
                                    y_f64,
                                    x_total,
                                    y_total,
                                )
                            })
                            .collect();
                        outputs_sender.send((y, row)).expect("failed to send color");
                    }
                });
            }

            for y in 0..image_height {
                inputs_sender.send(y).expect("failed to send input");
            }

            outputs_reciever
        };

        let mut rows_remaining = image_height;
        let data: Vec<Color> = {
            let mut data: Vec<(usize, Vec<Color>)> = Vec::with_capacity(image_height);
            while let Ok((y, row)) = outputs_reciever.recv() {
                data.push((y, row));
                rows_remaining -= 1;
                eprint!("\r\x1B[0K");
                eprint!("rows remaining: {:?}", rows_remaining);
            }
            assert!(rows_remaining == 0);
            data.sort_by(|a, b| b.0.cmp(&a.0));
            data.into_iter().flat_map(|x| x.1.into_iter()).collect()
        };
        eprintln!();

        data
    };

    let image = Image {
        width: image_width,
        height: image_height,
        data,
    };

    eprintln!("Writing file...");
    image.render(&mut io::stdout()).expect("render failed");
}
