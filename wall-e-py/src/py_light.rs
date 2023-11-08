pub use crate::prelude::*;

#[pyclass]
#[pyo3(name = "Light")]
#[derive(PyNode)]
pub struct PyLight {
    pub inner: Light,
}

#[pymethods]
impl PyLight {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: Light::default(),
        })
    }
}
