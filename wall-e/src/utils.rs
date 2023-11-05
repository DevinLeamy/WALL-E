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
    normal
}
