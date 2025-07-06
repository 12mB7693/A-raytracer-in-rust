
use crate::math::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        return &self.origin + &(&self.direction * t);
    }
}