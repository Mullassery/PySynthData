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
pub mod real_world_mess;
pub mod enterprise;

use schema::Schema;
use generator::WorldGenerator;
use data_quality::{DataQualityAnalyzer, DataQualityMetrics, OutlierPattern};
use enterprise::{DataGovernanceManager, DataGovernancePolicy, AuditEvent, AuditEventType, GDPRCompliance, HIPAACompliance, SOC2Compliance, ComplianceStatus};

#[pymodule]
#[pyo3(name = "_core")]
fn pysynthdata(_py: Python, m: &pyo3::Bound<pyo3::types::PyModule>) -> PyResult<()> {
    m.add_class::<PySchema>()?;
    m.add_class::<PyWorldGenerator>()?;
    m.add_class::<PyDataQualityAnalyzer>()?;
    m.add_class::<PyDataQualityMetrics>()?;
    m.add_class::<PyOutlierPattern>()?;
    m.add_class::<PyDataGovernanceManager>()?;
    m.add_class::<PyDataGovernancePolicy>()?;
    m.add_class::<PyAuditEvent>()?;
    m.add_class::<PyAuditEventType>()?;
    m.add_class::<PyGDPRCompliance>()?;
    m.add_class::<PyHIPAACompliance>()?;
    m.add_class::<PySOC2Compliance>()?;
    m.add_class::<PyComplianceStatus>()?;
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

// ============================================================================
// DATA QUALITY CLASSES
// ============================================================================

#[pyclass(name = "DataQualityAnalyzer")]
pub struct PyDataQualityAnalyzer;

#[pymethods]
impl PyDataQualityAnalyzer {
    #[new]
    fn new() -> Self {
        PyDataQualityAnalyzer
    }

    fn __repr__(&self) -> String {
        "DataQualityAnalyzer()".to_string()
    }
}

#[pyclass(name = "DataQualityMetrics")]
pub struct PyDataQualityMetrics {
    #[pyo3(get)]
    total_records: usize,
    #[pyo3(get)]
    missing_values: usize,
    #[pyo3(get)]
    duplicate_records: usize,
    #[pyo3(get)]
    outlier_records: usize,
    #[pyo3(get)]
    inconsistent_records: usize,
    #[pyo3(get)]
    temporal_issues: usize,
    #[pyo3(get)]
    overall_quality_score: f64,
}

#[pymethods]
impl PyDataQualityMetrics {
    #[new]
    fn new(total: usize, missing: usize, duplicates: usize, outliers: usize, inconsistent: usize, temporal: usize, score: f64) -> Self {
        PyDataQualityMetrics {
            total_records: total,
            missing_values: missing,
            duplicate_records: duplicates,
            outlier_records: outliers,
            inconsistent_records: inconsistent,
            temporal_issues: temporal,
            overall_quality_score: score,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "DataQualityMetrics(total={}, missing={}, duplicates={}, outliers={}, score={:.2})",
            self.total_records, self.missing_values, self.duplicate_records, self.outlier_records, self.overall_quality_score
        )
    }
}

#[pyclass(name = "OutlierPattern")]
pub struct PyOutlierPattern;

#[pymethods]
impl PyOutlierPattern {
    #[staticmethod]
    fn statistical() -> &'static str {
        "statistical"
    }

    #[staticmethod]
    fn isolation_forest() -> &'static str {
        "isolation_forest"
    }

    #[staticmethod]
    fn local_outlier_factor() -> &'static str {
        "local_outlier_factor"
    }
}

// ============================================================================
// DATA GOVERNANCE CLASSES
// ============================================================================

#[pyclass(name = "DataGovernanceManager")]
pub struct PyDataGovernanceManager;

#[pymethods]
impl PyDataGovernanceManager {
    #[new]
    fn new() -> Self {
        PyDataGovernanceManager
    }

    fn create_policy(&mut self, name: String) -> PyResult<()> {
        Ok(())
    }
}

#[pyclass(name = "DataGovernancePolicy")]
pub struct PyDataGovernancePolicy {
    name: String,
}

#[pymethods]
impl PyDataGovernancePolicy {
    #[new]
    fn new(name: String) -> Self {
        PyDataGovernancePolicy { name }
    }

    #[getter]
    fn name(&self) -> String {
        self.name.clone()
    }

    fn __repr__(&self) -> String {
        format!("DataGovernancePolicy(name='{}')", self.name)
    }
}

#[pyclass(name = "AuditEvent")]
pub struct PyAuditEvent {
    event_type: String,
    timestamp: String,
}

#[pymethods]
impl PyAuditEvent {
    #[new]
    fn new(event_type: String) -> Self {
        use chrono::Local;
        PyAuditEvent {
            event_type,
            timestamp: Local::now().to_rfc3339(),
        }
    }

    #[getter]
    fn event_type(&self) -> String {
        self.event_type.clone()
    }

    #[getter]
    fn timestamp(&self) -> String {
        self.timestamp.clone()
    }

    fn __repr__(&self) -> String {
        format!("AuditEvent(type='{}', timestamp='{}')", self.event_type, self.timestamp)
    }
}

#[pyclass(name = "AuditEventType")]
pub struct PyAuditEventType;

#[pymethods]
impl PyAuditEventType {
    #[staticmethod]
    fn data_access() -> &'static str {
        "data_access"
    }

    #[staticmethod]
    fn data_modification() -> &'static str {
        "data_modification"
    }

    #[staticmethod]
    fn policy_change() -> &'static str {
        "policy_change"
    }

    #[staticmethod]
    fn compliance_check() -> &'static str {
        "compliance_check"
    }
}

#[pyclass(name = "GDPRCompliance")]
pub struct PyGDPRCompliance;

#[pymethods]
impl PyGDPRCompliance {
    #[new]
    fn new() -> Self {
        PyGDPRCompliance
    }

    fn check_consent(&self) -> PyResult<bool> {
        Ok(true)
    }

    fn anonymize_personal_data(&self) -> PyResult<()> {
        Ok(())
    }

    fn __repr__(&self) -> String {
        "GDPRCompliance()".to_string()
    }
}

#[pyclass(name = "HIPAACompliance")]
pub struct PyHIPAACompliance;

#[pymethods]
impl PyHIPAACompliance {
    #[new]
    fn new() -> Self {
        PyHIPAACompliance
    }

    fn encrypt_phi(&self) -> PyResult<()> {
        Ok(())
    }

    fn audit_access_logs(&self) -> PyResult<()> {
        Ok(())
    }

    fn __repr__(&self) -> String {
        "HIPAACompliance()".to_string()
    }
}

#[pyclass(name = "SOC2Compliance")]
pub struct PySOC2Compliance;

#[pymethods]
impl PySOC2Compliance {
    #[new]
    fn new() -> Self {
        PySOC2Compliance
    }

    fn verify_access_controls(&self) -> PyResult<bool> {
        Ok(true)
    }

    fn __repr__(&self) -> String {
        "SOC2Compliance()".to_string()
    }
}

#[pyclass(name = "ComplianceStatus")]
pub struct PyComplianceStatus;

#[pymethods]
impl PyComplianceStatus {
    #[staticmethod]
    fn compliant() -> &'static str {
        "compliant"
    }

    #[staticmethod]
    fn non_compliant() -> &'static str {
        "non_compliant"
    }

    #[staticmethod]
    fn requires_review() -> &'static str {
        "requires_review"
    }
}
