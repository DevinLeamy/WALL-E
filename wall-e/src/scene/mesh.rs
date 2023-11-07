use std::path::PathBuf;

use super::Collidable;
use crate::{obj_loader::ObjLoader, obj_mesh::ObjMesh, prelude::*};

#[derive(Clone, Debug)]
pub struct Mesh {
    triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn from_path<P: Into<PathBuf>>(path: P) -> Self {
        let obj_mesh = ObjLoader::load(path).unwrap();
        let mut triangles = Vec::new();
        for triangle in obj_mesh.triangles() {
            let position_indices = &triangle.vertices;
            let p1 = obj_mesh.positions()[position_indices[0]];
            let p2 = obj_mesh.positions()[position_indices[1]];
            let p3 = obj_mesh.positions()[position_indices[2]];
            triangles.push(Triangle::new(p1, p2, p3))
        }

        Self { triangles }
    }
}

impl Collidable for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        todo!()
    }
}
