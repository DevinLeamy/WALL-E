use nalgebra::{Unit, Vector3};

use crate::prelude::PhongMaterial;

const DEFAULT_AMBIENT_INTENSITY: Vector3<f32> = Vector3::<f32>::new(0.1, 0.1, 0.1);

pub fn phong_illumination(
    // Surface normal at the point of intersection.
    normal: Unit<Vector3<f32>>,
    // Vector from the point of intersection to the light.
    light_ray: Unit<Vector3<f32>>,
    // Vector from the point of intersection to the viewer.
    viewer_ray: Unit<Vector3<f32>>,
    // Surface properties.
    material: &PhongMaterial,
) -> Vector3<f32> {
    let light_dot_normal = f32::max(0.0, normal.dot(&light_ray));
    let diffuse = light_dot_normal * material.diffuse();
    let mut specular = Vector3::<f32>::zeros();

    if light_dot_normal > 0.0 {
        let viewer_light = viewer_ray.into_inner() + light_ray.into_inner();
        let viewer_light_abs = viewer_light.clone().abs();

        let h = Unit::new_normalize(Vector3::new(
            viewer_light.x / viewer_light_abs.x,
            viewer_light.y / viewer_light_abs.y,
            viewer_light.z / viewer_light_abs.z,
        ));

        specular = f32::max(0.0, normal.dot(&h).powf(material.shininess())) * material.specular();
    }
    // println!("DIFF: {:?}", diffuse);
    // println!("SPEC: {:?}", specular);

    DEFAULT_AMBIENT_INTENSITY + diffuse + specular
}
