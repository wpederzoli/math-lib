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
