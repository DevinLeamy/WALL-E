use super::{Node, Transformation};

pub struct Scene {
    root: Node,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            root: Transformation::new().into(),
        }
    }
}

impl Scene {}
