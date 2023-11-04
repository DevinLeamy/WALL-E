use crate::prelude::{Ray, Intersection};

use super::Collidable;

#[derive(Clone, Debug)]
pub struct Sphere {
    radius: f32,
}

impl Sphere {
    /// Create a sphere with a given radius.
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl Collidable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct Cube {
    size: f32,
}

impl Cube {
    /// Create a cube with a given side length.
    pub fn new(size: f32) -> Self {
        Self { size }
    }
}

impl Collidable for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        todo!()
    }
}
