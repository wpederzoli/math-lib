use std::fmt::Display;
use std::ops;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    pub fn dot(self, vec: Vector3) -> f32 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    pub fn cross(self, vec: Vector3) -> Vector3 {
        Vector3 {
            x: (self.y * vec.z - vec.y * self.z),
            y: -(self.x * vec.z - self.z * vec.x),
            z: (self.x * vec.y - self.y * vec.x),
        }
    }

    pub fn magnitude(self) -> f32 {
        (f32::powf(self.x, 2.) + f32::powf(self.y, 2.) + f32::powf(self.z, 2.)).sqrt()
    }

    pub fn normalize(self) -> Vector3 {
        let mag = self.magnitude();
        if mag != 0. {
            self / self.magnitude()
        } else {
            self
        }
    }
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

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
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

impl ops::Mul<Vector3> for Vector3 {
    type Output = f32;
    fn mul(self, rhs: Vector3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
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

impl ops::Rem<Vector3> for Vector3 {
    type Output = Vector3;
    fn rem(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: (self.y * rhs.z - rhs.y * self.z),
            y: -(self.x * rhs.z - self.z * rhs.x),
            z: (self.x * rhs.y - self.y * rhs.x),
        }
    }
}

impl ops::RemAssign<Vector3> for Vector3 {
    fn rem_assign(&mut self, rhs: Vector3) {
        let x = self.y * rhs.z - self.z * rhs.y;
        let y = -(self.x * rhs.z - self.z * rhs.x);
        let z = self.x * rhs.y - self.y - rhs.x;
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

#[cfg(test)]
mod tests;
