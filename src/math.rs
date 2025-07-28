use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Debug)]
#[derive(PartialEq)]
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
        (self.0*self.0 + self.1*self.1 + self.2*self.2).sqrt()
    }
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.0*other.0 + self.1*other.1 + self.2*other.2
    }
    pub fn normalize(&self) -> Vec3 {
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

// pub fn unit_vector(vector: &Vec3) -> Vec3 {
//     //let unit_vector = Vec3 {..*vector};
//     let len = vector.length();
//     Vec3 (vector.0 / len, vector.1 / len, vector.2 / len)
// }