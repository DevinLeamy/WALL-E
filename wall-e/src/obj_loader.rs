use std::path::PathBuf;

use nalgebra::{Vector2, Vector3};
use obj::{IndexTuple, Obj};

use crate::obj_mesh::{ObjMesh, ObjTriangle, ObjVertex};

/// Loads wavefront .obj files.
pub struct ObjLoader;

impl ObjLoader {
    pub fn load<P: Into<PathBuf>>(path: P) -> Result<ObjMesh, String> {
        let path = path.into();
        let obj: Obj = Obj::load(path).map_err(|e| e.to_string())?;
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();

        let vertex_count = obj.data.position.len();

        for i in 0..vertex_count {
            let position = obj.data.position[i];
            let mut normal: Option<Vector3<f32>> = None;
            if let Some(v) = obj.data.texture.get(i) {
                normal = Some(Vector3::new(v[0], v[1], v[2]));
            }
            let mut uv: Option<Vector2<f32>> = None;
            if let Some(v) = obj.data.texture.get(i) {
                uv = Some(Vector2::new(v[0], v[1]));
            }

            vertices.push(ObjVertex {
                position: Vector3::new(position[0], position[1], position[2]),
                normal,
                uv,
            });
        }

        for shape in obj.data.objects.iter().flat_map(|o| &o.groups[0].polys) {
            assert!(shape.0.len() == 3);
            let mut vertices = Vec::new();
            let mut normals = Vec::new();
            let mut uvs = Vec::new();

            for IndexTuple(v_position, v_normal, v_uv) in &shape.0 {
                vertices.push(*v_position);
                if let Some(normal) = v_normal {
                    normals.push(*normal);
                }
                if let Some(uv) = v_uv {
                    uvs.push(*uv);
                }
            }

            triangles.push(ObjTriangle {
                vertices,
                normals,
                uvs,
            });
        }

        Ok(ObjMesh::new(vertices, triangles))
    }
}
