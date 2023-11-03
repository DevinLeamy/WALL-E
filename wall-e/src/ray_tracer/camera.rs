use nalgebra::Vector3;

#[derive(Clone)]
pub struct Camera {
    position: Vector3<f32>,
    fov: f32,
    target: Vector3<f32>,
    up: Vector3<f32>,
}

impl Camera {
    pub fn new(position: Vector3<f32>, fov: f32, target: Vector3<f32>, up: Vector3<f32>) -> Self {
        Self {
            position,
            fov,
            target,
            up,
        }
    }
}

impl Camera {
    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
    }

    pub fn set_target(&mut self, target: Vector3<f32>) {
        self.target = target;
    }

    pub fn set_up(&mut self, up: Vector3<f32>) {
        self.up = up;
    }
}
