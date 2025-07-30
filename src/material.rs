use crate::Ray;
use crate::Vec3;
use crate::HitRecord;
use crate::random_in_unit_sphere;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool);
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
        let reflected = r_in.direction.normalize().reflect(&rec.normal);
        let scattered = Ray {
            origin: rec.p.clone(), 
            direction: reflected + random_in_unit_sphere()*self.fuzz
        };
        let attenuation = self.albedo.clone();
        let is_scattered = scattered.direction.dot(&rec.normal) > 0.0;
        (attenuation, scattered, is_scattered)
    }
}

pub struct Lambertian {
    pub albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
        let target = &rec.p + &rec.normal + random_in_unit_sphere();
        let scattered = Ray {

            origin: rec.p.clone(), 
            direction: target - &rec.p
        };
        let attenuation = self.albedo.clone();
        (attenuation, scattered, true)
    }
}