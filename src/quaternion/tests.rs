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
