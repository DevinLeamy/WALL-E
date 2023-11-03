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
    pub fn transform(&self) -> &Transform {
        match self {
            Node::Light(light) => light.transform(),
            Node::Geometry(geometry) => geometry.transform(),
            Node::Transformation(transformation) => transformation.transform(),
        }
    }

    pub fn children(&self) -> &Vec<Node> {
        match self {
            Node::Light(light) => &light.children,
            Node::Geometry(geometry) => &geometry.children,
            Node::Transformation(transformation) => &transformation.children,
        }
    }

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

impl Default for Geometry {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            children: Vec::new(),
            material: PhongMaterial::default(),
            primitive: Box::new(Sphere::new(1.0)),
        }
    }
}

impl Geometry {
    pub fn new(
        transform: Transform,
        material: PhongMaterial,
        primitive: Box<dyn Collidable>,
        children: Vec<Node>,
    ) -> Self {
        Self {
            transform,
            children,
            material,
            primitive,
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

    pub fn children(&self) -> &Vec<Node> {
        &self.children
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

    pub fn primitive(&self) -> &Box<dyn Collidable> {
        &self.primitive
    }

    pub fn material(&self) -> &PhongMaterial {
        &self.material
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

impl Default for Light {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            children: Vec::new(),
        }
    }
}

impl Light {
    pub fn new(transform: Transform, children: Vec<Node>) -> Self {
        Self {
            transform,
            children,
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn children(&self) -> &Vec<Node> {
        &self.children
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

    pub fn children(&self) -> &Vec<Node> {
        &self.children
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
