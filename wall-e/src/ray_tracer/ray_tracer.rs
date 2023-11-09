use std::time::Instant;

use nalgebra::Unit;
use rand::{Rng, rngs::ThreadRng};

use crate::prelude::*;

use super::background::PatternedBackground;

const DISTANCE_TO_SCREEN: f32 = 1.0;
const SAMPLES_PER_PIXEL: u32 = 9;

pub struct RayTracer<B: Buffer<Value = Vector3<f32>>> {
    buffer: B,
    scene: FlatScene,
    camera: Camera,
    background: PatternedBackground,
    /// Number of samples per pixel.
    samples: u32,
    /// Random number generator.
    random: ThreadRng
}

impl<B: Buffer<Value = Vector3<f32>>> RayTracer<B> {
    pub fn new(buffer: B, scene: Scene, camera: Camera) -> Self {
        Self {
            background: PatternedBackground::new(
                "./wall-e/assets/wall-e-5.png",
                buffer.width(),
                buffer.height(),
                (buffer.width() / 3).max(30).min(60),
            ),
            buffer,
            scene: scene.into(),
            camera,
            samples: SAMPLES_PER_PIXEL,
            random: rand::thread_rng()
        }
    }
}

impl<B: Buffer<Value = Vector3<f32>>> RayTracer<B> {
    pub fn run(&mut self) -> B {
        let start_time = Instant::now();
        let total_pixels = self.buffer.width() * self.buffer.height();
        let mut traced = 0;

        for x in 0..self.buffer.width() {
            for y in 0..self.buffer.height() {
                traced += 1;
                println!(
                    "⏳ Completed {:.1}%",
                    traced as f32 / total_pixels as f32 * 100.0
                );
                self.color_pixel(x, y);
            }
        }
        let duration = Instant::now().duration_since(start_time);
        println!("📷 Render time {:.2}s", duration.as_secs_f32());
        self.buffer.clone()
    }
}

impl<B: Buffer<Value = Vector3<f32>>> RayTracer<B> {
    fn color_pixel(&mut self, x: u32, y: u32) {
        let mut total_light = Vector3::zeros();
        for _ in 0..self.samples {
            let xx = x as f32 + self.random.gen_range(0.0..=1.0) - 0.5;
            let yy = y as f32 + self.random.gen_range(0.0..=1.0) - 0.5;
            let pixel_position = self.compute_pixel_position(xx, yy);

            let ray = Ray::from_points(self.camera.origin(), pixel_position);

            // Cast the primary ray into the scene to intersect with the scene's geometry.
            let Some(intersection) = self.cast_primary_ray(ray) else {
                total_light += self.background.color(x, y);
                continue;
            };

            // Compute the light at the point of intersection by casting secondary, shadow,
            // rays directly towards the lights in the scene.
            let mut sample_light = Vector3::<f32>::zeros();
            for light in self.scene.lights() {
                sample_light += self.light_contribution_at_intersection(light, &intersection);
            }

            total_light += sample_light;
        }

        let average_light = 1.0 / (self.samples as f32) * total_light;
        self.buffer.set(x, y, average_light);
    }

    fn cast_primary_ray(&mut self, ray: Ray) -> Option<Intersection> {
        let mut nearest: Option<Intersection> = None;

        for geometry in self.scene.geometry() {
            if let Some(intersection) = geometry.intersect(&ray) {
                if nearest.is_none() || nearest.as_ref().unwrap().t() > intersection.t() {
                    nearest = Some(intersection);
                }
            }
        }

        nearest
    }

    fn light_contribution_at_intersection(
        &self,
        light: &Light,
        intersection: &Intersection,
    ) -> Vector3<f32> {
        let light_ray = Unit::new_normalize(light.transform().translation() - intersection.point());
        let ray = Ray::from_points(
            intersection.point() + light_ray.into_inner() * 0.1,
            light.transform().translation(),
        );
        let light_t = ray.t(&light.transform().translation());

        let mut in_shadow = false;
        for geometry in self.scene.geometry() {
            if let Some(intersection) = geometry.intersect(&ray) {
                if intersection.t() < light_t {
                    in_shadow = true;
                    break;
                }
            }
        }

        super::shader::phong_illumination(
            self.camera.origin(),
            intersection.point(),
            light.transform().translation(),
            intersection.normal(),
            intersection.material(),
            self.scene.ambient(),
            light.colour(),
            light.attenuation(),
            in_shadow,
        )
    }

    fn compute_pixel_position(&mut self, x: f32, y: f32) -> Vector3<f32> {
        let d = DISTANCE_TO_SCREEN;

        // Convert the pixel coordinates to NDC coordinates.
        //
        // We add 0.5 to x and y to get the center of the pixel.
        let ndc_x = (x + 0.5) / self.buffer.width() as f32;
        let ndc_y = (y + 0.5) / self.buffer.height() as f32;

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
