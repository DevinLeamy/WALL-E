use nalgebra::Vector3;

use crate::{
    prelude::{Intersection, Ray},
    scene::Collidable,
};

#[derive(Clone, Debug)]
pub struct Triangle {
    p1: Vector3<f32>,
    p2: Vector3<f32>,
    p3: Vector3<f32>,
}

impl Triangle {
    /// Create a new triangle with three vertices.
    pub fn new(p1: Vector3<f32>, p2: Vector3<f32>, p3: Vector3<f32>) -> Self {
        Self { p1, p2, p3 }
    }
}

impl Collidable for Triangle {
    fn intersect(&self, _ray: &Ray) -> Option<Intersection> {
        todo!()
        // Some(Intersection::new(ray.clone(), None, t, normal))
    }
}
