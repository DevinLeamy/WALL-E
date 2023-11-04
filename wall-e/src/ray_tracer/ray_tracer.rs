use nalgebra::Unit;

use crate::prelude::*;

const DISTANCE_TO_SCREEN: f32 = 1.0;

pub struct RayTracer<B: Buffer<Value = Vector3<f32>>> {
    buffer: B,
    scene: FlatScene,
    camera: Camera,
}

impl<B: Buffer<Value = Vector3<f32>>> RayTracer<B> {
    pub fn new(buffer: B, scene: Scene, camera: Camera) -> Self {
        Self {
            buffer,
            scene: scene.into(),
            camera,
        }
    }
}

impl<B: Buffer<Value = Vector3<f32>>> RayTracer<B> {
    pub fn run(&mut self) -> B {
        for x in 0..self.buffer.width() {
            for y in 0..self.buffer.height() {
                let pixel_pos = self.compute_pixel_position(x, y);
                let ray = Ray::from_points(self.camera.origin(), pixel_pos);

                // Cast the primary ray into the scene to intersect with the scene's geometry.
                let Some(intersection) = self.cast_primary_ray(ray) else {
                    continue;
                };

                // Compute the light at the point of intersection by casting secondary, shadow,
                // rays directly towards the lights in the scene.
                let mut total_light = Vector3::<f32>::zeros();
                for light in self.scene.lights() {
                    total_light += self.light_contribution_at_intersection(light, &intersection);
                }
                // println!("({:?}) TOTAL LIGHT: {:?}", self.scene.lights().len(), total_light);

                self.buffer.set(x, y, total_light);
            }
        }
        self.buffer.clone()
    }
}

impl<B: Buffer<Value = Vector3<f32>>> RayTracer<B> {
    fn cast_primary_ray(&mut self, ray: Ray) -> Option<Intersection> {
        let intersections = self.ray_geometry_intersections(&ray);
        if intersections.is_empty() {
            return None;
        }
        let mut nearest: &Intersection = &intersections[0];
        for i in 1..intersections.len() {
            if intersections[i].t() < nearest.t() {
                nearest = &intersections[i];
            }
        }


        Some(nearest.clone())
    }

    fn light_contribution_at_intersection(
        &self,
        light: &Light,
        intersection: &Intersection,
    ) -> Vector3<f32> {
        let light_ray = Unit::new_normalize(light.transform().translation() - intersection.point());
        let viewer_ray = -intersection.ray().direction();
        let normal = intersection.normal();

        let light = super::shader::phong_illumination(
            normal,
            light_ray,
            viewer_ray,
            intersection.material(),
        );

        light
    }

    fn ray_geometry_intersections(&mut self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = Vec::<Intersection>::new();
        for geometry in self.scene.geometry() {
            if let Some(intersection) = geometry.intersect(&ray) {
                intersections.push(intersection)
            }
        }

        intersections
    }

    fn compute_pixel_position(&mut self, x: u32, y: u32) -> Vector3<f32> {
        let d = DISTANCE_TO_SCREEN;

        // Convert the pixel coordinates to NDC coordinates.
        //
        // We add 0.5 to x and y to get the center of the pixel.
        let ndc_x = (x as f32 + 0.5) / self.buffer.width() as f32;
        let ndc_y = (y as f32 + 0.5) / self.buffer.height() as f32;

        // Convert the NDC coordinates to Screen coordinates.
        let screen_x = (ndc_x - 0.5) * 2.0;
        let screen_y = (ndc_y - 0.5) * 2.0 * -1.0;

        // Correct for the aspect ratio.
        let screen_x = screen_x * self.buffer.aspect();

        // Convert the Screen coordinates to Camera coordinates.
        let tan_half_fov = (self.camera.fov().to_radians() / 2.0).tan();
        let camera_x = screen_x * tan_half_fov * d;
        let camera_y = screen_y * tan_half_fov * d;

        let pixel_camera_pos = Vector3::new(camera_x, camera_y, d);

        // Convert the Camera coordinates to World coordinates.
        let pixel_world_pos = self.camera.camera_to_world_mat() * pixel_camera_pos.push(1.0);
        
        pixel_world_pos.xyz()
    }
}
