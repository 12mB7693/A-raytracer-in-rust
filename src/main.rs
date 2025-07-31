mod math;
mod hitable;
mod geometric;
mod camera;
mod material;

use math::Vec3;
use hitable::{HitableList, Sphere, HitRecord, Hitable};
use geometric::Ray;
use camera::Camera;
use material::{Metal, Lambertian, Dielectric};
use rand::Rng;

fn main() {
    let nx = 200*3;
    let ny = 100*3;
    let ns = 20;
    //let first_line = "P3\n" + nx.to_string();
    //println!(first_line);
    let header = format!("P3\n{} {}\n255\n", nx.to_string(), ny.to_string());
    println!("{}", header);
    
    let mut hitable_list: Vec<Sphere> = Vec::new();
    hitable_list.push(Sphere {center: Vec3(0.0, 0.0, -1.0), radius: 0.5, material: Box::new(Lambertian { albedo: Vec3(0.1, 0.2, 0.5) })});
    hitable_list.push(Sphere {center: Vec3(0.0, -100.5, -1.0), radius: 100.0, material:Box::new( Lambertian { albedo: Vec3(0.8, 0.8, 0.0) })});
    hitable_list.push(Sphere {center: Vec3(1.0, 0.0, -1.0), radius: 0.5, material: Box::new(Metal { albedo: Vec3(0.8, 0.6, 0.2), fuzz: 0.2})});
    hitable_list.push(Sphere {center: Vec3(-1.0, 0.0, -1.0), radius: 0.5, material: Box::new(Dielectric { ref_idx: 1.5 })});
    hitable_list.push(Sphere {center: Vec3(-1.0, 0.0, -1.0), radius: -0.45, material: Box::new(Dielectric { ref_idx: 1.5 })});
    let world = HitableList { hitable_list: hitable_list };
    let camera = Camera { ..Default::default() };
    let mut rng = rand::rng();
    for j in (0..ny-1).rev()
    {
        for i in 0..nx
        {
            let mut col = Vec3(0.0, 0.0, 0.0);
            for _ in 0..ns
            {
                let x : f64 = rng.random();
                let y : f64 = rng.random();
                // let x = 0.0;
                // let y = 0.0;
                let u = (i as f64 + x)/nx as f64;
                let v = (j as f64 + y)/ny as f64;

                // let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
                // let horizontal = Vec3(4.0, 0.0, 0.0);
                // let vertical = Vec3(0.0, 2.0, 0.0);
                // let origin = Vec3(0.0, 0.0, 0.0);

                // let r = Ray{
                //     origin: origin, 
                //     direction: lower_left_corner + horizontal*u + vertical*v
                // };
                let r = camera.get_ray(u, v);
                col = col + color(&r, &world, 0); // Vec3(u, v, 0.2);

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
            col = col * (1.0 / ns as f64) ;
            col = Vec3(col.r().sqrt(), col.g().sqrt(), col.b().sqrt()); // gamma correction
            let ir = (255.99*col.r()) as u8;
            let ig = (255.99*col.g()) as u8;
            let ib = (255.99*col.b()) as u8;
            println!("{}", format!("{} {} {}\n", ir, ig, ib));
        }
    }  
}


fn color<T: Hitable>(r: &geometric::Ray, world: &T, depth: i32) -> Vec3 {
    let (hit, rec, material) = world.hit(r, 0.001, f64::INFINITY);
    if hit
    {
        if material.is_none() {
            return Vec3(0.0, 0.0, 0.0);
        }
        let material = material.unwrap();
        let (attenuation, scattered, is_scattered) = material.scatter(r, &rec);
        if depth < 50 && is_scattered {
            let new_color = color(&scattered, world, depth+1);
            return Vec3(new_color.x()*attenuation.x(), new_color.y()*attenuation.y(), new_color.z()*attenuation.z());
        }
        else {
            return Vec3(0.0, 0.0, 0.0);
        }
    }
    else {
        let unit_direction = r.direction.normalize();
        let t = (unit_direction.y() + 1.0)*0.5;
        return Vec3(1.0, 1.0, 1.0)*(1.0 - t) + Vec3(0.5, 0.7, 1.0)*t;
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p; 
    let mut rng = rand::rng();
    loop {
        let x : f64 = rng.random();
        let y : f64 = rng.random();
        let z : f64 = rng.random();
        p = Vec3(x, y, z)*2.0 - Vec3(1.0, 1.0, 1.0);
        if p.length()*p.length() < 1.0 {
            break;
        }
    }
    p
}


