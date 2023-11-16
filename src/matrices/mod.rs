use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign},
};

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
            self.data[3],
            self.data[6],
            self.data[1],
            self.data[4],
            self.data[7],
            self.data[2],
            self.data[5],
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
            self.data[0] * rhs,
            self.data[3] * rhs,
            self.data[6] * rhs,
            self.data[1] * rhs,
            self.data[4] * rhs,
            self.data[7] * rhs,
            self.data[2] * rhs,
            self.data[5] * rhs,
            self.data[8] * rhs,
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

#[cfg(test)]
mod tests;
