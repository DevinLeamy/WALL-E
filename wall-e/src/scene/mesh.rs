use std::path::PathBuf;

use super::Collidable;
use crate::{obj_loader::ObjLoader, prelude::*};

#[derive(Clone, Debug)]
pub struct Mesh {}

impl Mesh {
    pub fn from_path<P: Into<PathBuf>>(path: P) -> Self {
        let obj_mesh = ObjLoader::load(path);

        todo!()
    }
}

impl Collidable for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        todo!()
    }
}
