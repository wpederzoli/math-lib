use std::fmt::Display;
use std::ops;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Vector3 {
    fn default() -> Self {
        Vector3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: f32) -> Self::Output {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_default() {
        let vec = Vector3::default();
        let expected = Vector3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        assert_eq!(vec, expected);
    }

    #[test]
    fn create_from_values() {
        let vec = Vector3::new(1., 2., 3.);
        let expected = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };

        assert_eq!(vec, expected);
    }

    #[test]
    fn addition() {
        let expected = Vector3 {
            x: 2.,
            y: 4.,
            z: 6.,
        };
        let vec_a = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let vec_b = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let vec = vec_a + vec_b;
        assert_eq!(vec, expected);
    }

    #[test]
    fn add_assign() {
        let expected = Vector3 {
            x: 2.,
            y: 4.,
            z: 6.,
        };
        let mut vec = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };

        vec += Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };

        assert_eq!(vec, expected);
    }

    #[test]
    fn substract() {
        let expected = Vector3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let vec = Vector3 {
            x: 1.,
            y: 1.,
            z: 1.,
        } - Vector3 {
            x: 1.,
            y: 1.,
            z: 1.,
        };

        assert_eq!(vec, expected);
    }

    #[test]
    fn substract_assign() {
        let expected = Vector3 {
            x: -1.,
            y: -1.,
            z: -1.,
        };

        let mut vec = Vector3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        vec -= Vector3 {
            x: 1.,
            y: 1.,
            z: 1.,
        };

        assert_eq!(vec, expected);
    }

    #[test]
    fn scalar_mult() {
        let expected = Vector3 {
            x: 4.,
            y: 6.,
            z: 8.,
        };
        let vec = Vector3 {
            x: 2.,
            y: 3.,
            z: 4.,
        } * 2.;
        assert_eq!(vec, expected);
    }

    #[test]
    fn scalar_mult_assign() {
        let expected = Vector3 {
            x: 4.,
            y: 6.,
            z: 8.,
        };
        let mut vec = Vector3 {
            x: 2.,
            y: 3.,
            z: 4.,
        };

        vec *= 2.;

        assert_eq!(vec, expected);
    }

    #[test]
    fn scalar_div() {
        let expected = Vector3 {
            x: 2.,
            y: 3.,
            z: 4.,
        };
        let vec = Vector3 {
            x: 4.,
            y: 6.,
            z: 8.,
        } / 2.;

        assert_eq!(vec, expected);
    }

    #[test]
    fn scalar_div_assign() {
        let expected = Vector3 {
            x: 2.,
            y: 3.,
            z: 4.,
        };
        let mut vec = Vector3 {
            x: 4.,
            y: 6.,
            z: 8.,
        };
        vec /= 2.;

        assert_eq!(vec, expected);
    }
}
