use nalgebra::{Vector2, Vector3};

#[derive(Clone, Debug)]
pub struct ObjVertex {
    pub position: Vector3<f32>,
    pub normal: Option<Vector3<f32>>,
    pub uv: Option<Vector2<f32>>,
}

#[derive(Clone, Debug)]
pub struct ObjTriangle {
    pub vertices: Vec<usize>,
    pub normals: Vec<usize>,
    pub uvs: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct ObjMesh {
    vertices: Vec<ObjVertex>,
    triangles: Vec<ObjTriangle>,
}

impl ObjMesh {
    pub fn new(vertices: Vec<ObjVertex>, triangles: Vec<ObjTriangle>) -> Self {
        Self {
            vertices,
            triangles,
        }
    }

    pub fn vertices(&self) -> &Vec<ObjVertex> {
        &self.vertices
    }

    pub fn triangles(&self) -> &Vec<ObjTriangle> {
        &self.triangles
    }
}
