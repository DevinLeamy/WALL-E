mod output;
mod ray_tracer;
mod scene;

pub mod prelude {
    pub use crate::output::*;
    pub use crate::ray_tracer::*;
    pub use crate::scene::*;
    pub use nalgebra::Vector3;
}
