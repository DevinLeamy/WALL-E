use nalgebra::Vector3;

#[derive(Clone, Debug)]
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

    pub fn diffuse(&self) -> &Vector3<f32> {
        &self.diffuse
    }
}

impl Default for PhongMaterial {
    fn default() -> Self {
        Self {
            diffuse: Vector3::zeros(),
            specular: Vector3::zeros(),
            shininess: 0.0,
        }
    }
}
