use crate::geometric::Ray;
use crate::material::Material;
use crate::math::Vec3;

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

impl Default for HitRecord {
    fn default() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 0.0),
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord, Option<&dyn Material>);
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord, Option<&dyn Material>) {
        let oc = &r.origin - &self.center;
        let a = &r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0001 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let rec = HitRecord {
                    t: temp,
                    p: r.point_at_parameter(temp),
                    normal: (&r.point_at_parameter(temp) - &self.center) / self.radius,
                };
                return (true, rec, Some(&*self.material));
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let rec = HitRecord {
                    t: temp,
                    p: r.point_at_parameter(temp),
                    normal: (&r.point_at_parameter(temp) - &self.center) / self.radius,
                };
                return (true, rec, Some(&*self.material));
            }
        }
        return (
            false,
            HitRecord {
                ..Default::default()
            },
            None,
        );
    }
}

pub struct HitableList<T: Hitable> {
    pub hitable_list: Vec<T>,
}

impl Hitable for HitableList<Sphere> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord, Option<&dyn Material>) {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut rec = HitRecord {
            ..Default::default()
        };
        let mut material = None;
        for hitable in &self.hitable_list {
            let (hit, temp_rec, mat) = hitable.hit(r, t_min, closest_so_far);
            if hit {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec;
                material = mat
            }
        }
        return (hit_anything, rec, material);
    }
}
