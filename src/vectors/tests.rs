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

#[test]
fn dot_product() {
    let expected = 10.;
    let vec_a = Vector3 {
        x: 1.,
        y: 1.,
        z: 1.,
    };
    let vec_b = Vector3 {
        x: 2.,
        y: 2.,
        z: 6.,
    };
    assert_eq!(vec_a * vec_b, expected);
    assert_eq!(vec_a.dot(vec_b), expected);
}

#[test]
fn cross_product() {
    let expected = Vector3 {
        x: 3.,
        y: 3.,
        z: -3.,
    };
    let vec_a = Vector3 {
        x: 1.,
        y: 2.,
        z: 3.,
    };
    let vec_b = Vector3 {
        x: 2.,
        y: 1.,
        z: 3.,
    };

    assert_eq!(vec_a % vec_b, expected);
    assert_eq!(vec_a.cross(vec_b), expected);
}

#[test]
fn cross_assign() {
    let expected = Vector3 {
        x: 3.,
        y: 3.,
        z: -3.,
    };
    let mut vec = Vector3 {
        x: 1.,
        y: 2.,
        z: 3.,
    };
    let vec_b = Vector3 {
        x: 2.,
        y: 1.,
        z: 3.,
    };

    vec %= vec_b;

    assert_eq!(vec, expected);
}

#[test]
fn magnitude() {
    let expected = (14.0f32).sqrt();
    let vec = Vector3::new(1., 2., 3.);

    assert_eq!(vec.magnitude(), expected);
}

#[test]
fn test_normalize_nonzero() {
    let vector = Vector3 {
        x: 3.0,
        y: 4.0,
        z: 5.0,
    };
    let normalized = vector.normalize();
    assert_eq!(normalized.magnitude(), 1.0);
}

#[test]
fn test_normalize_zero_magnitude() {
    let zero_vector = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let normalized = zero_vector.normalize();

    // For a zero vector, normalization should result in the same zero vector
    assert_eq!(normalized, zero_vector);
}
