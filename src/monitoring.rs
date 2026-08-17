use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoringConfig {
    pub track_schema_drift: bool,
    pub track_data_drift: bool,
    pub track_constraint_violations: bool,
    pub track_edge_cases: bool,
    pub track_performance_metrics: bool,
    pub track_temporal_anomalies: bool,
    pub alert_thresholds: AlertThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub schema_drift_threshold: f64,
    pub data_drift_threshold: f64,
    pub constraint_violation_threshold: f64,
    pub edge_case_frequency_threshold: f64,
    pub performance_latency_threshold_ms: u64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        AlertThresholds {
            schema_drift_threshold: 0.1,            // 10% deviation
            data_drift_threshold: 0.15,             // 15% deviation
            constraint_violation_threshold: 0.01,   // 1% violations
            edge_case_frequency_threshold: 0.05,    // 5% edge cases
            performance_latency_threshold_ms: 5000, // 5 seconds
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftMetrics {
    pub timestamp: u64,
    pub schema_drift_score: f64,
    pub data_drift_score: f64,
    pub constraint_violations_pct: f64,
    pub edge_case_frequency: f64,
    pub temporal_anomalies: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DriftType {
    SchemaDrift,
    DataDrift,
    ConstraintViolation,
    EdgeCaseAnomaly,
    TemporalAnomaly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftAlert {
    pub drift_type: DriftType,
    pub severity: AlertSeverity,
    pub value: f64,
    pub threshold: f64,
    pub message: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnomalyType {
    MinorAnomaly,
    MajorAnomaly,
    FormatAnomaly,
    OutlierDetection,
    PatternBreak,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetection {
    pub entity_id: String,
    pub anomaly_type: AnomalyType,
    pub severity: AlertSeverity,
    pub affected_field: String,
    pub expected_value: String,
    pub actual_value: String,
    pub confidence_score: f64,
    pub timestamp: u64,
}

pub struct AnomalyDetector {
    anomalies: Vec<AnomalyDetection>,
}

impl Default for AnomalyDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl AnomalyDetector {
    pub fn new() -> Self {
        AnomalyDetector {
            anomalies: Vec::new(),
        }
    }

    pub fn detect_minor_anomaly(
        &mut self,
        entity_id: String,
        field: String,
        expected: String,
        actual: String,
        confidence: f64,
        timestamp: u64,
    ) {
        self.anomalies.push(AnomalyDetection {
            entity_id,
            anomaly_type: AnomalyType::MinorAnomaly,
            severity: AlertSeverity::Info,
            affected_field: field,
            expected_value: expected,
            actual_value: actual,
            confidence_score: confidence,
            timestamp,
        });
    }

    pub fn detect_major_anomaly(
        &mut self,
        entity_id: String,
        field: String,
        expected: String,
        actual: String,
        confidence: f64,
        timestamp: u64,
    ) {
        self.anomalies.push(AnomalyDetection {
            entity_id,
            anomaly_type: AnomalyType::MajorAnomaly,
            severity: AlertSeverity::Critical,
            affected_field: field,
            expected_value: expected,
            actual_value: actual,
            confidence_score: confidence,
            timestamp,
        });
    }

    pub fn detect_format_anomaly(
        &mut self,
        entity_id: String,
        field: String,
        expected_format: String,
        actual_value: String,
        timestamp: u64,
    ) {
        self.anomalies.push(AnomalyDetection {
            entity_id,
            anomaly_type: AnomalyType::FormatAnomaly,
            severity: AlertSeverity::Warning,
            affected_field: field,
            expected_value: expected_format,
            actual_value,
            confidence_score: 0.95,
            timestamp,
        });
    }

    pub fn detect_outlier(
        &mut self,
        entity_id: String,
        field: String,
        expected_range: String,
        actual_value: String,
        confidence: f64,
        timestamp: u64,
    ) {
        self.anomalies.push(AnomalyDetection {
            entity_id,
            anomaly_type: AnomalyType::OutlierDetection,
            severity: if confidence > 0.95 {
                AlertSeverity::Critical
            } else {
                AlertSeverity::Warning
            },
            affected_field: field,
            expected_value: expected_range,
            actual_value,
            confidence_score: confidence,
            timestamp,
        });
    }

    pub fn detect_pattern_break(
        &mut self,
        entity_id: String,
        field: String,
        expected_pattern: String,
        actual_value: String,
        confidence: f64,
        timestamp: u64,
    ) {
        self.anomalies.push(AnomalyDetection {
            entity_id,
            anomaly_type: AnomalyType::PatternBreak,
            severity: AlertSeverity::Warning,
            affected_field: field,
            expected_value: expected_pattern,
            actual_value,
            confidence_score: confidence,
            timestamp,
        });
    }

    pub fn get_anomalies(&self) -> &[AnomalyDetection] {
        &self.anomalies
    }

    pub fn get_anomalies_by_type(&self, anomaly_type: &AnomalyType) -> Vec<&AnomalyDetection> {
        self.anomalies
            .iter()
            .filter(|a| &a.anomaly_type == anomaly_type)
            .collect()
    }

    pub fn get_high_confidence_anomalies(&self, threshold: f64) -> Vec<&AnomalyDetection> {
        self.anomalies
            .iter()
            .filter(|a| a.confidence_score >= threshold)
            .collect()
    }

    pub fn get_anomaly_summary(&self) -> AnomalySummary {
        let mut summary = AnomalySummary::default();

        for anomaly in &self.anomalies {
            summary.total_anomalies += 1;

            match anomaly.severity {
                AlertSeverity::Info => summary.info_count += 1,
                AlertSeverity::Warning => summary.warning_count += 1,
                AlertSeverity::Critical => summary.critical_count += 1,
            }

            match anomaly.anomaly_type {
                AnomalyType::MinorAnomaly => summary.minor_anomalies += 1,
                AnomalyType::MajorAnomaly => summary.major_anomalies += 1,
                AnomalyType::FormatAnomaly => summary.format_anomalies += 1,
                AnomalyType::OutlierDetection => summary.outliers += 1,
                AnomalyType::PatternBreak => summary.pattern_breaks += 1,
            }

            summary.avg_confidence += anomaly.confidence_score;
        }

        if !self.anomalies.is_empty() {
            summary.avg_confidence /= self.anomalies.len() as f64;
        }

        summary
    }

    pub fn clear_anomalies(&mut self) {
        self.anomalies.clear();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnomalySummary {
    pub total_anomalies: usize,
    pub minor_anomalies: usize,
    pub major_anomalies: usize,
    pub format_anomalies: usize,
    pub outliers: usize,
    pub pattern_breaks: usize,
    pub info_count: usize,
    pub warning_count: usize,
    pub critical_count: usize,
    pub avg_confidence: f64,
}

pub struct DriftDetector {
    baseline_metrics: Option<DriftMetrics>,
    alert_history: Vec<DriftAlert>,
}

impl Default for DriftDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl DriftDetector {
    pub fn new() -> Self {
        DriftDetector {
            baseline_metrics: None,
            alert_history: Vec::new(),
        }
    }

    pub fn set_baseline(&mut self, metrics: DriftMetrics) {
        self.baseline_metrics = Some(metrics);
    }

    pub fn detect_drift(
        &mut self,
        current_metrics: DriftMetrics,
        config: &MonitoringConfig,
    ) -> Vec<DriftAlert> {
        let mut alerts = Vec::new();

        if let Some(baseline) = &self.baseline_metrics {
            if config.track_schema_drift {
                if let Some(alert) = self.check_schema_drift(
                    baseline,
                    &current_metrics,
                    config.alert_thresholds.schema_drift_threshold,
                ) {
                    alerts.push(alert);
                }
            }

            if config.track_data_drift {
                if let Some(alert) = self.check_data_drift(
                    baseline,
                    &current_metrics,
                    config.alert_thresholds.data_drift_threshold,
                ) {
                    alerts.push(alert);
                }
            }

            if config.track_constraint_violations {
                if let Some(alert) = self.check_constraint_violations(
                    &current_metrics,
                    config.alert_thresholds.constraint_violation_threshold,
                ) {
                    alerts.push(alert);
                }
            }

            if config.track_edge_cases {
                if let Some(alert) = self.check_edge_case_frequency(
                    &current_metrics,
                    config.alert_thresholds.edge_case_frequency_threshold,
                ) {
                    alerts.push(alert);
                }
            }

            if config.track_temporal_anomalies {
                if let Some(alert) = self.check_temporal_anomalies(&current_metrics) {
                    alerts.push(alert);
                }
            }
        }

        for alert in &alerts {
            self.alert_history.push(alert.clone());
        }

        alerts
    }

    fn check_schema_drift(
        &self,
        baseline: &DriftMetrics,
        current: &DriftMetrics,
        threshold: f64,
    ) -> Option<DriftAlert> {
        let drift = (current.schema_drift_score - baseline.schema_drift_score).abs();

        if drift > threshold {
            Some(DriftAlert {
                drift_type: DriftType::SchemaDrift,
                severity: if drift > threshold * 2.0 {
                    AlertSeverity::Critical
                } else {
                    AlertSeverity::Warning
                },
                value: drift,
                threshold,
                message: format!("Schema drift detected: {:.2}% change", drift * 100.0),
                timestamp: current.timestamp,
            })
        } else {
            None
        }
    }

    fn check_data_drift(
        &self,
        baseline: &DriftMetrics,
        current: &DriftMetrics,
        threshold: f64,
    ) -> Option<DriftAlert> {
        let drift = (current.data_drift_score - baseline.data_drift_score).abs();

        if drift > threshold {
            Some(DriftAlert {
                drift_type: DriftType::DataDrift,
                severity: if drift > threshold * 2.0 {
                    AlertSeverity::Critical
                } else {
                    AlertSeverity::Warning
                },
                value: drift,
                threshold,
                message: format!("Data drift detected: {:.2}% change", drift * 100.0),
                timestamp: current.timestamp,
            })
        } else {
            None
        }
    }

    fn check_constraint_violations(
        &self,
        current: &DriftMetrics,
        threshold: f64,
    ) -> Option<DriftAlert> {
        if current.constraint_violations_pct > threshold {
            Some(DriftAlert {
                drift_type: DriftType::ConstraintViolation,
                severity: if current.constraint_violations_pct > threshold * 3.0 {
                    AlertSeverity::Critical
                } else {
                    AlertSeverity::Warning
                },
                value: current.constraint_violations_pct,
                threshold,
                message: format!(
                    "Constraint violations: {:.2}%",
                    current.constraint_violations_pct * 100.0
                ),
                timestamp: current.timestamp,
            })
        } else {
            None
        }
    }

    fn check_edge_case_frequency(
        &self,
        current: &DriftMetrics,
        threshold: f64,
    ) -> Option<DriftAlert> {
        if current.edge_case_frequency > threshold {
            Some(DriftAlert {
                drift_type: DriftType::EdgeCaseAnomaly,
                severity: if current.edge_case_frequency > threshold * 2.0 {
                    AlertSeverity::Critical
                } else {
                    AlertSeverity::Warning
                },
                value: current.edge_case_frequency,
                threshold,
                message: format!(
                    "High edge case frequency: {:.2}%",
                    current.edge_case_frequency * 100.0
                ),
                timestamp: current.timestamp,
            })
        } else {
            None
        }
    }

    fn check_temporal_anomalies(&self, current: &DriftMetrics) -> Option<DriftAlert> {
        if current.temporal_anomalies > 10 {
            Some(DriftAlert {
                drift_type: DriftType::TemporalAnomaly,
                severity: if current.temporal_anomalies > 50 {
                    AlertSeverity::Critical
                } else {
                    AlertSeverity::Warning
                },
                value: current.temporal_anomalies as f64,
                threshold: 10.0,
                message: format!(
                    "Temporal anomalies detected: {} instances",
                    current.temporal_anomalies
                ),
                timestamp: current.timestamp,
            })
        } else {
            None
        }
    }

    pub fn get_alerts(&self) -> &[DriftAlert] {
        &self.alert_history
    }

    pub fn clear_alerts(&mut self) {
        self.alert_history.clear();
    }

    pub fn get_alert_summary(&self) -> AlertSummary {
        let mut summary = AlertSummary::default();

        for alert in &self.alert_history {
            summary.total_alerts += 1;
            match alert.severity {
                AlertSeverity::Info => summary.info_count += 1,
                AlertSeverity::Warning => summary.warning_count += 1,
                AlertSeverity::Critical => summary.critical_count += 1,
            }

            match alert.drift_type {
                DriftType::SchemaDrift => summary.schema_drifts += 1,
                DriftType::DataDrift => summary.data_drifts += 1,
                DriftType::ConstraintViolation => summary.constraint_violations += 1,
                DriftType::EdgeCaseAnomaly => summary.edge_case_anomalies += 1,
                DriftType::TemporalAnomaly => summary.temporal_anomalies += 1,
            }
        }

        summary
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertSummary {
    pub total_alerts: usize,
    pub info_count: usize,
    pub warning_count: usize,
    pub critical_count: usize,
    pub schema_drifts: usize,
    pub data_drifts: usize,
    pub constraint_violations: usize,
    pub edge_case_anomalies: usize,
    pub temporal_anomalies: usize,
}

pub struct PerformanceMonitor {
    metrics: Vec<PerformanceMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    pub operation: String,
    pub duration_ms: u64,
    pub timestamp: u64,
    pub records_processed: usize,
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        PerformanceMonitor {
            metrics: Vec::new(),
        }
    }

    pub fn record_operation(
        &mut self,
        operation: String,
        duration_ms: u64,
        records_processed: usize,
        timestamp: u64,
    ) {
        self.metrics.push(PerformanceMetric {
            operation,
            duration_ms,
            timestamp,
            records_processed,
        });
    }

    pub fn get_average_latency(&self, operation: &str) -> Option<f64> {
        let matching: Vec<_> = self
            .metrics
            .iter()
            .filter(|m| m.operation == operation)
            .collect();

        if matching.is_empty() {
            return None;
        }

        let total: u64 = matching.iter().map(|m| m.duration_ms).sum();
        Some(total as f64 / matching.len() as f64)
    }

    pub fn get_throughput(&self, operation: &str) -> Option<f64> {
        let matching: Vec<_> = self
            .metrics
            .iter()
            .filter(|m| m.operation == operation)
            .collect();

        if matching.is_empty() {
            return None;
        }

        let total_records: usize = matching.iter().map(|m| m.records_processed).sum();
        let total_ms: u64 = matching.iter().map(|m| m.duration_ms).sum();

        if total_ms == 0 {
            return None;
        }

        Some(total_records as f64 / (total_ms as f64 / 1000.0))
    }

    pub fn get_slowest(&self) -> Option<&PerformanceMetric> {
        self.metrics.iter().max_by_key(|m| m.duration_ms)
    }

    pub fn get_fastest(&self) -> Option<&PerformanceMetric> {
        self.metrics.iter().min_by_key(|m| m.duration_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitoring_config_defaults() {
        let config = MonitoringConfig::default();
        assert!(!config.track_schema_drift);
        assert!(!config.track_data_drift);
        assert_eq!(config.alert_thresholds.schema_drift_threshold, 0.1);
    }

    #[test]
    fn test_enable_monitoring() {
        let config = MonitoringConfig {
            track_schema_drift: true,
            track_data_drift: true,
            ..Default::default()
        };

        assert!(config.track_schema_drift);
        assert!(config.track_data_drift);
    }

    #[test]
    fn test_drift_detection_schema() {
        let mut detector = DriftDetector::new();

        let baseline = DriftMetrics {
            timestamp: 1000,
            schema_drift_score: 0.0,
            data_drift_score: 0.0,
            constraint_violations_pct: 0.0,
            edge_case_frequency: 0.0,
            temporal_anomalies: 0,
        };

        detector.set_baseline(baseline);

        let current = DriftMetrics {
            timestamp: 2000,
            schema_drift_score: 0.15, // 15% drift (above 10% threshold)
            data_drift_score: 0.0,
            constraint_violations_pct: 0.0,
            edge_case_frequency: 0.0,
            temporal_anomalies: 0,
        };

        let config = MonitoringConfig {
            track_schema_drift: true,
            ..Default::default()
        };

        let alerts = detector.detect_drift(current, &config);
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].drift_type, DriftType::SchemaDrift);
    }

    #[test]
    fn test_alert_summary() {
        let mut detector = DriftDetector::new();

        let baseline = DriftMetrics {
            timestamp: 1000,
            schema_drift_score: 0.0,
            data_drift_score: 0.0,
            constraint_violations_pct: 0.0,
            edge_case_frequency: 0.0,
            temporal_anomalies: 0,
        };

        detector.set_baseline(baseline);

        let current = DriftMetrics {
            timestamp: 2000,
            schema_drift_score: 0.15,
            data_drift_score: 0.02,
            constraint_violations_pct: 0.0,
            edge_case_frequency: 0.0,
            temporal_anomalies: 0,
        };

        let config = MonitoringConfig {
            track_schema_drift: true,
            ..Default::default()
        };

        detector.detect_drift(current, &config);
        let summary = detector.get_alert_summary();

        assert_eq!(summary.total_alerts, 1);
        assert_eq!(summary.critical_count, 0);
    }

    #[test]
    fn test_performance_monitor() {
        let mut monitor = PerformanceMonitor::new();

        monitor.record_operation("generate_schema".to_string(), 100, 1000, 1000);
        monitor.record_operation("generate_schema".to_string(), 150, 1000, 2000);
        monitor.record_operation("generate_schema".to_string(), 200, 1000, 3000);

        let avg_latency = monitor.get_average_latency("generate_schema");
        assert!(avg_latency.is_some());
        assert_eq!(avg_latency.unwrap(), 150.0);

        let throughput = monitor.get_throughput("generate_schema");
        assert!(throughput.is_some());
    }

    #[test]
    fn test_performance_extremes() {
        let mut monitor = PerformanceMonitor::new();

        monitor.record_operation("op1".to_string(), 50, 100, 1000);
        monitor.record_operation("op1".to_string(), 200, 100, 2000);
        monitor.record_operation("op1".to_string(), 100, 100, 3000);

        assert_eq!(monitor.get_slowest().unwrap().duration_ms, 200);
        assert_eq!(monitor.get_fastest().unwrap().duration_ms, 50);
    }
}
