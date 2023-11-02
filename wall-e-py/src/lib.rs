use nalgebra::Vector3;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3::wrap_pyfunction;

// Import your Rust structures here
use wall_e::prelude::{Geometry, Light, Node, Scene, Transformation};

#[pyclass]
#[pyo3(name = "Node")]
struct PyNode {
    inner: Node,
}

#[pymethods]
impl PyNode {
    #[new]
    fn new(obj_type: &str) -> PyResult<Self> {
        let inner = match obj_type {
            "light" => Node::Light(Light::new().into()),
            "transformation" => Node::Transformation(Transformation::new().into()),
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Unknown node type",
                ))
            }
        };
        Ok(Self { inner })
    }

    fn rotate(&mut self, v: (f32, f32, f32)) {
        self.inner.rotate(Vector3::new(v.0, v.1, v.2));
    }

    fn scale_nonuniform(&mut self, v: (f32, f32, f32)) {
        self.inner.scale_nonuniform(Vector3::new(v.0, v.1, v.2));
    }

    fn add_child(&mut self, child: PyRef<PyNode>) {
        self.inner.add_child(child.inner.clone());
    }
}

#[pyclass]
#[pyo3(name = "Geometry")]
struct PyGeometry {
    inner: Geometry,
}

#[pymethods]
impl PyGeometry {
    #[new]
    fn new(primitive_type: &str) -> PyResult<Self> {
        // You should implement a function to create a geometry based on a string.
        // For example, "sphere" should return a Geometry with a Sphere primitive.
        // let inner = Geometry::from_primitive(/* ... */);
        let inner = Geometry::new();
        Ok(Self { inner })
    }

    fn scale_nonuniform(&mut self, v: (f32, f32, f32)) {
        self.inner
            .transform_mut()
            .scale_nonuniform(Vector3::new(v.0, v.1, v.2));
    }
}

#[pyclass]
#[pyo3(name = "Scene")]
struct PyScene {
    inner: Scene,
}

#[pymethods]
impl PyScene {
    #[new]
    fn new() -> Self {
        Self {
            inner: Scene::new(),
        }
    }

    fn add_child(&mut self, child: PyRef<PyNode>) {
        self.inner.root_mut().add_child(child.inner.clone());
    }
}

#[pymodule]
fn wall_e_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyNode>()?;
    m.add_class::<PyGeometry>()?;
    m.add_class::<PyScene>()?;
    Ok(())
}
