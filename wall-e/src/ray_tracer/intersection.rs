use super::Ray;

#[derive(Clone)]
pub struct Intersection {
    t: f32,
    ray: Ray,
}

impl Intersection {
    pub fn new(ray: Ray, t: f32) -> Self {
        Self {
            t,
            ray
        }
    }

    pub fn t(&self) -> f32 {
        self.t
    }
}
