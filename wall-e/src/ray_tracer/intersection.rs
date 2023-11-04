use crate::prelude::PhongMaterial;

use super::Ray;

#[derive(Clone)]
pub struct Intersection {
    t: f32,
    ray: Ray,
    material: PhongMaterial,
}

impl Intersection {
    pub fn new(ray: Ray, material: Option<PhongMaterial>, t: f32) -> Self {
        Self {
            t,
            material: material.unwrap_or_default(),
            ray,
        }
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn material(&self) -> &PhongMaterial {
        &self.material
    }

    pub fn set_material(&mut self, material: PhongMaterial) {
        self.material = material;
    }
}
