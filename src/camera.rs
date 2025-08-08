use crate::Ray;
use crate::Vec3;
use rand::Rng;

//#[derive(Default)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f64,
    u: Vec3,
    v: Vec3,
}

// impl Default for Camera {
//     fn default() -> Camera {
//         Camera {
//             origin: Vec3(0.0, 0.0, 0.0),
//             lower_left_corner: Vec3(-2.0, -1.0, -1.0),
//             horizontal: Vec3(4.0, 0.0, 0.0),
//             vertical: Vec3(0.0, 2.0, 0.0),
//             lens_radius: 0.0
//         }
//     }
// }

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = f64::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let w = (&lookfrom - &lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);
        Camera {
            origin: lookfrom.clone(),
            lower_left_corner: &lookfrom
                - &u * half_width * focus_dist
                - &v * half_height * focus_dist
                - &w * focus_dist,
            horizontal: &u * 2.0 * half_width * focus_dist,
            vertical: &v * 2.0 * half_height * focus_dist,
            lens_radius: aperture / 2.0,
            u: u,
            v: v,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        //let direction = &(&(&self.lower_left_corner + &(&self.horizontal*u)) + &(&self.vertical*v)) - &self.origin;

        // let horizontal = &self.horizontal * u;
        // let vertical = &self.vertical * v;
        // let lower_left_plus_horiz = &self.lower_left_corner + &horizontal;
        // let direction = &(&lower_left_plus_horiz + &vertical )- &self.origin;

        //let lower_left_plus_horiz = self.lower_left_corner + &horizontal;
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = &self.u * rd.x() + &self.v * rd.y();
        let direction = &self.lower_left_corner + &self.horizontal * u + &self.vertical * v
            - &self.origin
            - &offset;

        let origin = self.origin.clone();
        Ray {
            origin: origin + offset,
            direction: direction,
        }
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut p;
    let mut rng = rand::rng();
    loop {
        let x: f64 = rng.random();
        let y: f64 = rng.random();
        p = Vec3(x, y, 0.0) * 2.0 - Vec3(1.0, 1.0, 0.0);
        if p.length() * p.length() < 1.0 {
            break;
        }
    }
    p
}
