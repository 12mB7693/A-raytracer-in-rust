use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

fn main() {
    let nx = 200*3;
    let ny = 100*3;
    //let first_line = "P3\n" + nx.to_string();
    //println!(first_line);
    let header = format!("P3\n{} {}\n255\n", nx.to_string(), ny.to_string());
    println!("{}", header);
    
    for j in (0..ny-1).rev()
    {
        for i in 0..nx
        {
            let u = i as f64/nx as f64;
            let v = j as f64/ny as f64;


            let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
            let horizontal = Vec3(4.0, 0.0, 0.0);
            let vertical = Vec3(0.0, 2.0, 0.0);
            let origin = Vec3(0.0, 0.0, 0.0);

            let r = Ray{
                origin: origin, 
                direction: lower_left_corner + horizontal*u + vertical*v
            };
            let col = color(&r); // Vec3(u, v, 0.2);

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

             
            let ir = (255.99*col.r()) as u8;
            let ig = (255.99*col.g()) as u8;
            let ib = (255.99*col.b()) as u8;
            println!("{}", format!("{} {} {}\n", ir, ig, ib));
        }
    }  
}

#[derive(Debug)]
struct Vec3(f64, f64, f64);

impl Vec3 {
     fn x(&self) -> f64 {
         self.0
     }
    fn y(&self) -> f64 {
        self.1
    }
    fn z(&self) -> f64 {
        self.2
    }
    fn r(&self) -> f64 {
        self.0
    }
    fn g(&self) -> f64 {
        self.1
    }
    fn b(&self) -> f64 {
        self.2
    }
    fn length(&self) -> f64 {
        (self.0*self.0 + self.1*self.1 + self.2*self.2).sqrt()
    }
    fn dot(&self, other: &Vec3) -> f64 {
        self.0*other.0 + self.1*other.1 + self.2*other.2
    }
    fn normalize(&self) -> Vec3 {
        let mag = self.length();
        if mag == 0.0 {
            Vec3(0.0, 0.0, 0.0) // or handle differently if you prefer
        } else {
            Vec3(self.0 / mag, self.1 / mag, self.2 / mag)
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2
        )
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 (self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self (
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self (self.0*rhs, self.1*rhs, self.2*rhs)
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 (self.0*rhs, self.1*rhs, self.2*rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self (self.0/rhs, self.1/rhs, self.2/rhs)
    }
}

struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    fn point_at_parameter(&self, t: f64) -> Vec3 {
        return &self.origin + &(&self.direction * t);
    }
}
    
fn unit_vector(vector: &Vec3) -> Vec3 {
    //let unit_vector = Vec3 {..*vector};
    let len = vector.length();
    Vec3 (vector.0 / len, vector.1 / len, vector.2 / len)
}

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let diff = &ray.origin - center;
    let oc = &diff;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b*b - a*c*4.0;
    if discriminant < 0.0 {
        -1.0
    }
    else {
        (-b - discriminant.sqrt())/ (2.0 * a)
    }
}

fn color(r: &Ray) -> Vec3 {
    //let unit_direction = unit_vector(&r.direction);
    let t =  hit_sphere(&Vec3(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = unit_vector(&(r.point_at_parameter(t) - Vec3(0.0, 0.0, -1.0)));
        return Vec3(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)*0.5;
    }
        
    let unit_direction = &r.direction.normalize();
    let t = (unit_direction.y() + 1.0)*0.5;
    return Vec3(1.0, 1.0, 1.0)*(1.0 - t) + Vec3(0.5, 0.7, 1.0)*t;
}

struct HitRecord {
    t: f64,
    p: Vec3, 
    normal: Vec3
}

trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

struct Sphere {
    center: Vec3,
    radius: f64
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = &r.origin - &self.center;
        let a = &r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let temp = (-b - (b*b-a*c).sqrt())/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (&rec.p - &self.center) / self.radius;
                return true;
            }
            let temp = (-b + (b*b-a*c).sqrt())/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (&rec.p - &self.center) / self.radius;
                return true;
            }
        }
        return false;
    }
}

struct HitableList <T: Hitable> {
    hitable_list: Vec<T>
}

impl Hitable for HitableList<Sphere> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
       
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for hitable in &self.hitable_list {
            let mut temp_rec = HitRecord{t: 0.0, p: Vec3(0.0,0.0,0.0), normal: Vec3(0.0,0.0,0.0)};
            if hitable.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        return hit_anything;
    }
}

