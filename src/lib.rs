// pyo3's `#[pymethods]` macro expansion triggers clippy's `useless_conversion`
// lint on essentially every method returning `PyResult<T>` (a known pyo3/clippy
// interaction, not a real bug in this crate's code) — this predates the changes
// in this file; silencing it here keeps `cargo clippy -D warnings` meaningful
// for actual issues instead of macro noise.
#![allow(clippy::useless_conversion)]

use pyo3::prelude::*;
use std::collections::HashMap;

pub mod behaviors;
pub mod data_quality;
pub mod enterprise;
pub mod errors;
pub mod generator;
pub mod monitoring;
pub mod parser;
pub mod real_world_mess;
pub mod research;
pub mod robotics;
pub mod ros2_bridge;
pub mod schema;
pub mod unconventional_data;
pub mod validation;

use data_quality::DataQualityAnalyzer;
use enterprise::{DataGovernanceManager, DataGovernancePolicy as RustDataGovernancePolicy};
use generator::WorldGenerator;
use schema::{Cardinality, Constraint, ConstraintType, Field, FieldType, Relationship, Schema};

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
    Ok(())
}

fn value_error(msg: impl std::fmt::Display) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyValueError, _>(msg.to_string())
}

fn parse_field_type(type_str: &str) -> PyResult<FieldType> {
    match type_str.to_lowercase().as_str() {
        "string" => Ok(FieldType::String),
        "int" | "integer" => Ok(FieldType::Int),
        "float" | "double" => Ok(FieldType::Float),
        "boolean" | "bool" => Ok(FieldType::Boolean),
        "datetime" | "timestamp" => Ok(FieldType::DateTime),
        "uuid" => Ok(FieldType::Uuid),
        "json" => Ok(FieldType::Json),
        s if s.starts_with("enum(") => {
            let values: Vec<String> = s
                .trim_start_matches("enum(")
                .trim_end_matches(')')
                .split(',')
                .map(|v| v.trim().to_string())
                .collect();
            Ok(FieldType::Enum(values))
        }
        other => Err(value_error(format!("Unknown field type: {}", other))),
    }
}

fn parse_cardinality(raw: &str) -> PyResult<Cardinality> {
    match raw.to_lowercase().as_str() {
        "1:1" | "one_to_one" => Ok(Cardinality::OneToOne),
        "1:n" | "one_to_many" => Ok(Cardinality::OneToMany),
        "n:m" | "many_to_many" => Ok(Cardinality::ManyToMany),
        other => Err(value_error(format!("Unknown cardinality: {}", other))),
    }
}

fn parse_constraint_type(raw: &str) -> PyResult<ConstraintType> {
    match raw.to_lowercase().as_str() {
        "range" => Ok(ConstraintType::Range),
        "length" => Ok(ConstraintType::Length),
        "pattern" => Ok(ConstraintType::Pattern),
        "custom" => Ok(ConstraintType::Custom),
        other => Err(value_error(format!("Unknown constraint type: {}", other))),
    }
}

/// Recursively convert a `serde_json::Value` into a native Python object
/// (dict/list/str/int/float/bool/None) so generated rows are directly usable
/// from Python without any intermediate stub layer.
fn json_to_py(py: Python<'_>, value: &serde_json::Value) -> PyResult<PyObject> {
    use serde_json::Value;
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(b) => Ok(b.into_py(py)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_py(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_py(py))
            } else {
                Ok(n.to_string().into_py(py))
            }
        }
        Value::String(s) => Ok(s.into_py(py)),
        Value::Array(arr) => {
            let mut items = Vec::with_capacity(arr.len());
            for v in arr {
                items.push(json_to_py(py, v)?);
            }
            Ok(pyo3::types::PyList::new_bound(py, &items).into_py(py))
        }
        Value::Object(map) => {
            let dict = pyo3::types::PyDict::new_bound(py);
            for (k, v) in map {
                dict.set_item(k, json_to_py(py, v)?)?;
            }
            Ok(dict.into_py(py))
        }
    }
}

#[pyclass(name = "Schema")]
#[derive(Clone)]
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

    /// Parse a schema from a YAML file on disk (entities/fields/relationships/constraints).
    #[staticmethod]
    fn from_yaml(path: String) -> PyResult<Self> {
        parser::SchemaParser::from_yaml(&path)
            .map(|inner| PySchema { inner })
            .map_err(value_error)
    }

    /// Parse a schema from a YAML string.
    #[staticmethod]
    fn from_yaml_str(yaml: String) -> PyResult<Self> {
        parser::SchemaParser::from_yaml_string(&yaml)
            .map(|inner| PySchema { inner })
            .map_err(value_error)
    }

    fn add_entity(&mut self, name: String) -> PyResult<()> {
        self.inner.add_entity(name);
        Ok(())
    }

    #[pyo3(signature = (entity, name, field_type, nullable=false, unique=false, constraints=None))]
    fn add_field(
        &mut self,
        entity: String,
        name: String,
        field_type: String,
        nullable: bool,
        unique: bool,
        constraints: Option<Vec<String>>,
    ) -> PyResult<()> {
        let field_type = parse_field_type(&field_type)?;
        let field = Field {
            name,
            field_type,
            nullable,
            unique,
            constraints: constraints.unwrap_or_default(),
        };
        self.inner.add_field(&entity, field).map_err(value_error)
    }

    fn add_relationship(
        &mut self,
        from_entity: String,
        to_entity: String,
        from_field: String,
        to_field: String,
        cardinality: String,
    ) -> PyResult<()> {
        let cardinality = parse_cardinality(&cardinality)?;
        self.inner.add_relationship(Relationship {
            from_entity,
            to_entity,
            from_field,
            to_field,
            cardinality,
        });
        Ok(())
    }

    #[pyo3(signature = (constraint_type, entity, value, field=None))]
    fn add_constraint(
        &mut self,
        constraint_type: String,
        entity: String,
        value: String,
        field: Option<String>,
    ) -> PyResult<()> {
        let constraint_type = parse_constraint_type(&constraint_type)?;
        self.inner.add_constraint(Constraint {
            constraint_type,
            entity,
            field,
            value,
        });
        Ok(())
    }

    fn entity_names(&self) -> Vec<String> {
        self.inner.entities.keys().cloned().collect()
    }

    fn to_yaml(&self) -> PyResult<String> {
        self.inner.to_yaml().map_err(value_error)
    }

    fn to_json(&self) -> PyResult<String> {
        self.inner.to_json().map_err(value_error)
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

    /// Generate `num_records` rows per entity and return a dict:
    /// {"entities": {entity_name: [row_dict, ...]}, "metadata": {...}, "quality": {...}}
    ///
    /// `quality.fidelity_score` and `quality.constraint_violations` are computed for
    /// real against the schema (nullability, uniqueness, range/length/pattern
    /// constraints) — not hardcoded stubs.
    fn generate(&self, num_records: usize, seed: u64) -> PyResult<PyObject> {
        let world = self
            .inner
            .generate(num_records, seed)
            .map_err(value_error)?;
        let report = self.inner.evaluate(&world);

        Python::with_gil(|py| {
            let entities_dict = pyo3::types::PyDict::new_bound(py);
            for (entity_name, rows) in &world.entities {
                let mut py_rows = Vec::with_capacity(rows.len());
                for row in rows {
                    py_rows.push(json_to_py(py, row)?);
                }
                let list = pyo3::types::PyList::new_bound(py, &py_rows);
                entities_dict.set_item(entity_name, list)?;
            }

            let metadata = pyo3::types::PyDict::new_bound(py);
            metadata.set_item("seed", world.metadata.seed)?;
            metadata.set_item("record_count", world.metadata.record_count)?;
            metadata.set_item(
                "generation_time_ms",
                world.metadata.generation_time_ms as u64,
            )?;

            let quality = pyo3::types::PyDict::new_bound(py);
            quality.set_item("fidelity_score", report.fidelity_score)?;
            quality.set_item("total_checks", report.total_checks)?;
            quality.set_item("null_violations", report.null_violations)?;
            quality.set_item("uniqueness_violations", report.uniqueness_violations)?;
            quality.set_item("constraint_violations", report.constraint_violations)?;

            let result = pyo3::types::PyDict::new_bound(py);
            result.set_item("entities", entities_dict)?;
            result.set_item("metadata", metadata)?;
            result.set_item("quality", quality)?;

            Ok(result.into())
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

    /// Analyze row data (a list of string-keyed/string-valued dicts, the same
    /// shape produced by `DataQualityDegradation`) and return real counts —
    /// missing/"NULL" markers, exact-duplicate rows, injected-outlier
    /// markers, temporal-offset markers — plus a derived quality score. No
    /// hardcoded values. `inconsistent_records` is always 0: detecting
    /// logical inconsistency between semantically-related fields needs
    /// domain knowledge this generic analyzer doesn't have, so it's left
    /// honestly unimplemented rather than faked.
    fn analyze(&self, data: Vec<HashMap<String, String>>) -> PyDataQualityMetrics {
        let metrics = DataQualityAnalyzer::analyze(&data);
        PyDataQualityMetrics {
            total_records: metrics.total_records,
            missing_values: metrics.missing_values,
            duplicate_records: metrics.duplicate_records,
            outlier_records: metrics.outlier_records,
            inconsistent_records: metrics.inconsistent_records,
            temporal_issues: metrics.temporal_issues,
            overall_quality_score: metrics.overall_quality_score,
        }
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
    fn new(
        total: usize,
        missing: usize,
        duplicates: usize,
        outliers: usize,
        inconsistent: usize,
        temporal: usize,
        score: f64,
    ) -> Self {
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
            self.total_records,
            self.missing_values,
            self.duplicate_records,
            self.outlier_records,
            self.overall_quality_score
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
//
// NOTE ON SCOPE: the GDPR/HIPAA/SOC2 "compliance" classes that used to live
// here (`check_consent`, `encrypt_phi`, `verify_access_controls`) always
// returned a hardcoded `true`/success regardless of actual state — a
// compliance API that always says "compliant" is worse than no API, so they
// were deleted rather than kept as decoration. This governance manager is
// kept because it does something real and honest: it stores whatever policy
// it's given and returns exactly that back, with no legal/compliance claim
// attached.
// ============================================================================

#[pyclass(name = "DataGovernanceManager")]
pub struct PyDataGovernanceManager {
    inner: DataGovernanceManager,
}

#[pymethods]
impl PyDataGovernanceManager {
    #[new]
    fn new() -> Self {
        PyDataGovernanceManager {
            inner: DataGovernanceManager::new(),
        }
    }

    /// Actually store a policy (previously a no-op stub) and return it.
    #[pyo3(signature = (policy_id, policy_name, retention_days=None, encryption_required=false))]
    fn create_policy(
        &mut self,
        policy_id: String,
        policy_name: String,
        retention_days: Option<u32>,
        encryption_required: bool,
    ) -> PyResult<PyDataGovernancePolicy> {
        let policy = RustDataGovernancePolicy {
            policy_id: policy_id.clone(),
            policy_name: policy_name.clone(),
            retention_days,
            encryption_required,
            access_control_level: enterprise::AccessControlLevel::Internal,
            data_classification: enterprise::DataClassification::Internal,
        };
        self.inner.add_policy(policy);
        Ok(PyDataGovernancePolicy {
            policy_id,
            name: policy_name,
        })
    }

    /// Look up a previously created policy by id. Returns `None` if it was
    /// never created (rather than silently pretending one exists).
    fn get_policy(&self, policy_id: String) -> Option<PyDataGovernancePolicy> {
        self.inner
            .get_policy(&policy_id)
            .map(|p| PyDataGovernancePolicy {
                policy_id: p.policy_id.clone(),
                name: p.policy_name.clone(),
            })
    }
}

#[pyclass(name = "DataGovernancePolicy")]
pub struct PyDataGovernancePolicy {
    #[pyo3(get)]
    policy_id: String,
    name: String,
}

#[pymethods]
impl PyDataGovernancePolicy {
    #[new]
    fn new(name: String) -> Self {
        PyDataGovernancePolicy {
            policy_id: String::new(),
            name,
        }
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
        format!(
            "AuditEvent(type='{}', timestamp='{}')",
            self.event_type, self.timestamp
        )
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
