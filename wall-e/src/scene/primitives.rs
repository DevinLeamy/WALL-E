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
        // Note: we assume the sphere is centered at the origin.
        let a = 1.0; // ray.direction^2
        let b = 2.0 * ray.origin().dot(&ray.direction().into_inner());
        let c = ray.origin().dot(&ray.origin()) - self.radius * self.radius;

        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return None;
        } 

        let t = if disc == 0.0 {
            -b / (2.0 * a)
        } else {
            let t0 = (-b + disc.sqrt()) / (2.0 * a);
            let t1 = (b + disc.sqrt()) / (2.0 * a);
            f32::max(t0, t1)
        };

        if t <= 0.0 {
            return None;
        } 
        
        Some(Intersection::new(ray.clone(), t))
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
