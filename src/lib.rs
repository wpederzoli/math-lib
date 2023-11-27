mod matrices;
mod quaternion;
mod vectors;

pub mod prelude {
    pub use crate::matrices::Matrix3x3;
    pub use crate::quaternion::Quaternion;
    pub use crate::vectors::Vector3;
}
