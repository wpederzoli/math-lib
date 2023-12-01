use super::*;

#[test]
fn create_default() {
    let expected = Quaternion {
        scalar: 0.,
        vector: Vector3::default(),
    };

    assert_eq!(Quaternion::default(), expected);
}

#[test]
fn create_new() {
    let expected = Quaternion {
        scalar: 2.,
        vector: Vector3::new(1., 2., 3.),
    };

    assert_eq!(Quaternion::new(2., Vector3::new(1., 2., 3.)), expected);
}

#[test]
fn assign_quaternion() {
    let expected = Quaternion::new(2., Vector3::new(1., 2., 3.));
    let mut quat = Quaternion::default();

    quat = expected;

    assert_eq!(quat, expected);
}

#[test]
fn sum_quaternions() {
    let expected = Quaternion::new(2., Vector3::new(4., 5., 6.));

    let quat_a = Quaternion::new(1., Vector3::new(2., 3., 4.));
    let quat_b = Quaternion::new(1., Vector3::new(2., 2., 2.));

    assert_eq!(quat_a + quat_b, expected);
}

#[test]
fn sum_assign_quaternion() {
    let expected = Quaternion::new(2., Vector3::new(4., 5., 6.));
    let mut quat = Quaternion::new(1., Vector3::new(2., 3., 4.));
    let quat_b = Quaternion::new(1., Vector3::new(2., 2., 2.));

    quat += quat_b;

    assert_eq!(quat, expected);
}

#[test]
fn substract_quaternions() {
    let expected = Quaternion::new(2., Vector3::new(4., 5., 6.));

    let quat_a = Quaternion::new(3., Vector3::new(6., 7., 8.));
    let quat_b = Quaternion::new(1., Vector3::new(2., 2., 2.));

    assert_eq!(quat_a - quat_b, expected);
}

#[test]
fn substract_assign_quaternion() {
    let expected = Quaternion::new(1., Vector3::new(4., 5., 6.));
    let mut quat = Quaternion::new(2., Vector3::new(6., 7., 8.));
    let quat_b = Quaternion::new(1., Vector3::new(2., 2., 2.));

    quat -= quat_b;

    assert_eq!(quat, expected);
}

#[test]
fn multiply_quaternion() {
    let expected = Quaternion::new(-8., Vector3::new(1., 14., 3.));
    let a = Quaternion::new(1., Vector3::new(1., 2., 3.));
    let b = Quaternion::new(2., Vector3::new(3., 2., 1.));

    assert_eq!(a * b, expected);
}

#[test]
fn mult_assign_quaternion() {
    let expected = Quaternion::new(-8., Vector3::new(1., 14., 3.));
    let mut a = Quaternion::new(1., Vector3::new(1., 2., 3.));
    let b = Quaternion::new(2., Vector3::new(3., 2., 1.));

    a *= b;

    assert_eq!(a, expected);
}

#[test]
fn multiply_scalar_quaternion() {
    let expected = Quaternion::new(2., Vector3::new(2., 4., 6.));
    let quat = Quaternion::new(1., Vector3::new(1., 2., 3.));

    assert_eq!(quat * 2., expected);
}

#[test]
fn multiply_scalar_assign_quat() {
    let expected = Quaternion::new(2., Vector3::new(2., 4., 6.));
    let mut quat = Quaternion::new(1., Vector3::new(1., 2., 3.));

    quat *= 2.;

    assert_eq!(quat, expected);
}

#[test]
fn norm_quaternion() {
    let q = Quaternion::new(1., Vector3::new(1., 2., 3.));
    let expected = (q.scalar * q.scalar)
        + (q.vector.x * q.vector.x)
        + (q.vector.y * q.vector.y)
        + (q.vector.z * q.vector.z);

    assert_eq!(q.norm(), f32::trunc(expected.sqrt() * 100.) / 100.);
}

#[test]
fn unit_normalize_quat() {
    let expected = Quaternion::new(0.26, Vector3::new(0.26, 0.52, 0.78));
    let mut q = Quaternion::new(1., Vector3::new(1., 2., 3.));
    q.normalize();

    assert_eq!(q, expected);
}
