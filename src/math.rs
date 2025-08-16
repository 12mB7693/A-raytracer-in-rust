use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, Clone, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }
    pub fn r(&self) -> f64 {
        self.0
    }
    pub fn g(&self) -> f64 {
        self.1
    }
    pub fn b(&self) -> f64 {
        self.2
    }
    pub fn length(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, v2: &Vec3) -> Vec3 {
        Vec3(
            self.y() * v2.z() - self.z() * v2.y(),
            self.z() * v2.x() - self.x() * v2.z(),
            self.x() * v2.y() - self.y() * v2.x(),
        )
    }
    pub fn normalize(&self) -> Vec3 {
        let mag = self.length();
        if mag == 0.0 {
            Vec3(0.0, 0.0, 0.0)
        } else {
            Vec3(self.0 / mag, self.1 / mag, self.2 / mag)
        }
    }
    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        self - n * self.dot(n) * 2.0
    }

    pub fn refract(&self, n: &Vec3, ni_over_nt: f64) -> (bool, Vec3) {
        let uv = self.normalize();
        let dt = uv.dot(n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        let mut refracted = Vec3(0.0, 0.0, 0.0);
        if discriminant > 0.0 {
            refracted = (uv - n * dt) * ni_over_nt - n * discriminant.sqrt();
            (true, refracted)
        } else {
            (false, refracted)
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add_for_vec3_obj_obj() {
        let vector1 = Vec3(1.0, 2.0, 3.0);
        let vector2 = Vec3(2.0, -1.0, 4.0);

        let result = vector1 + vector2;

        let expected = Vec3(3.0, 1.0, 7.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_for_vec3_obj_ref() {
        let vector1 = Vec3(1.0, 2.0, 3.0);
        let vector2 = Vec3(2.0, -1.0, 4.0);

        let result = vector1 + &vector2;

        let expected = Vec3(3.0, 1.0, 7.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_for_vec3_ref_obj() {
        let vector1 = Vec3(1.0, 2.0, 3.0);
        let vector2 = Vec3(2.0, -1.0, 4.0);

        let result = &vector1 + vector2;

        let expected = Vec3(3.0, 1.0, 7.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_for_vec3_ref_ref() {
        let vector1 = Vec3(1.0, 2.0, 3.0);
        let vector2 = Vec3(2.0, -1.0, 4.0);

        let result = &vector1 + &vector2;

        let expected = Vec3(3.0, 1.0, 7.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sub_for_vec3_obj_obj() {
        let vector1 = Vec3(1.0, 2.0, 3.0);
        let vector2 = Vec3(2.0, -1.0, 4.0);

        let result = vector1 - vector2;

        let expected = Vec3(-1.0, 3.0, -1.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sub_for_vec3_obj_ref() {
        let vector1 = Vec3(1.0, 2.0, 3.0);
        let vector2 = Vec3(2.0, -1.0, 4.0);

        let result = vector1 - &vector2;

        let expected = Vec3(-1.0, 3.0, -1.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sub_for_vec3_ref_obj() {
        let vector1 = Vec3(1.0, 2.0, 3.0);
        let vector2 = Vec3(2.0, -1.0, 4.0);

        let result = &vector1 - vector2;

        let expected = Vec3(-1.0, 3.0, -1.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sub_for_vec3_ref_ref() {
        let vector1 = Vec3(1.0, 2.0, 3.0);
        let vector2 = Vec3(2.0, -1.0, 4.0);
        let result = &vector1 - &vector2;
        let expected = Vec3(-1.0, 3.0, -1.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mul_for_vec3_obj() {
        let vector = Vec3(1.0, 2.0, 3.0);
        let scalar = 2.0;
        let result = vector * scalar;
        let expected = Vec3(2.0, 4.0, 6.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mul_for_vec3_ref() {
        let vector = Vec3(1.0, 2.0, 3.0);
        let scalar = 0.5;
        let result = &vector * scalar;
        let expected = Vec3(0.5, 1.0, 1.5);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_div_for_vec3_obj() {
        let vector = Vec3(1.0, 2.0, 3.0);
        let scalar = 0.5;
        let result = vector / scalar;
        let expected = Vec3(2.0, 4.0, 6.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_length() {
        let vector1 = Vec3(1.0, 2.0, 2.0);
        let vector2 = Vec3(2.0, -1.0, 2.0);
        let vector3 = Vec3(-4.0, 3.0, 0.0);
        assert_eq!(vector1.length(), 3.0);
        assert_eq!(vector2.length(), 3.0);
        assert_eq!(vector3.length(), 5.0);
    }

    #[test]
    fn test_dot() {
        let vector1 = Vec3(1.0, 2.0, 2.0);
        let vector2 = Vec3(2.0, -1.0, 2.0);
        let result = vector1.dot(&vector2);
        assert_eq!(result, 4.0);
    }
}
