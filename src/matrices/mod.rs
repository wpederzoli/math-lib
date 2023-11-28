use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign},
};

use crate::prelude::Vector3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrix3x3 {
    pub data: [f32; 9],
}

impl Matrix3x3 {
    pub fn new(
        m0: f32,
        m1: f32,
        m2: f32,
        m3: f32,
        m4: f32,
        m5: f32,
        m6: f32,
        m7: f32,
        m8: f32,
    ) -> Self {
        Matrix3x3 {
            data: [m0, m3, m6, m1, m4, m7, m2, m5, m8],
        }
    }

    pub fn identity() -> Matrix3x3 {
        Matrix3x3::new(1., 0., 0., 0., 1., 0., 0., 0., 1.)
    }

    pub fn inverse(self) -> Matrix3x3 {
        let d = self.determinant();
        if d == 0. {
            return self;
        }

        let cof = self.co_factor();

        cof * (1. / d)
    }

    pub fn transpose(self) -> Matrix3x3 {
        let m0 = self.data[0];
        let m1 = self.data[1];
        let m2 = self.data[2];
        let m3 = self.data[3];
        let m4 = self.data[4];
        let m5 = self.data[5];
        let m6 = self.data[6];
        let m7 = self.data[7];
        let m8 = self.data[8];

        Matrix3x3::new(m0, m1, m2, m3, m4, m5, m6, m7, m8)
    }

    fn determinant(self) -> f32 {
        self.data[0] * self.data[4] * self.data[8]
            + (self.data[3] * self.data[7] * self.data[2])
            + (self.data[6] * self.data[1] * self.data[5])
            - (self.data[6] * self.data[4] * self.data[2])
            - (self.data[0] * self.data[7] * self.data[5])
            - (self.data[3] * self.data[1] * self.data[8])
    }

    fn co_factor(self) -> Matrix3x3 {
        let c0 = self.data[4] * self.data[8] - (self.data[7] * self.data[5]);
        let c1 = self.data[1] * self.data[8] - (self.data[7] * self.data[2]);
        let c2 = self.data[1] * self.data[5] - (self.data[4] * self.data[2]);
        let c3 = self.data[3] * self.data[8] - (self.data[6] * self.data[5]);
        let c4 = self.data[0] * self.data[8] - (self.data[6] * self.data[2]);
        let c5 = self.data[0] * self.data[5] - (self.data[3] * self.data[2]);
        let c6 = self.data[3] * self.data[7] - (self.data[6] * self.data[4]);
        let c7 = self.data[0] * self.data[7] - (self.data[6] * self.data[1]);
        let c8 = self.data[0] * self.data[4] - (self.data[3] * self.data[1]);

        Matrix3x3::new(c0, -(c3), c6, -(c1), c4, -(c7), c2, -(c5), c8)
    }
}

impl Default for Matrix3x3 {
    fn default() -> Self {
        Matrix3x3 { data: [0.; 9] }
    }
}

impl Display for Matrix3x3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n| {}, {}, {} |\n| {}, {}, {} |\n| {}, {}, {} |",
            self.data[0],
            self.data[1],
            self.data[2],
            self.data[3],
            self.data[4],
            self.data[5],
            self.data[6],
            self.data[7],
            self.data[8]
        )
    }
}

impl Add for Matrix3x3 {
    type Output = Matrix3x3;
    fn add(self, rhs: Self) -> Self::Output {
        Matrix3x3::new(
            self.data[0] + rhs.data[0],
            self.data[3] + rhs.data[3],
            self.data[6] + rhs.data[6],
            self.data[1] + rhs.data[1],
            self.data[4] + rhs.data[4],
            self.data[7] + rhs.data[7],
            self.data[2] + rhs.data[2],
            self.data[5] + rhs.data[5],
            self.data[8] + rhs.data[8],
        )
    }
}

impl AddAssign for Matrix3x3 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..self.data.len() {
            self.data[i] += rhs.data[i];
        }
    }
}

impl Mul<f32> for Matrix3x3 {
    type Output = Matrix3x3;
    fn mul(self, rhs: f32) -> Self::Output {
        Matrix3x3::new(
            f32::trunc((self.data[0] * rhs) * 100.) / 100.,
            f32::trunc((self.data[3] * rhs) * 100.) / 100.,
            f32::trunc((self.data[6] * rhs) * 100.) / 100.,
            f32::trunc((self.data[1] * rhs) * 100.) / 100.,
            f32::trunc((self.data[4] * rhs) * 100.) / 100.,
            f32::trunc((self.data[7] * rhs) * 100.) / 100.,
            f32::trunc((self.data[2] * rhs) * 100.) / 100.,
            f32::trunc((self.data[5] * rhs) * 100.) / 100.,
            f32::trunc((self.data[8] * rhs) * 100.) / 100.,
        )
    }
}

impl MulAssign<f32> for Matrix3x3 {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..self.data.len() {
            self.data[i] *= rhs;
        }
    }
}

impl Mul<Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;
    fn mul(self, rhs: Matrix3x3) -> Self::Output {
        let m0 =
            self.data[0] * rhs.data[0] + self.data[1] * rhs.data[3] + self.data[2] * rhs.data[6];
        let m1 =
            self.data[3] * rhs.data[0] + self.data[4] * rhs.data[3] + self.data[5] * rhs.data[6];
        let m2 =
            self.data[6] * rhs.data[0] + self.data[7] * rhs.data[3] + self.data[8] * rhs.data[6];
        let m3 =
            self.data[0] * rhs.data[1] + self.data[1] * rhs.data[4] + self.data[2] * rhs.data[7];
        let m4 =
            self.data[3] * rhs.data[1] + self.data[4] * rhs.data[4] + self.data[5] * rhs.data[7];
        let m5 =
            self.data[6] * rhs.data[1] + self.data[7] * rhs.data[4] + self.data[8] * rhs.data[7];
        let m6 =
            self.data[0] * rhs.data[2] + self.data[1] * rhs.data[5] + self.data[2] * rhs.data[8];
        let m7 =
            self.data[3] * rhs.data[2] + self.data[4] * rhs.data[5] + self.data[5] * rhs.data[8];
        let m8 =
            self.data[6] * rhs.data[2] + self.data[7] * rhs.data[5] + self.data[8] * rhs.data[8];

        Matrix3x3::new(m0, m1, m2, m3, m4, m5, m6, m7, m8)
    }
}

impl MulAssign<Matrix3x3> for Matrix3x3 {
    fn mul_assign(&mut self, rhs: Matrix3x3) {
        let m0 =
            self.data[0] * rhs.data[0] + self.data[1] * rhs.data[3] + self.data[2] * rhs.data[6];
        let m1 =
            self.data[3] * rhs.data[0] + self.data[4] * rhs.data[3] + self.data[5] * rhs.data[6];
        let m2 =
            self.data[6] * rhs.data[0] + self.data[7] * rhs.data[3] + self.data[8] * rhs.data[6];
        let m3 =
            self.data[0] * rhs.data[1] + self.data[1] * rhs.data[4] + self.data[2] * rhs.data[7];
        let m4 =
            self.data[3] * rhs.data[1] + self.data[4] * rhs.data[4] + self.data[5] * rhs.data[7];
        let m5 =
            self.data[6] * rhs.data[1] + self.data[7] * rhs.data[4] + self.data[8] * rhs.data[7];
        let m6 =
            self.data[0] * rhs.data[2] + self.data[1] * rhs.data[5] + self.data[2] * rhs.data[8];
        let m7 =
            self.data[3] * rhs.data[2] + self.data[4] * rhs.data[5] + self.data[5] * rhs.data[8];
        let m8 =
            self.data[6] * rhs.data[2] + self.data[7] * rhs.data[5] + self.data[8] * rhs.data[8];

        self.data[0] = m0;
        self.data[3] = m1;
        self.data[6] = m2;
        self.data[1] = m3;
        self.data[4] = m4;
        self.data[7] = m5;
        self.data[2] = m6;
        self.data[5] = m7;
        self.data[8] = m8;
    }
}

impl Mul<Vector3> for Matrix3x3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        let x = self.data[0] * rhs.x + self.data[3] * rhs.y + self.data[6] * rhs.z;
        let y = self.data[1] * rhs.x + self.data[4] * rhs.y + self.data[7] * rhs.z;
        let z = self.data[2] * rhs.x + self.data[5] * rhs.y + self.data[8] * rhs.z;

        Vector3::new(x, y, z)
    }
}

#[cfg(test)]
mod tests;
