use crate::prelude::*;

pub struct RayTracer<B: Buffer> {
    buffer: B,
    scene: Scene,
    camera: Camera,
}

impl<B: Buffer> RayTracer<B> {
    pub fn new(buffer: B, scene: Scene, camera: Camera) -> Self {
        Self {
            buffer,
            scene,
            camera,
        }
    }
}

impl<B: Buffer> RayTracer<B> {
    pub fn run(&mut self) -> B {
        println!("RUN");
        self.buffer.clone()
    }
}
