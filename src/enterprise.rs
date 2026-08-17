use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ============================================================================
// Data Lineage & Provenance
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLineage {
    pub record_id: String,
    pub source: LineageSource,
    pub transformations: Vec<Transformation>,
    pub audit_trail: Vec<AuditEvent>,
    pub compliance_tags: Vec<ComplianceTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineageSource {
    OriginalData(String), // From production database
    SyntheticGeneration(GenerationMethod),
    Transformation(String),
    Migration(MigrationDetails),
    UserUpload(String),
    ApiIngest(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenerationMethod {
    SchemaBasedGeneration,
    BehaviorSimulation,
    FleetCoordination,
    DomainResearch,
    DataQualityDegradation,
    UnconventionalPattern,
    RealWorldMess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationDetails {
    pub from_system: String,
    pub to_system: String,
    pub migration_date: String,
    pub validated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transformation {
    pub transformation_id: String,
    pub transformer: String,
    pub timestamp: DateTime<Utc>,
    pub parameters: HashMap<String, String>,
    pub reversible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub event_id: String,
    pub event_type: AuditEventType,
    pub user: String,
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub status: EventStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditEventType {
    DataAccess,
    DataModification,
    DataExport,
    ComplianceCheck,
    SecurityAudit,
    PolicyEnforcement,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventStatus {
    Success,
    PartialSuccess,
    Failed,
    Blocked,
}

// ============================================================================
// Compliance Frameworks
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceTag {
    pub framework: ComplianceFramework,
    pub requirement_id: String,
    pub status: ComplianceStatus,
    pub last_verified: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplianceFramework {
    GDPR,
    HIPAA,
    CCPA,
    SOC2,
    PciDss,
    NIST,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplianceStatus {
    Compliant,
    PartiallyCompliant,
    NonCompliant,
    RequiresReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GDPRCompliance {
    pub data_anonymized: bool,
    pub pii_redacted: bool,
    pub retention_policy_applied: bool,
    pub user_consent_recorded: bool,
    pub processing_agreement_signed: bool,
    pub data_processing_purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HIPAACompliance {
    pub phi_encrypted: bool,
    pub access_controls_enforced: bool,
    pub audit_logging_enabled: bool,
    pub backup_procedures: bool,
    pub breach_notification_plan: bool,
    pub business_associate_agreement: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOC2Compliance {
    pub access_controls: bool,
    pub change_management: bool,
    pub monitoring_enabled: bool,
    pub incident_response_plan: bool,
    pub vendor_management: bool,
}

// ============================================================================
// Advanced Quality Metrics
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedQualityMetrics {
    pub completeness: f64,          // % of non-null fields
    pub uniqueness: f64,            // % of unique records
    pub validity: f64,              // % conforming to constraints
    pub consistency: f64,           // Cross-field consistency
    pub timeliness: f64,            // Recency of data
    pub accuracy: f64,              // % accurate vs ground truth
    pub referential_integrity: f64, // % of valid FKs
    pub temporal_integrity: f64,    // Time ordering correctness
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityScorecard {
    pub record_id: String,
    pub overall_quality_score: f64, // 0-100
    pub metrics: AdvancedQualityMetrics,
    pub issues: Vec<QualityIssue>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityIssue {
    pub field: String,
    pub issue_type: String,
    pub severity: IssueSeverity,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

// ============================================================================
// Data Governance
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataGovernancePolicy {
    pub policy_id: String,
    pub policy_name: String,
    pub retention_days: Option<u32>,
    pub encryption_required: bool,
    pub access_control_level: AccessControlLevel,
    pub data_classification: DataClassification,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccessControlLevel {
    Public,
    Internal,
    Confidential,
    Secret,
    TopSecret,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataClassification {
    Public,
    Internal,
    PII,
    PHI,
    FinancialData,
    TradeSecret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataGovernanceManager {
    policies: HashMap<String, DataGovernancePolicy>,
    audit_trail: Vec<AuditEvent>,
}

impl Default for DataGovernanceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DataGovernanceManager {
    pub fn new() -> Self {
        DataGovernanceManager {
            policies: HashMap::new(),
            audit_trail: Vec::new(),
        }
    }

    pub fn add_policy(&mut self, policy: DataGovernancePolicy) {
        self.policies.insert(policy.policy_id.clone(), policy);
    }

    pub fn get_policy(&self, policy_id: &str) -> Option<&DataGovernancePolicy> {
        self.policies.get(policy_id)
    }

    pub fn audit_access(&mut self, event: AuditEvent) {
        self.audit_trail.push(event);
    }

    pub fn get_audit_trail(&self) -> &[AuditEvent] {
        &self.audit_trail
    }
}

// ============================================================================
// Cost & Performance Tracking
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostEstimate {
    pub operation: String,
    pub records_processed: usize,
    pub compute_hours: f64,
    pub storage_gb: f64,
    pub estimated_cost_usd: f64,
    pub cost_per_record: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceProfile {
    pub operation: String,
    pub duration_seconds: f64,
    pub throughput_records_per_sec: f64,
    pub peak_memory_mb: f64,
    pub cpu_utilization_pct: f64,
    pub io_operations: usize,
}

pub struct CostAndPerformanceTracker {
    estimates: Vec<CostEstimate>,
    profiles: Vec<PerformanceProfile>,
}

impl Default for CostAndPerformanceTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl CostAndPerformanceTracker {
    pub fn new() -> Self {
        CostAndPerformanceTracker {
            estimates: Vec::new(),
            profiles: Vec::new(),
        }
    }

    pub fn add_estimate(&mut self, estimate: CostEstimate) {
        self.estimates.push(estimate);
    }

    pub fn add_profile(&mut self, profile: PerformanceProfile) {
        self.profiles.push(profile);
    }

    pub fn total_estimated_cost(&self) -> f64 {
        self.estimates.iter().map(|e| e.estimated_cost_usd).sum()
    }

    pub fn average_throughput(&self) -> f64 {
        if self.profiles.is_empty() {
            return 0.0;
        }
        let sum: f64 = self
            .profiles
            .iter()
            .map(|p| p.throughput_records_per_sec)
            .sum();
        sum / self.profiles.len() as f64
    }

    pub fn peak_memory(&self) -> f64 {
        self.profiles
            .iter()
            .map(|p| p.peak_memory_mb)
            .fold(0.0, f64::max)
    }
}

// ============================================================================
// Data Provenance & Reproducibility
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProvenance {
    pub dataset_id: String,
    pub generation_timestamp: DateTime<Utc>,
    pub generation_config: GenerationConfig,
    pub seed: u64,
    pub source_version: String,
    pub tool_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub schema_source: String,
    pub num_records: usize,
    pub messiness_level: String,
    pub quality_settings: HashMap<String, String>,
    pub transformations_applied: Vec<String>,
}

pub struct ProvenanceTracker {
    provenance_records: HashMap<String, DataProvenance>,
}

impl Default for ProvenanceTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl ProvenanceTracker {
    pub fn new() -> Self {
        ProvenanceTracker {
            provenance_records: HashMap::new(),
        }
    }

    pub fn record_generation(&mut self, dataset_id: String, config: GenerationConfig, seed: u64) {
        let provenance = DataProvenance {
            dataset_id: dataset_id.clone(),
            generation_timestamp: Utc::now(),
            generation_config: config,
            seed,
            source_version: "1.0".to_string(),
            tool_version: env!("CARGO_PKG_VERSION").to_string(),
        };
        self.provenance_records.insert(dataset_id, provenance);
    }

    pub fn is_reproducible(&self, dataset_id: &str) -> bool {
        self.provenance_records.contains_key(dataset_id)
    }

    pub fn get_provenance(&self, dataset_id: &str) -> Option<&DataProvenance> {
        self.provenance_records.get(dataset_id)
    }
}

// ============================================================================
// Regulatory Reporting
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryReport {
    pub report_id: String,
    pub framework: ComplianceFramework,
    pub generated_date: DateTime<Utc>,
    pub audit_trail: Vec<AuditEvent>,
    pub compliance_status: ComplianceStatus,
    pub findings: Vec<RegulatoryFinding>,
    pub executive_summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryFinding {
    pub requirement_id: String,
    pub finding_type: String,
    pub status: ComplianceStatus,
    pub remediation_plan: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
}

pub struct RegulatoryReportGenerator;

impl RegulatoryReportGenerator {
    pub fn generate_gdpr_report(dataset_id: &str, audit_trail: &[AuditEvent]) -> RegulatoryReport {
        RegulatoryReport {
            report_id: Uuid::new_v4().to_string(),
            framework: ComplianceFramework::GDPR,
            generated_date: Utc::now(),
            audit_trail: audit_trail.to_vec(),
            compliance_status: ComplianceStatus::RequiresReview,
            findings: vec![],
            executive_summary: format!("GDPR Compliance Report for dataset: {}", dataset_id),
        }
    }

    pub fn generate_hipaa_report(dataset_id: &str, audit_trail: &[AuditEvent]) -> RegulatoryReport {
        RegulatoryReport {
            report_id: Uuid::new_v4().to_string(),
            framework: ComplianceFramework::HIPAA,
            generated_date: Utc::now(),
            audit_trail: audit_trail.to_vec(),
            compliance_status: ComplianceStatus::RequiresReview,
            findings: vec![],
            executive_summary: format!("HIPAA Compliance Report for dataset: {}", dataset_id),
        }
    }

    pub fn generate_soc2_report(dataset_id: &str, audit_trail: &[AuditEvent]) -> RegulatoryReport {
        RegulatoryReport {
            report_id: Uuid::new_v4().to_string(),
            framework: ComplianceFramework::SOC2,
            generated_date: Utc::now(),
            audit_trail: audit_trail.to_vec(),
            compliance_status: ComplianceStatus::RequiresReview,
            findings: vec![],
            executive_summary: format!("SOC2 Compliance Report for dataset: {}", dataset_id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_lineage_creation() {
        let lineage = DataLineage {
            record_id: "rec_123".to_string(),
            source: LineageSource::OriginalData("prod_db".to_string()),
            transformations: vec![],
            audit_trail: vec![],
            compliance_tags: vec![],
        };

        assert_eq!(lineage.record_id, "rec_123");
    }

    #[test]
    fn test_governance_manager() {
        let mut manager = DataGovernanceManager::new();
        let policy = DataGovernancePolicy {
            policy_id: "pol_1".to_string(),
            policy_name: "Production Data Policy".to_string(),
            retention_days: Some(90),
            encryption_required: true,
            access_control_level: AccessControlLevel::Confidential,
            data_classification: DataClassification::PII,
        };

        manager.add_policy(policy.clone());
        assert!(manager.get_policy("pol_1").is_some());
    }

    #[test]
    fn test_cost_tracking() {
        let mut tracker = CostAndPerformanceTracker::new();

        tracker.add_estimate(CostEstimate {
            operation: "generate_synthetic_data".to_string(),
            records_processed: 1_000_000,
            compute_hours: 2.5,
            storage_gb: 5.0,
            estimated_cost_usd: 25.0,
            cost_per_record: 0.000025,
        });

        assert_eq!(tracker.total_estimated_cost(), 25.0);
    }

    #[test]
    fn test_provenance_tracking() {
        let mut tracker = ProvenanceTracker::new();
        let config = GenerationConfig {
            schema_source: "banking.yaml".to_string(),
            num_records: 100_000,
            messiness_level: "Level 3".to_string(),
            quality_settings: HashMap::new(),
            transformations_applied: vec![],
        };

        tracker.record_generation("ds_123".to_string(), config, 42);
        assert!(tracker.is_reproducible("ds_123"));
    }

    #[test]
    fn test_regulatory_report_generation() {
        let report = RegulatoryReportGenerator::generate_gdpr_report("ds_123", &[]);
        assert_eq!(report.framework, ComplianceFramework::GDPR);
    }
}
