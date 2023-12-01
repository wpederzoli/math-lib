use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use crate::prelude::Vector3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Quaternion {
    scalar: f32,
    vector: Vector3,
}

impl Default for Quaternion {
    fn default() -> Self {
        Quaternion {
            scalar: 0.,
            vector: Vector3::default(),
        }
    }
}

impl Quaternion {
    pub fn new(scalar: f32, vector: Vector3) -> Self {
        Quaternion { scalar, vector }
    }

    pub fn norm(self) -> f32 {
        let n = (self.scalar * self.scalar)
            + (self.vector.x * self.vector.x)
            + (self.vector.y * self.vector.y)
            + (self.vector.z * self.vector.z);
        f32::trunc(n.sqrt() * 100.) / 100.
    }

    pub fn normalize(&mut self) {
        let n = self.norm();
        if n != 0. {
            self.scalar *= ((1. / n) * 100.).round() / 100.;
            self.vector *= ((1. / n) * 100.).round() / 100.;
        }
    }
}

impl Add for Quaternion {
    type Output = Quaternion;

    fn add(self, rhs: Self) -> Self::Output {
        Quaternion {
            scalar: self.scalar + rhs.scalar,
            vector: self.vector + rhs.vector,
        }
    }
}

impl AddAssign for Quaternion {
    fn add_assign(&mut self, rhs: Self) {
        self.scalar += rhs.scalar;
        self.vector += rhs.vector;
    }
}

impl Sub for Quaternion {
    type Output = Quaternion;

    fn sub(self, rhs: Self) -> Self::Output {
        Quaternion {
            scalar: self.scalar - rhs.scalar,
            vector: self.vector - rhs.vector,
        }
    }
}

impl SubAssign for Quaternion {
    fn sub_assign(&mut self, rhs: Self) {
        self.vector -= rhs.vector;
        self.scalar -= rhs.scalar;
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: Self) -> Self::Output {
        Quaternion {
            scalar: self.scalar * rhs.scalar - self.vector.dot(rhs.vector),
            vector: rhs.vector * self.scalar
                + self.vector * rhs.scalar
                + self.vector.cross(rhs.vector),
        }
    }
}

impl MulAssign for Quaternion {
    fn mul_assign(&mut self, rhs: Self) {
        let q = *self * rhs;

        self.scalar = q.scalar;
        self.vector = q.vector;
    }
}

impl Mul<f32> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: f32) -> Self::Output {
        Quaternion {
            scalar: self.scalar * rhs,
            vector: self.vector * rhs,
        }
    }
}

impl MulAssign<f32> for Quaternion {
    fn mul_assign(&mut self, rhs: f32) {
        self.scalar *= rhs;
        self.vector *= rhs;
    }
}

#[cfg(test)]
mod tests;
