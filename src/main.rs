mod math;
mod hitable;
mod geometric;
mod camera;

use math::Vec3;
use hitable::{HitableList, Sphere, HitRecord, Hitable};
use geometric::Ray;
use camera::Camera;
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
    hitable_list.push(Sphere {center: Vec3(0.0, 0.0, -1.0), radius: 0.5});
    hitable_list.push(Sphere {center: Vec3(0.0, -100.5, -1.0), radius: 100.0});
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
                //let secret_number = rand::thread_rng().gen_range(1..=100);
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
                col = col + color(&r, &world); // Vec3(u, v, 0.2);

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


    

// fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
//     let diff = &ray.origin - center;
//     let oc = &diff;
//     let a = ray.direction.dot(&ray.direction);
//     let b = 2.0 * oc.dot(&ray.direction);
//     let c = oc.dot(oc) - radius * radius;
//     let discriminant = b*b - a*c*4.0;
//     if discriminant < 0.0 {
//         -1.0
//     }
//     else {
//         (-b - discriminant.sqrt())/ (2.0 * a)
//     }
// }


fn color<T: Hitable>(r: &geometric::Ray, world: &T) -> Vec3 {
    //let unit_direction = unit_vector(&r.direction);
    // let t =  hit_sphere(&Vec3(0.0, 0.0, -1.0), 0.5, r);
    let mut rec = HitRecord{t: 0.0, p: Vec3(0.0,0.0,0.0), normal: Vec3(0.0,0.0,0.0)};
    if world.hit(r, 0.001, f64::INFINITY, &mut rec)
    {
        let target = &rec.p + &rec.normal + random_in_unit_sphere();
        let ray = Ray {
            origin: rec.p.clone(), 
            direction: target - &rec.p
        };
        return color(&ray, world)*0.5;
        //return Vec3(rec.normal.x() + 1.0, rec.normal.y() + 1.0, rec.normal.z() + 1.0)*0.5;
    }
    else {
        let unit_direction = r.direction.normalize();
        let t = (unit_direction.y() + 1.0)*0.5;
        return Vec3(1.0, 1.0, 1.0)*(1.0 - t) + Vec3(0.5, 0.7, 1.0)*t;
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3 (0.0, 0.0, 0.0);
    let mut rng = rand::rng();
    loop {
        let x : f64 = rng.random();
        let y : f64 = rng.random();
        let z : f64 = rng.random();
        p = Vec3(x, y, z)*2.0 - Vec3(1.0, 1.0, 1.0);
        if (p.length()*p.length() < 1.0) {
            break;
        }
    }
    p
}


