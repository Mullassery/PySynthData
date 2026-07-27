use pysynthdata::monitoring::*;

#[test]
fn test_monitoring_config_defaults() {
    let config = MonitoringConfig::default();
    assert!(!config.track_schema_drift);
    assert!(!config.track_data_drift);
    assert!(!config.track_constraint_violations);
    assert!(!config.track_edge_cases);
}

#[test]
fn test_enable_all_monitoring() {
    let mut config = MonitoringConfig::default();
    config.track_schema_drift = true;
    config.track_data_drift = true;
    config.track_constraint_violations = true;
    config.track_edge_cases = true;
    config.track_performance_metrics = true;
    config.track_temporal_anomalies = true;

    assert!(config.track_schema_drift);
    assert!(config.track_data_drift);
    assert!(config.track_constraint_violations);
    assert!(config.track_edge_cases);
    assert!(config.track_performance_metrics);
    assert!(config.track_temporal_anomalies);
}

#[test]
fn test_custom_thresholds() {
    let mut config = MonitoringConfig::default();
    config.alert_thresholds.schema_drift_threshold = 0.05;
    config.alert_thresholds.data_drift_threshold = 0.1;
    config.alert_thresholds.constraint_violation_threshold = 0.02;

    assert_eq!(config.alert_thresholds.schema_drift_threshold, 0.05);
    assert_eq!(config.alert_thresholds.data_drift_threshold, 0.1);
}

#[test]
fn test_drift_detector_init() {
    let detector = DriftDetector::new();
    assert_eq!(detector.get_alerts().len(), 0);
}

#[test]
fn test_schema_drift_detection() {
    let mut detector = DriftDetector::new();

    let baseline = DriftMetrics {
        timestamp: 1000,
        schema_drift_score: 0.05,
        data_drift_score: 0.02,
        constraint_violations_pct: 0.005,
        edge_case_frequency: 0.02,
        temporal_anomalies: 2,
    };

    detector.set_baseline(baseline);

    let current = DriftMetrics {
        timestamp: 2000,
        schema_drift_score: 0.20, // 15% increase (exceeds 10% threshold)
        data_drift_score: 0.02,
        constraint_violations_pct: 0.005,
        edge_case_frequency: 0.02,
        temporal_anomalies: 2,
    };

    let mut config = MonitoringConfig::default();
    config.track_schema_drift = true;

    let alerts = detector.detect_drift(current, &config);
    assert_eq!(alerts.len(), 1);
    assert_eq!(alerts[0].drift_type, DriftType::SchemaDrift);
    assert!(matches!(alerts[0].severity, AlertSeverity::Warning | AlertSeverity::Critical));
}

#[test]
fn test_data_drift_detection() {
    let mut detector = DriftDetector::new();

    let baseline = DriftMetrics {
        timestamp: 1000,
        schema_drift_score: 0.05,
        data_drift_score: 0.10,
        constraint_violations_pct: 0.005,
        edge_case_frequency: 0.02,
        temporal_anomalies: 2,
    };

    detector.set_baseline(baseline);

    let current = DriftMetrics {
        timestamp: 2000,
        schema_drift_score: 0.05,
        data_drift_score: 0.30, // 20% increase (exceeds 15% threshold)
        constraint_violations_pct: 0.005,
        edge_case_frequency: 0.02,
        temporal_anomalies: 2,
    };

    let mut config = MonitoringConfig::default();
    config.track_data_drift = true;

    let alerts = detector.detect_drift(current, &config);
    assert_eq!(alerts.len(), 1);
    assert_eq!(alerts[0].drift_type, DriftType::DataDrift);
}

#[test]
fn test_constraint_violation_alert() {
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
        schema_drift_score: 0.0,
        data_drift_score: 0.0,
        constraint_violations_pct: 0.05, // 5% violations (exceeds 1% threshold)
        edge_case_frequency: 0.0,
        temporal_anomalies: 0,
    };

    let mut config = MonitoringConfig::default();
    config.track_constraint_violations = true;

    let alerts = detector.detect_drift(current, &config);
    assert_eq!(alerts.len(), 1);
    assert_eq!(alerts[0].drift_type, DriftType::ConstraintViolation);
}

#[test]
fn test_edge_case_frequency_alert() {
    let mut detector = DriftDetector::new();

    let baseline = DriftMetrics {
        timestamp: 1000,
        schema_drift_score: 0.0,
        data_drift_score: 0.0,
        constraint_violations_pct: 0.0,
        edge_case_frequency: 0.01,
        temporal_anomalies: 0,
    };

    detector.set_baseline(baseline);

    let current = DriftMetrics {
        timestamp: 2000,
        schema_drift_score: 0.0,
        data_drift_score: 0.0,
        constraint_violations_pct: 0.0,
        edge_case_frequency: 0.10, // 10% edge cases (exceeds 5% threshold)
        temporal_anomalies: 0,
    };

    let mut config = MonitoringConfig::default();
    config.track_edge_cases = true;

    let alerts = detector.detect_drift(current, &config);
    assert_eq!(alerts.len(), 1);
    assert_eq!(alerts[0].drift_type, DriftType::EdgeCaseAnomaly);
}

#[test]
fn test_multiple_drift_alerts() {
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
        schema_drift_score: 0.15, // Exceeds schema drift threshold
        data_drift_score: 0.20,   // Exceeds data drift threshold
        constraint_violations_pct: 0.05, // Exceeds constraint violation threshold
        edge_case_frequency: 0.10, // Exceeds edge case threshold
        temporal_anomalies: 0,
    };

    let mut config = MonitoringConfig::default();
    config.track_schema_drift = true;
    config.track_data_drift = true;
    config.track_constraint_violations = true;
    config.track_edge_cases = true;

    let alerts = detector.detect_drift(current, &config);
    assert_eq!(alerts.len(), 4);
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
        data_drift_score: 0.20,
        constraint_violations_pct: 0.05,
        edge_case_frequency: 0.10,
        temporal_anomalies: 0,
    };

    let mut config = MonitoringConfig::default();
    config.track_schema_drift = true;
    config.track_data_drift = true;
    config.track_constraint_violations = true;
    config.track_edge_cases = true;

    detector.detect_drift(current, &config);
    let summary = detector.get_alert_summary();

    assert_eq!(summary.total_alerts, 4);
    assert_eq!(summary.schema_drifts, 1);
    assert_eq!(summary.data_drifts, 1);
    assert_eq!(summary.constraint_violations, 1);
    assert_eq!(summary.edge_case_anomalies, 1);
}

#[test]
fn test_performance_monitor_latency() {
    let mut monitor = PerformanceMonitor::new();

    monitor.record_operation("generate".to_string(), 100, 1000, 1000);
    monitor.record_operation("generate".to_string(), 200, 1000, 2000);
    monitor.record_operation("generate".to_string(), 300, 1000, 3000);

    let avg = monitor.get_average_latency("generate");
    assert!(avg.is_some());
    assert_eq!(avg.unwrap(), 200.0);
}

#[test]
fn test_performance_monitor_throughput() {
    let mut monitor = PerformanceMonitor::new();

    monitor.record_operation("process".to_string(), 1000, 1000, 1000); // 1000 records in 1 second
    monitor.record_operation("process".to_string(), 1000, 1000, 2000); // 1000 records in 1 second

    let throughput = monitor.get_throughput("process");
    assert!(throughput.is_some());
    assert!(throughput.unwrap() > 0.0);
}

#[test]
fn test_performance_extremes() {
    let mut monitor = PerformanceMonitor::new();

    monitor.record_operation("task".to_string(), 50, 100, 1000);
    monitor.record_operation("task".to_string(), 500, 100, 2000);
    monitor.record_operation("task".to_string(), 200, 100, 3000);

    let slowest = monitor.get_slowest();
    let fastest = monitor.get_fastest();

    assert!(slowest.is_some());
    assert!(fastest.is_some());
    assert_eq!(slowest.unwrap().duration_ms, 500);
    assert_eq!(fastest.unwrap().duration_ms, 50);
}

#[test]
fn test_minor_anomaly_detection() {
    let mut detector = AnomalyDetector::new();

    detector.detect_minor_anomaly(
        "entity_1".to_string(),
        "field_1".to_string(),
        "expected_value".to_string(),
        "actual_value".to_string(),
        0.75,
        1000,
    );

    assert_eq!(detector.get_anomalies().len(), 1);
    let anomaly = &detector.get_anomalies()[0];
    assert_eq!(anomaly.anomaly_type, AnomalyType::MinorAnomaly);
    assert_eq!(anomaly.severity, AlertSeverity::Info);
}

#[test]
fn test_major_anomaly_detection() {
    let mut detector = AnomalyDetector::new();

    detector.detect_major_anomaly(
        "entity_1".to_string(),
        "field_critical".to_string(),
        "expected".to_string(),
        "completely_different".to_string(),
        0.95,
        1000,
    );

    assert_eq!(detector.get_anomalies().len(), 1);
    let anomaly = &detector.get_anomalies()[0];
    assert_eq!(anomaly.anomaly_type, AnomalyType::MajorAnomaly);
    assert_eq!(anomaly.severity, AlertSeverity::Critical);
}

#[test]
fn test_format_anomaly_detection() {
    let mut detector = AnomalyDetector::new();

    detector.detect_format_anomaly(
        "entity_1".to_string(),
        "email".to_string(),
        "email@example.com format".to_string(),
        "not_an_email".to_string(),
        1000,
    );

    assert_eq!(detector.get_anomalies().len(), 1);
    let anomaly = &detector.get_anomalies()[0];
    assert_eq!(anomaly.anomaly_type, AnomalyType::FormatAnomaly);
    assert_eq!(anomaly.severity, AlertSeverity::Warning);
}

#[test]
fn test_outlier_detection() {
    let mut detector = AnomalyDetector::new();

    detector.detect_outlier(
        "entity_1".to_string(),
        "age".to_string(),
        "18-65".to_string(),
        "150".to_string(),
        0.98,
        1000,
    );

    assert_eq!(detector.get_anomalies().len(), 1);
    let anomaly = &detector.get_anomalies()[0];
    assert_eq!(anomaly.anomaly_type, AnomalyType::OutlierDetection);
    assert_eq!(anomaly.severity, AlertSeverity::Critical); // High confidence → critical
}

#[test]
fn test_pattern_break_detection() {
    let mut detector = AnomalyDetector::new();

    detector.detect_pattern_break(
        "entity_1".to_string(),
        "status_sequence".to_string(),
        "active -> inactive -> closed".to_string(),
        "active -> closed".to_string(),
        0.87,
        1000,
    );

    assert_eq!(detector.get_anomalies().len(), 1);
    let anomaly = &detector.get_anomalies()[0];
    assert_eq!(anomaly.anomaly_type, AnomalyType::PatternBreak);
    assert_eq!(anomaly.severity, AlertSeverity::Warning);
}

#[test]
fn test_anomalies_by_type() {
    let mut detector = AnomalyDetector::new();

    detector.detect_minor_anomaly(
        "entity_1".to_string(),
        "field1".to_string(),
        "exp".to_string(),
        "act".to_string(),
        0.7,
        1000,
    );
    detector.detect_major_anomaly(
        "entity_2".to_string(),
        "field2".to_string(),
        "exp".to_string(),
        "act".to_string(),
        0.9,
        2000,
    );
    detector.detect_format_anomaly(
        "entity_3".to_string(),
        "field3".to_string(),
        "format".to_string(),
        "bad_format".to_string(),
        3000,
    );

    let minors = detector.get_anomalies_by_type(&AnomalyType::MinorAnomaly);
    let majors = detector.get_anomalies_by_type(&AnomalyType::MajorAnomaly);
    let formats = detector.get_anomalies_by_type(&AnomalyType::FormatAnomaly);

    assert_eq!(minors.len(), 1);
    assert_eq!(majors.len(), 1);
    assert_eq!(formats.len(), 1);
}

#[test]
fn test_high_confidence_anomalies() {
    let mut detector = AnomalyDetector::new();

    detector.detect_minor_anomaly("e1".to_string(), "f1".to_string(), "e".to_string(), "a".to_string(), 0.6, 1000);
    detector.detect_minor_anomaly("e2".to_string(), "f2".to_string(), "e".to_string(), "a".to_string(), 0.95, 2000);
    detector.detect_minor_anomaly("e3".to_string(), "f3".to_string(), "e".to_string(), "a".to_string(), 0.88, 3000);

    let high_conf = detector.get_high_confidence_anomalies(0.90);
    assert_eq!(high_conf.len(), 1);
    assert_eq!(high_conf[0].confidence_score, 0.95);
}

#[test]
fn test_anomaly_summary() {
    let mut detector = AnomalyDetector::new();

    detector.detect_minor_anomaly("e1".to_string(), "f1".to_string(), "e".to_string(), "a".to_string(), 0.8, 1000);
    detector.detect_major_anomaly("e2".to_string(), "f2".to_string(), "e".to_string(), "a".to_string(), 0.9, 2000);
    detector.detect_format_anomaly("e3".to_string(), "f3".to_string(), "fmt".to_string(), "bad".to_string(), 3000);
    detector.detect_outlier("e4".to_string(), "f4".to_string(), "range".to_string(), "outlier".to_string(), 0.85, 4000);

    let summary = detector.get_anomaly_summary();

    assert_eq!(summary.total_anomalies, 4);
    assert_eq!(summary.minor_anomalies, 1);
    assert_eq!(summary.major_anomalies, 1);
    assert_eq!(summary.format_anomalies, 1);
    assert_eq!(summary.outliers, 1);
    assert_eq!(summary.critical_count, 1); // Major anomaly
    assert_eq!(summary.warning_count, 2);  // Format + Outlier
    assert_eq!(summary.info_count, 1);     // Minor
    assert!(summary.avg_confidence > 0.8);
}

#[test]
fn test_clear_anomalies() {
    let mut detector = AnomalyDetector::new();

    detector.detect_minor_anomaly("e1".to_string(), "f1".to_string(), "e".to_string(), "a".to_string(), 0.8, 1000);
    assert_eq!(detector.get_anomalies().len(), 1);

    detector.clear_anomalies();
    assert_eq!(detector.get_anomalies().len(), 0);
}
