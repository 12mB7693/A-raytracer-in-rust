use crate::math::Vec3;
use crate::geometric::Ray;

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3, 
    pub normal: Vec3
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
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

pub struct HitableList <T: Hitable> {
    pub hitable_list: Vec<T>
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
