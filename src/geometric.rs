
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_at_zero_is_origin() {
        let ray = Ray {
            origin: Vec3(1.0, 2.0, 3.0),
            direction: Vec3(0.0, 1.0, 0.0)
        };

        assert_eq!(ray.point_at_parameter(0.0), ray.origin);
    }

    #[test]
    fn point_at_non_zero_is_not_origin() {
        let ray = Ray {
            origin: Vec3(1.0, 2.0, 3.0),
            direction: Vec3(0.0, 1.0, 0.0)
        };

        assert_ne!(ray.point_at_parameter(1.0), ray.origin);
    }

    #[test]
    fn point_at_non_zero() {
        let ray = Ray {
            origin: Vec3(1.0, 2.0, 3.0),
            direction: Vec3(0.0, 1.0, 0.0)
        };

        let expected = Vec3 (1.0, 3.0, 3.0);


        assert_eq!(ray.point_at_parameter(1.0), expected);
    }
}