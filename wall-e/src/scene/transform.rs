use nalgebra::{Matrix3, Matrix4, Rotation3, Vector3};

#[derive(Clone, Debug)]
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

    pub fn rotate_x(&mut self, _rad: f32) {
        todo!()
    }

    pub fn rotate_y(&mut self, _rad: f32) {
        todo!()
    }

    pub fn rotate_z(&mut self, _rad: f32) {
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

    /// Apply the given transform to self and return the new transform.
    pub fn transform(&self, transform: &Transform) -> Transform {
        let new_scale = Vector3::new(
            self.scale.x * transform.scale.x,
            self.scale.y * transform.scale.y,
            self.scale.z * transform.scale.z,
        );
        let current_rotation = rotation_to_rot3(self.rotation);
        let other_rotation = rotation_to_rot3(transform.rotation);
        let new_rot3 = current_rotation * other_rotation;
        let rotated_other_translation = current_rotation * transform.translation;
        let new_translation = self.translation + rotated_other_translation;
        let new_rotation = rot3_to_rotation(new_rot3);

        Transform {
            rotation: new_rotation,
            translation: new_translation,
            scale: new_scale,
        }
    }

    pub fn as_mat4(&self) -> Matrix4<f32> {
        // Order: T - R - S
        let mut m = Matrix4::<f32>::identity();
        m.prepend_nonuniform_scaling_mut(&self.scale);
        m.prepend_translation_mut(&self.translation);

        m
    }

    pub fn as_mat3(&self) -> Matrix3<f32> {
        // Order: R - S
        let m4 = self.as_mat4();
        let m3_view = m4.fixed_slice::<3, 3>(0, 0);
        let m = Matrix3::from_iterator(m3_view.iter().cloned());

        m
    }
}

fn rotation_to_rot3(rotation: Vector3<f32>) -> Rotation3<f32> {
    Rotation3::from_euler_angles(rotation.x, rotation.y, rotation.y)
}

fn rot3_to_rotation(r: Rotation3<f32>) -> Vector3<f32> {
    let (x, y, z) = r.euler_angles();
    Vector3::new(x, y, z)
}
