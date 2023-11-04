use nalgebra::{Unit, Vector3};

#[derive(Clone, Debug)]
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

    pub fn origin(&self) -> Vector3<f32> {
        self.origin
    }

    pub fn direction(&self) -> Unit<Vector3<f32>> {
        self.direction
    }

    pub fn point(&self, t: f32) -> Vector3<f32> {
        self.origin + self.direction.into_inner() * t
    }
}
