use nalgebra::Vector3;

use super::Transform;

pub enum Node {
    Light(Light),
    Geometry(Geometry),
    Transformation(Transformation),
}

impl Node {
    pub fn add_child(&mut self, child: Node) {
        self.children_mut().push(child);
    }

    pub fn rotate(&mut self, v: Vector3<f32>) {
        todo!()
    }

    pub fn translate(&mut self, v: Vector3<f32>) {
        self.transform_mut().translate(v)
    }

    pub fn scale_nonuniform(&mut self, v: Vector3<f32>) {
        self.transform_mut().scale_nonuniform(v)
    }
}

impl Node {
    fn transform_mut(&mut self) -> &mut Transform {
        match self {
            Node::Light(light) => &mut light.transform,
            Node::Geometry(geometry) => &mut geometry.transform,
            Node::Transformation(transformation) => &mut transformation.transform,
        }
    }

    fn children_mut(&mut self) -> &mut Vec<Node> {
        match self {
            Node::Light(light) => &mut light.children,
            Node::Geometry(geometry) => &mut geometry.children,
            Node::Transformation(transformation) => &mut transformation.children,
        }
    }
}

pub struct Geometry {
    transform: Transform,
    children: Vec<Node>,
}

impl Geometry {
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
            children: Vec::new(),
        }
    }
}

impl Into<Node> for Geometry {
    fn into(self) -> Node {
        Node::Geometry(self)
    }
}

pub struct Light {
    transform: Transform,
    children: Vec<Node>,
}

impl Light {
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
            children: Vec::new(),
        }
    }
}

impl Into<Node> for Light {
    fn into(self) -> Node {
        Node::Light(self)
    }
}

pub struct Transformation {
    transform: Transform,
    children: Vec<Node>,
}

impl Transformation {
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
            children: Vec::new(),
        }
    }
}

impl Into<Node> for Transformation {
    fn into(self) -> Node {
        Node::Transformation(self)
    }
}
