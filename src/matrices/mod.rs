#[derive(Debug, PartialEq)]
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
}

impl Default for Matrix3x3 {
    fn default() -> Self {
        Matrix3x3 { data: [0.; 9] }
    }
}

#[cfg(test)]
mod tests;
