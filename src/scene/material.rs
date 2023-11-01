use nalgebra::Vector3;

pub struct PhongMaterial {
    diffuse: Vector3<f32>,
    specular: Vector3<f32>,
    shininess: f32,
}

impl PhongMaterial {
    pub fn new(diffuse: Vector3<f32>, specular: Vector3<f32>, shininess: f32) -> Self {
        Self {
            diffuse,
            specular,
            shininess,
        }
    }
}
