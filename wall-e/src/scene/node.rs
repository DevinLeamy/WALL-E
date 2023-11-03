use nalgebra::Vector3;

use super::{Collidable, PhongMaterial, Sphere, Transform};

#[derive(Clone)]
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
            Node::Light(light) => light.transform_mut(),
            Node::Geometry(geometry) => geometry.transform_mut(),
            Node::Transformation(transformation) => transformation.transform_mut(),
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

#[derive(Clone)]
pub struct Geometry {
    transform: Transform,
    children: Vec<Node>,
    material: PhongMaterial,
    primitive: Box<dyn Collidable>,
}

impl Geometry {
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
            children: Vec::new(),
            material: PhongMaterial::default(),
            primitive: Box::new(Sphere::new(1.0)),
        }
    }

    pub fn from_primitive(primitive: Box<dyn Collidable>) -> Self {
        Self {
            transform: Transform::default(),
            children: Vec::new(),
            material: PhongMaterial::default(),
            primitive,
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn set_material(&mut self, material: PhongMaterial) {
        self.material = material;
    }
}

impl Into<Node> for Geometry {
    fn into(self) -> Node {
        Node::Geometry(self)
    }
}

#[derive(Clone)]
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

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

impl Into<Node> for Light {
    fn into(self) -> Node {
        Node::Light(self)
    }
}

#[derive(Clone)]
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

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

impl Into<Node> for Transformation {
    fn into(self) -> Node {
        Node::Transformation(self)
    }
}
