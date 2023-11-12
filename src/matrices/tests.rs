use super::*;

#[test]
fn create_matrix() {
    let expected = Matrix3x3 {
        data: [1., 2., 3., 1., 2., 3., 1., 2., 3.],
    };
    let mat3 = Matrix3x3::new(1., 1., 1., 2., 2., 2., 3., 3., 3.);

    assert_eq!(mat3, expected);
}

#[test]
fn default_matrix() {
    let expected = Matrix3x3 { data: [0.; 9] };
    let mat3 = Matrix3x3::default();

    assert_eq!(mat3, expected);
}
