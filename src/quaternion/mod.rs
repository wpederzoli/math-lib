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
    fn new(scalar: f32, vector: Vector3) -> Self {
        Quaternion { scalar, vector }
    }
}

#[cfg(test)]
mod tests;
