use std::ops::{Add, AddAssign, Sub, SubAssign};

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

#[cfg(test)]
mod tests;
