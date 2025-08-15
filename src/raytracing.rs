use crate::camera::Camera;
use crate::geometric::Ray;
use crate::hitable::{Hitable, HitableList, Sphere};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::math::Vec3;

use rand::Rng;

pub fn generate_scene() {
    let nx = 1200;
    let ny = 800;
    let ns = 20;

    let header = format!("P3\n{} {}\n255\n", nx.to_string(), ny.to_string());
    println!("{}", header);
    let mut rng = rand::rng();

    let mut hitable_list: Vec<Sphere> = Vec::new();
    hitable_list.push(Sphere {
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Box::new(Lambertian {
            albedo: Vec3(0.5, 0.5, 0.5),
        }),
    });

    // ---- (un)comment to (not) render 100 random spheres -----
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.random();
            let x: f64 = rng.random();
            let center = Vec3(
                a as f64 + 0.9 * x,
                0.2,
                b as f64 + 0.9 * rng.random_range(0.0..1.0),
            );
            if (&center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let random_vec = Vec3(
                    rng.random_range(0.0..1.0) * rng.random_range(0.0..1.0),
                    rng.random_range(0.0..1.0) * rng.random_range(0.0..1.0),
                    rng.random_range(0.0..1.0) * rng.random_range(0.0..1.0),
                );
                if choose_mat < 0.8 {
                    hitable_list.push(Sphere {
                        center: center,
                        radius: 0.2,
                        material: Box::new(Lambertian { albedo: random_vec }),
                    });
                } else if choose_mat < 0.95 {
                    hitable_list.push(Sphere {
                        center: center,
                        radius: 0.2,
                        material: Box::new(Metal {
                            albedo: random_vec,
                            fuzz: 0.1,
                        }),
                    });
                } else {
                    hitable_list.push(Sphere {
                        center: center,
                        radius: 0.2,
                        material: Box::new(Dielectric { ref_idx: 1.5 }),
                    });
                }
            }
        }
    }
    // -------------------------------------------------------

    hitable_list.push(Sphere {
        center: Vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Dielectric { ref_idx: 1.5 }),
    });
    hitable_list.push(Sphere {
        center: Vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Lambertian {
            albedo: Vec3(0.4, 0.2, 0.1),
        }),
    });
    hitable_list.push(Sphere {
        center: Vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Metal {
            albedo: Vec3(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    });

    // test spheres for defocus blur
    // hitable_list.push(Sphere {center: Vec3(0.0, 0.0, -1.0), radius: 0.5, material: Box::new(Lambertian { albedo: Vec3(0.1, 0.2, 0.5) })});
    // hitable_list.push(Sphere {center: Vec3(0.0, -100.5, -1.0), radius: 100.0, material:Box::new( Lambertian { albedo: Vec3(0.8, 0.8, 0.0) })});
    // hitable_list.push(Sphere {center: Vec3(1.0, 0.0, -1.0), radius: 0.5, material: Box::new(Metal { albedo: Vec3(0.8, 0.6, 0.2), fuzz: 0.2})});
    // hitable_list.push(Sphere {center: Vec3(-1.0, 0.0, -1.0), radius: 0.5, material: Box::new(Dielectric { ref_idx: 1.5 })});
    // hitable_list.push(Sphere {center: Vec3(-1.0, 0.0, -1.0), radius: -0.45, material: Box::new(Dielectric { ref_idx: 1.5 })});

    // let r = f64::cos(std::f64::consts::PI/4.0);
    // hitable_list.push(Sphere {center: Vec3(-r, 0.0, -1.0), radius: r, material: Box::new(Lambertian { albedo: Vec3(0.0, 0.0, 1.0) })});
    // hitable_list.push(Sphere {center: Vec3( r, 0.0, -1.0), radius: r, material: Box::new(Lambertian { albedo: Vec3(1.0, 0.0, 0.0) })});

    let world = HitableList {
        hitable_list: hitable_list,
    };
    let lookfrom = Vec3(13.0, 2.0, 3.0);
    let lookat = Vec3(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3(0.0, 1.0, 0.0),
        20.0,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
    );

    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let mut col = Vec3(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let x: f64 = rng.random();
                let y: f64 = rng.random();

                let u = (i as f64 + x) / nx as f64;
                let v = (j as f64 + y) / ny as f64;

                let r = camera.get_ray(u, v);
                col = col + trace_ray(&r, &world, 0);

                // Debugging
                // if col.r() < 1.0 && col.g() < 1.0
                // {
                //     let unit_direction = unit_vector(&r.direction);
                //     let t = (unit_direction.y() + 1.0)*0.5;
                //     println!("Warning: i = {}, j = {}, t = {}, length = {}", i, j, t, r.direction.length());

                // }
                // else
                // {
                //     let unit_direction = unit_vector(&r.direction);
                //     let t = (unit_direction.y() + 1.0)*0.5;
                //     println!("i = {}, j = {}, t = {}, length = {}, direction={:?}", i, j, t, r.direction.length(), r.direction);
                // }
            }
            col = col * (1.0 / ns as f64);
            col = Vec3(col.r().sqrt(), col.g().sqrt(), col.b().sqrt()); // gamma correction
            let ir = (255.99 * col.r()) as u8;
            let ig = (255.99 * col.g()) as u8;
            let ib = (255.99 * col.b()) as u8;
            println!("{}", format!("{} {} {}\n", ir, ig, ib));
        }
    }
}

fn trace_ray<T: Hitable>(r: &Ray, world: &T, depth: i32) -> Vec3 {
    let (hit, rec, material) = world.hit(r, 0.001, f64::INFINITY);
    if hit {
        if material.is_none() {
            return Vec3(0.0, 0.0, 0.0);
        }
        let material = material.unwrap();
        let (attenuation, scattered, is_scattered) = material.scatter(r, &rec);
        if depth < 50 && is_scattered {
            let new_color = trace_ray(&scattered, world, depth + 1);
            return Vec3(
                new_color.x() * attenuation.x(),
                new_color.y() * attenuation.y(),
                new_color.z() * attenuation.z(),
            );
        } else {
            return Vec3(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction = r.direction.normalize();
        let t = (unit_direction.y() + 1.0) * 0.5;
        return Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t;
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p;
    let mut rng = rand::rng();
    loop {
        let x: f64 = rng.random();
        let y: f64 = rng.random();
        let z: f64 = rng.random();
        p = Vec3(x, y, z) * 2.0 - Vec3(1.0, 1.0, 1.0);
        if p.length() * p.length() < 1.0 {
            break;
        }
    }
    p
}
