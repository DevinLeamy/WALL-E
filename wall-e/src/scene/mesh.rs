use std::path::PathBuf;

use super::Collidable;
use crate::{obj_loader::ObjLoader, obj_mesh::ObjMesh, prelude::*};

#[derive(Clone, Debug)]
pub struct Mesh {
    obj: ObjMesh,
}

impl Mesh {
    pub fn from_path<P: Into<PathBuf>>(path: P) -> Self {
        let obj_mesh = ObjLoader::load(path).unwrap();
        Self { obj: obj_mesh }
    }
}

impl Collidable for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        todo!()
    }
}
