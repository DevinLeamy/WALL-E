use nalgebra::Vector3;
use pyo3::prelude::*;

use wall_e::prelude::{Geometry, Light, Node, Scene, Transformation};

#[pyclass]
#[pyo3(name = "Node")]
struct PyNode {
    inner: Node,
}

#[pymethods]
impl PyNode {
    fn add_child(&mut self, py: Python, child: PyObject) {
        if let Ok(child) = child.extract::<PyRef<PyGeometry>>(py) {
            self.inner.add_child(child.inner.clone().into());
        } else if let Ok(child) = child.extract::<PyRef<PyTransform>>(py) {
            self.inner.add_child(child.inner.clone().into());
        } else if let Ok(child) = child.extract::<PyRef<PyLight>>(py) {
            self.inner.add_child(child.inner.clone().into());
        } else {
            panic!("add_child only accepts PyGeometry, PyTransform, or PyLight");
        }
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
        // TODO: Convert the primitive type into an actual primitive.
        Ok(Self {
            inner: Geometry::new(),
        })
    }

    fn add_child(&mut self, py: Python, child: PyObject) {
        if let Ok(child) = child.extract::<PyRef<PyGeometry>>(py) {
            self.inner.add_child(child.inner.clone().into());
        } else if let Ok(child) = child.extract::<PyRef<PyTransform>>(py) {
            self.inner.add_child(child.inner.clone().into());
        } else if let Ok(child) = child.extract::<PyRef<PyLight>>(py) {
            self.inner.add_child(child.inner.clone().into());
        } else {
            panic!("add_child only accepts PyGeometry, PyTransform, or PyLight");
        }
    }

    fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.inner
            .transform_mut()
            .scale_nonuniform(Vector3::new(x, y, z));
    }

    fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.inner.transform_mut().translate(Vector3::new(x, y, z));
    }
}

#[pyclass]
#[pyo3(name = "Light")]
struct PyLight {
    inner: Light,
}

#[pymethods]
impl PyLight {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: Light::new(),
        })
    }

    fn add_child(&mut self, py: Python, child: PyObject) {
        if let Ok(child) = child.extract::<PyRef<PyGeometry>>(py) {
            self.inner.add_child(child.inner.clone().into());
        } else if let Ok(child) = child.extract::<PyRef<PyTransform>>(py) {
            self.inner.add_child(child.inner.clone().into());
        } else if let Ok(child) = child.extract::<PyRef<PyLight>>(py) {
            self.inner.add_child(child.inner.clone().into());
        } else {
            panic!("add_child only accepts PyGeometry, PyTransform, or PyLight");
        }
    }

    fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.inner
            .transform_mut()
            .scale_nonuniform(Vector3::new(x, y, z));
    }

    fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.inner.transform_mut().translate(Vector3::new(x, y, z));
    }
}

#[pyclass]
#[pyo3(name = "Transform")]
struct PyTransform {
    inner: Transformation,
}

#[pymethods]
impl PyTransform {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: Transformation::new(),
        })
    }

    fn add_child(&mut self, py: Python, child: PyObject) {
        if let Ok(child) = child.extract::<PyRef<PyGeometry>>(py) {
            self.inner.add_child(child.inner.clone().into());
        } else if let Ok(child) = child.extract::<PyRef<PyTransform>>(py) {
            self.inner.add_child(child.inner.clone().into());
        } else if let Ok(child) = child.extract::<PyRef<PyLight>>(py) {
            self.inner.add_child(child.inner.clone().into());
        } else {
            panic!("add_child only accepts PyGeometry, PyTransform, or PyLight");
        }
    }

    fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.inner
            .transform_mut()
            .scale_nonuniform(Vector3::new(x, y, z));
    }

    fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.inner.transform_mut().translate(Vector3::new(x, y, z));
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

    fn set_root(&mut self, py: Python, root: PyObject) {
        if let Ok(child) = root.extract::<PyRef<PyGeometry>>(py) {
            *self.inner.root_mut() = child.inner.clone().into();
        } else if let Ok(child) = root.extract::<PyRef<PyTransform>>(py) {
            *self.inner.root_mut() = child.inner.clone().into();
        } else if let Ok(child) = root.extract::<PyRef<PyLight>>(py) {
            *self.inner.root_mut() = child.inner.clone().into();
        } else {
            panic!("add_child only accepts PyGeometry, PyTransform, or PyLight");
        }
    }
}

#[pymodule]
fn wall_e_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyGeometry>()?;
    m.add_class::<PyScene>()?;
    m.add_class::<PyLight>()?;
    m.add_class::<PyTransform>()?;
    Ok(())
}
