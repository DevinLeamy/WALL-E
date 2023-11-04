use nalgebra::Unit;

use crate::prelude::{Intersection, Ray};

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
            // TODO: We actually want to smallest value larger than 0.0, because we don't
            // want to register "internal" collisions.
            let t0 = (-b + disc.sqrt()) / (2.0 * a);
            let t1 = (b + disc.sqrt()) / (2.0 * a);
            f32::min(t0, t1)
        };

        if t <= 0.0 {
            return None;
        }

        let point = ray.point(t);
        // The normal at this point is simple the normalized point, because the
        // sphere is centered at the origin.
        let normal = Unit::new_normalize(point);

        Some(Intersection::new(ray.clone(), None, t, normal))
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
