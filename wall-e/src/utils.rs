use nalgebra::{Unit, Vector3};

use crate::prelude::Transform;

pub fn opposite_sign(n: f32) -> f32 {
    if n.is_sign_negative() {
        1.0
    } else {
        -1.0
    }
}

pub fn transform_normal(normal: Unit<Vector3<f32>>, transform: &Transform) -> Unit<Vector3<f32>> {
    Unit::new_normalize(transform.as_mat3_inverse() * normal.into_inner())
}

pub fn vector_mul(a: &Vector3<f32>, b: &Vector3<f32>) -> Vector3<f32> {
    Vector3::new(a.x * b.x, a.y * b.y, a.z * b.z)
}

pub fn vector_reciprocal(mut v: Vector3<f32>) -> Vector3<f32> {
    if v.x == 0.0 {
        v.x = 1.0;
    }
    if v.y == 0.0 {
        v.y = 1.0;
    }
    if v.z == 0.0 {
        v.z = 1.0;
    }

    Vector3::new(1.0 / v.x, 1.0 / v.y, 1.0 / v.z)
}
