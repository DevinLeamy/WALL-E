use super::{Node, Transformation};

#[derive(Clone)]
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

impl Scene {
    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn root_mut(&mut self) -> &mut Node {
        &mut self.root
    }
}

impl Scene {}
