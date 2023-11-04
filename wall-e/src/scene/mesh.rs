use super::Collidable;
use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Mesh {}

impl Collidable for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        todo!()
    }
}
