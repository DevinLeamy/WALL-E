use nalgebra::Vector3;
use pyo3::prelude::*;

use wall_e::prelude::{
    Camera, Geometry, Light, Node, PhongMaterial, PngImage, RayTracer, Scene, Transformation,
};

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

    fn set_material(&mut self, material: PyRef<PyMaterial>) {
        self.inner.set_material(material.inner.clone());
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

#[pyclass]
#[pyo3(name = "Camera")]
struct PyCamera {
    inner: Camera,
}

#[pymethods]
impl PyCamera {
    #[new]
    fn new(
        position: (f32, f32, f32),
        view: (f32, f32, f32),
        up: (f32, f32, f32),
        fov: f32,
    ) -> Self {
        Self {
            inner: Camera::new(
                Vector3::new(position.0, position.1, position.2),
                fov,
                Vector3::new(up.0, up.1, up.2),
                Vector3::new(view.0, view.1, view.2),
            ),
        }
    }

    fn set_fov(&mut self, fov: f32) {
        self.inner.set_fov(fov);
    }

    fn set_up(&mut self, x: f32, y: f32, z: f32) {
        self.inner.set_up(Vector3::new(x, y, z));
    }

    fn set_view(&mut self, x: f32, y: f32, z: f32) {
        self.inner.set_target(Vector3::new(x, y, z));
    }

    fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.inner.set_position(Vector3::new(x, y, z));
    }

    fn look_at(&mut self, x: f32, y: f32, z: f32) {
        self.inner.look_at(Vector3::new(x, y, z));
    }
}

#[pyclass]
#[pyo3(name = "Material")]
struct PyMaterial {
    inner: PhongMaterial,
}

#[pymethods]
impl PyMaterial {
    #[new]
    fn new(kd: (f32, f32, f32), ks: (f32, f32, f32), shininess: f32) -> Self {
        Self {
            inner: PhongMaterial::new(
                Vector3::new(kd.0, kd.1, kd.2),
                Vector3::new(ks.0, ks.1, ks.2),
                shininess,
            ),
        }
    }
}

#[pyfunction]
fn ray_trace(
    scene: &PyScene,
    camera: &PyCamera,
    width: u32,
    height: u32,
    path: String,
) -> PyResult<()> {
    let mut tracer = RayTracer::new(
        PngImage::new(width, height),
        scene.inner.clone(),
        camera.inner.clone(),
    );
    let output = tracer.run();

    output.save(path);

    Ok(())
}

#[pymodule]
fn wall_e_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyGeometry>()?;
    m.add_class::<PyScene>()?;
    m.add_class::<PyLight>()?;
    m.add_class::<PyTransform>()?;
    m.add_class::<PyCamera>()?;
    m.add_class::<PyMaterial>()?;
    m.add_function(wrap_pyfunction!(ray_trace, m)?)?;

    Ok(())
}
