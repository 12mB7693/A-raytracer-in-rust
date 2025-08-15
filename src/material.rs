use crate::geometric::Ray;
use crate::hitable::HitRecord;
use crate::math::Vec3;
use crate::raytracing::random_in_unit_sphere;
use rand::Rng;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool);
}

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    fn schlick(&self, cosine: f64) -> f64 {
        let mut r0 = (1.0 - self.ref_idx) / (1.0 + self.ref_idx);
        r0 = r0 * r0;

        let f = f64::powf(1.0 - cosine, 5.0);
        r0 + (1.0 - r0) * f
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
        let reflected = r_in.direction.reflect(&rec.normal);
        let attenuation = Vec3(1.0, 1.0, 1.0);
        let outward_normal;
        let ni_over_nt;
        let reflect_prob;
        let cosine;
        let scattered;
        if r_in.direction.dot(&rec.normal) > 0.0 {
            outward_normal = &rec.normal * (-1.0);
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r_in.direction.dot(&rec.normal) / r_in.direction.length();
        } else {
            outward_normal = rec.normal.clone();
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = r_in.direction.dot(&rec.normal) * (-1.0) / r_in.direction.length();
        }

        let (is_scattered, refracted) = r_in.direction.refract(&outward_normal, ni_over_nt);
        if is_scattered {
            reflect_prob = self.schlick(cosine);
        } else {
            reflect_prob = 1.0;
        }
        let mut rng = rand::rng();
        let x: f64 = rng.random();
        if x < reflect_prob {
            scattered = Ray {
                origin: rec.p.clone(),
                direction: reflected,
            };
        } else {
            scattered = Ray {
                origin: rec.p.clone(),
                direction: refracted,
            };
        }

        return (attenuation, scattered, true);
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
        let reflected = r_in.direction.normalize().reflect(&rec.normal);
        let scattered = Ray {
            origin: rec.p.clone(),
            direction: reflected + random_in_unit_sphere() * self.fuzz,
        };
        let attenuation = self.albedo.clone();
        let is_scattered = scattered.direction.dot(&rec.normal) > 0.0;
        (attenuation, scattered, is_scattered)
    }
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
        let target = &rec.p + &rec.normal + random_in_unit_sphere();
        let scattered = Ray {
            origin: rec.p.clone(),
            direction: target - &rec.p,
        };
        let attenuation = self.albedo.clone();
        (attenuation, scattered, true)
    }
}
