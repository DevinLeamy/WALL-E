use nalgebra::Vector3;

#[derive(Clone)]
pub struct Transform {
    rotation: Vector3<f32>,
    translation: Vector3<f32>,
    scale: Vector3<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            rotation: Vector3::zeros(),
            translation: Vector3::zeros(),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

impl Transform {
    pub fn rotation(&self) -> Vector3<f32> {
        self.rotation
    }

    pub fn set_rotation(&mut self, v: Vector3<f32>) {
        self.rotation = v;
    }

    pub fn rotate_x(&mut self, rad: f32) {
        todo!()
    }

    pub fn rotate_y(&mut self, rad: f32) {
        todo!()
    }

    pub fn rotate_z(&mut self, rad: f32) {
        todo!()
    }

    pub fn scale(&self) -> Vector3<f32> {
        self.scale
    }

    pub fn set_scale(&mut self, v: Vector3<f32>) {
        self.scale = v;
    }

    pub fn scale_nonuniform(&mut self, v: Vector3<f32>) {
        let mut scale = self.scale();
        scale.x *= v.x;
        scale.y *= v.y;
        scale.z *= v.z;
        self.set_scale(scale);
    }

    pub fn translation(&self) -> Vector3<f32> {
        self.translation
    }

    pub fn set_translation(&mut self, v: Vector3<f32>) {
        self.translation = v;
    }

    pub fn translate(&mut self, v: Vector3<f32>) {
        self.set_translation(self.translation() + v);
    }
}
