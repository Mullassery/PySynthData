use pyo3::prelude::*;

pub mod schema;
pub mod parser;
pub mod generator;
pub mod validation;
pub mod errors;
pub mod behaviors;
pub mod robotics;
pub mod research;
pub mod ros2_bridge;
pub mod monitoring;
pub mod data_quality;
pub mod unconventional_data;

use schema::Schema;
use generator::WorldGenerator;

#[pymodule]
fn pysynthdata(_py: Python, m: &pyo3::Bound<pyo3::types::PyModule>) -> PyResult<()> {
    m.add_class::<PySchema>()?;
    m.add_class::<PyWorldGenerator>()?;
    Ok(())
}

#[pyclass(name = "Schema")]
pub struct PySchema {
    inner: Schema,
}

#[pymethods]
impl PySchema {
    #[new]
    fn new() -> Self {
        PySchema {
            inner: Schema::new(),
        }
    }

    fn add_entity(&mut self, name: String) -> PyResult<()> {
        self.inner.add_entity(name);
        Ok(())
    }

    fn to_yaml(&self) -> PyResult<String> {
        self.inner.to_yaml().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
        })
    }
}

#[pyclass(name = "WorldGenerator")]
pub struct PyWorldGenerator {
    inner: WorldGenerator,
}

#[pymethods]
impl PyWorldGenerator {
    #[new]
    fn new(schema: &PySchema) -> Self {
        PyWorldGenerator {
            inner: WorldGenerator::new(schema.inner.clone()),
        }
    }

    fn generate(&self, num_records: usize, seed: u64) -> PyResult<PyObject> {
        self.inner.generate(num_records, seed)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Python::with_gil(|py| {
            Ok(pyo3::types::PyDict::new_bound(py).into())
        })
    }
}
