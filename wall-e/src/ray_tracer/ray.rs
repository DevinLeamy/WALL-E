use nalgebra::{Unit, Vector3};

pub struct Ray {
    origin: Vector3<f32>,
    direction: Unit<Vector3<f32>>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Unit<Vector3<f32>>) -> Self {
        Self { origin, direction }
    }

    pub fn from_points(src: Vector3<f32>, dest: Vector3<f32>) -> Self {
        Self::new(src, Unit::new_normalize(dest - src))
    }
}
