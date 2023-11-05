use std::mem::swap;

use nalgebra::{SimdBool, Unit, Vector3};

use crate::{
    prelude::{Intersection, Ray},
    utils::opposite_sign,
};

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
            let t1 = (-b - disc.sqrt()) / (2.0 * a);
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

impl Cube {
    fn is_inside(&self, point: &Vector3<f32>) -> bool {
        let half_size = self.size / 2.0;
        let min = Vector3::new(-half_size, -half_size, -half_size);
        let max = Vector3::new(half_size, half_size, half_size);

        point.ge(&min).all() && point.le(&max).all()
    }
}

impl Collidable for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let half_size = self.size / 2.0;
        let (origin, direction) = (ray.origin(), ray.direction());
        if self.is_inside(&origin) {
            return None;
        }

        let mut tmin = (-half_size - origin.x) / direction.x;
        let mut tmax = (half_size - origin.x) / direction.x;
        if tmin > tmax {
            swap(&mut tmin, &mut tmax);
        }

        let mut tmin_y = (-half_size - origin.y) / direction.y;
        let mut tmax_y = (half_size - origin.y) / direction.y;
        if tmin_y > tmax_y {
            swap(&mut tmin_y, &mut tmax_y);
        }

        if tmin > tmax_y || tmax < tmin_y {
            return None;
        }

        tmin = f32::max(tmin, tmin_y);
        tmax = f32::min(tmax, tmax_y);

        let mut tmin_z = (-half_size - origin.z) / direction.z;
        let mut tmax_z = (half_size - origin.z) / direction.z;
        if tmin_z > tmax_z {
            swap(&mut tmin_z, &mut tmax_z);
        }

        if tmin > tmax_z || tmax < tmin_z {
            return None;
        }

        tmin = f32::max(tmin, tmin_z);
        tmax = f32::min(tmax, tmax_z);

        if tmin <= 0.0 {
            return None;
        }

        let mut normal = Vector3::<f32>::zeros();
        // Flip the sign based on the direction of the incoming ray.
        if tmin > tmin_y && tmin > tmin_z {
            normal.x = opposite_sign(direction.x);
        } else if tmin_y > tmin_z {
            normal.y = opposite_sign(direction.y);
        } else {
            normal.z = opposite_sign(direction.z);
        }

        Some(Intersection::new(
            ray.clone(),
            None,
            tmin,
            Unit::new_normalize(normal),
        ))
    }
}
