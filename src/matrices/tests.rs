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

#[test]
fn addition() {
    let expected = Matrix3x3::new(1., 1., 1., 2., 2., 2., 3., 3., 3.);
    let m3_a = Matrix3x3::new(0., 0., 0., 1., 1., 1., 2., 2., 2.);
    let m3_b = Matrix3x3::new(1., 1., 1., 1., 1., 1., 1., 1., 1.);

    assert_eq!(m3_a + m3_b, expected);
}

#[test]
fn add_assign() {
    let expected = Matrix3x3::new(2., 2., 2., 3., 3., 3., 4., 4., 4.);
    let mut mat3 = Matrix3x3::new(1., 1., 1., 2., 2., 2., 3., 3., 3.);
    mat3 += Matrix3x3::new(1., 1., 1., 1., 1., 1., 1., 1., 1.);

    assert_eq!(mat3, expected);
}

#[test]
fn scalar_mul() {
    let expected = Matrix3x3::new(2., 2., 2., 4., 4., 4., 6., 6., 6.);
    let mat3 = Matrix3x3::new(1., 1., 1., 2., 2., 2., 3., 3., 3.) * 2.;

    assert_eq!(mat3, expected);
}

#[test]
fn scalar_mul_assign() {
    let expected = Matrix3x3::new(2., 2., 2., 4., 4., 4., 6., 6., 6.);
    let mut mat3 = Matrix3x3::new(1., 1., 1., 2., 2., 2., 3., 3., 3.);

    mat3 *= 2.;

    assert_eq!(mat3, expected);
}

#[test]
fn multiplication() {
    let expected = Matrix3x3::new(12., 12., 12., 18., 18., 18., 24., 24., 24.);
    let mat3_a = Matrix3x3::new(1., 1., 1., 2., 2., 2., 3., 3., 3.);
    let mat3_b = Matrix3x3::new(2., 2., 2., 3., 3., 3., 4., 4., 4.);
    let result = mat3_a * mat3_b;

    assert_eq!(result, expected);
}

#[test]
fn multiplication_assign() {
    let expected = Matrix3x3::new(12., 12., 12., 18., 18., 18., 24., 24., 24.);
    let mut mat3 = Matrix3x3::new(1., 1., 1., 2., 2., 2., 3., 3., 3.);

    mat3 *= Matrix3x3::new(2., 2., 2., 3., 3., 3., 4., 4., 4.);

    assert_eq!(mat3, expected);
}

#[test]
fn identity_matrix() {
    let expected = Matrix3x3::new(1., 0., 0., 0., 1., 0., 0., 0., 1.);
    let mat3 = Matrix3x3::new(1., 2., 3., 4., 5., 6., 7., 8., 9.);

    assert_eq!(Matrix3x3::identity(), expected);
    assert_eq!(mat3 * Matrix3x3::identity(), mat3);
}

#[test]
fn inverse_not_exist() {
    let mat = Matrix3x3::new(1., 2., 3., 4., 5., 6., 7., 8., 9.);

    assert_eq!(mat.inverse(), mat);
}

#[test]
fn inverse() {
    let mat = Matrix3x3::new(3., 2., 5., 2., -1., 4., -1., 2., 1.);
    let expected = Matrix3x3::new(
        f32::trunc((3. / 8.) * 100.) / 100.,
        f32::trunc((-1. / 3.) * 100.) / 100.,
        f32::trunc((-13. / 24.) * 100.) / 100.,
        f32::trunc((1. / 4.) * 100.) / 100.,
        f32::trunc((-1. / 3.) * 100.) / 100.,
        f32::trunc((1. / 12.) * 100.) / 100.,
        f32::trunc((-1. / 8.) * 100.) / 100.,
        f32::trunc((1. / 3.) * 100.) / 100.,
        f32::trunc((7. / 24.) * 100.) / 100.,
    );

    assert_eq!(expected, mat.inverse());
}
