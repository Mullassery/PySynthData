use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityConfig {
    pub missing_value_rate: f64,           // 0.0 - 0.3 (0-30%)
    pub duplicate_rate: f64,               // 0.0 - 0.1 (0-10%)
    pub outlier_rate: f64,                 // 0.0 - 0.05 (0-5%)
    pub typo_rate: f64,                    // 0.0 - 0.05 (0-5%)
    pub inconsistency_rate: f64,           // 0.0 - 0.1 (0-10%)
    pub temporal_inconsistency_rate: f64,  // 0.0 - 0.05 (0-5%)
    pub null_clustering: bool,             // Nulls cluster together (realistic)
    pub field_specific_issues: HashMap<String, FieldQualityIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldQualityIssue {
    pub field_name: String,
    pub missing_rate: f64,
    pub outlier_pattern: Option<OutlierPattern>,
    pub typo_chars: Option<Vec<char>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutlierPattern {
    Extreme,         // Very large/small values
    Impossible,      // Values that violate domain logic
    Sparse,          // Rare but valid values
    Clustered,       // Groups of outliers (e.g., fraud patterns)
}

impl Default for DataQualityConfig {
    fn default() -> Self {
        DataQualityConfig {
            missing_value_rate: 0.02,              // 2% missing
            duplicate_rate: 0.005,                 // 0.5% duplicates
            outlier_rate: 0.03,                    // 3% outliers
            typo_rate: 0.01,                       // 1% typos
            inconsistency_rate: 0.02,              // 2% inconsistencies
            temporal_inconsistency_rate: 0.01,     // 1% temporal issues
            null_clustering: true,                 // Nulls cluster
            field_specific_issues: HashMap::new(),
        }
    }
}

pub struct DataQualityDegradation {
    rng: rand::rngs::StdRng,
}

impl DataQualityDegradation {
    pub fn new(seed: u64) -> Self {
        use rand::SeedableRng;
        DataQualityDegradation {
            rng: rand::rngs::StdRng::seed_from_u64(seed),
        }
    }

    pub fn inject_missing_values(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        rate: f64,
    ) {
        if data.is_empty() {
            return;
        }

        let rows = data.len();
        let missing_count = (rows as f64 * rate) as usize;

        for _ in 0..missing_count {
            let row_idx = self.rng.gen_range(0..rows);
            let row = &mut data[row_idx];

            if !row.is_empty() {
                let field_idx = self.rng.gen_range(0..row.len());
                let field_name = row.keys().nth(field_idx).unwrap().clone();
                row.insert(field_name, "NULL".to_string());
            }
        }
    }

    pub fn inject_duplicates(&mut self, data: &mut Vec<HashMap<String, String>>, rate: f64) {
        if data.is_empty() {
            return;
        }

        let rows = data.len();
        let dup_count = (rows as f64 * rate) as usize;

        for _ in 0..dup_count {
            let src_idx = self.rng.gen_range(0..rows);
            let src_row = data[src_idx].clone();
            data.push(src_row);
        }
    }

    pub fn inject_outliers(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        field_name: &str,
        rate: f64,
        pattern: &OutlierPattern,
    ) {
        if data.is_empty() {
            return;
        }

        let rows = data.len();
        let outlier_count = (rows as f64 * rate) as usize;

        for _ in 0..outlier_count {
            let row_idx = self.rng.gen_range(0..rows);
            let outlier_value = match pattern {
                OutlierPattern::Extreme => {
                    if self.rng.gen_bool(0.5) {
                        "999999.99".to_string()  // Extreme high
                    } else {
                        "-999999.99".to_string() // Extreme low
                    }
                }
                OutlierPattern::Impossible => "INVALID_VALUE".to_string(),
                OutlierPattern::Sparse => format!("RARE_{}", self.rng.gen_range(0..1000)),
                OutlierPattern::Clustered => {
                    if self.rng.gen_bool(0.3) {
                        "FRAUD_PATTERN".to_string()
                    } else {
                        "ANOMALY".to_string()
                    }
                }
            };

            data[row_idx].insert(field_name.to_string(), outlier_value);
        }
    }

    pub fn inject_typos(&mut self, data: &mut Vec<HashMap<String, String>>, field_name: &str, rate: f64) {
        if data.is_empty() {
            return;
        }

        let rows = data.len();
        let typo_count = (rows as f64 * rate) as usize;
        let common_chars = vec!['a', 'e', 'i', 'o', 'u', '0', '1', '2', '3', '4', '5'];

        for _ in 0..typo_count {
            let row_idx = self.rng.gen_range(0..rows);

            if let Some(value) = data[row_idx].get_mut(field_name) {
                if !value.is_empty() && value != "NULL" {
                    let char_idx = self.rng.gen_range(0..value.len());
                    let typo_char = common_chars[self.rng.gen_range(0..common_chars.len())];

                    let mut chars: Vec<char> = value.chars().collect();
                    chars[char_idx] = typo_char;
                    *value = chars.into_iter().collect();
                }
            }
        }
    }

    pub fn inject_inconsistencies(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        related_fields: (&str, &str),
        rate: f64,
    ) {
        if data.is_empty() {
            return;
        }

        let rows = data.len();
        let inconsistency_count = (rows as f64 * rate) as usize;

        for _ in 0..inconsistency_count {
            let row_idx = self.rng.gen_range(0..rows);
            let row = &mut data[row_idx];

            // Swap or modify related fields to create logical inconsistency
            if let (Some(val1), Some(val2)) = (row.get(related_fields.0).cloned(), row.get(related_fields.1).cloned()) {
                row.insert(related_fields.0.to_string(), val2);
                row.insert(related_fields.1.to_string(), val1);
            }
        }
    }

    pub fn inject_temporal_inconsistencies(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        timestamp_field: &str,
        rate: f64,
    ) {
        if data.is_empty() {
            return;
        }

        let rows = data.len();
        let temporal_issue_count = (rows as f64 * rate) as usize;

        for _ in 0..temporal_issue_count {
            let row_idx = self.rng.gen_range(0..rows);

            if let Some(timestamp) = data[row_idx].get_mut(timestamp_field) {
                if !timestamp.is_empty() && timestamp != "NULL" {
                    // Add random days (can be in past or future)
                    let offset_days = self.rng.gen_range(-365..365);
                    *timestamp = format!("{}_OFFSET_{}days", timestamp, offset_days);
                }
            }
        }
    }

    pub fn apply_quality_degradation(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        config: &DataQualityConfig,
    ) {
        if config.missing_value_rate > 0.0 {
            self.inject_missing_values(data, config.missing_value_rate);
        }

        if config.duplicate_rate > 0.0 {
            self.inject_duplicates(data, config.duplicate_rate);
        }

        if config.outlier_rate > 0.0 {
            for (field, issue) in &config.field_specific_issues {
                if let Some(pattern) = &issue.outlier_pattern {
                    self.inject_outliers(data, field, issue.missing_rate, pattern);
                }
            }
            // Generic outlier injection for unspecified fields
            if !config.field_specific_issues.is_empty() {
                self.inject_outliers(data, "amount", config.outlier_rate, &OutlierPattern::Extreme);
            }
        }

        if config.typo_rate > 0.0 {
            for field in config.field_specific_issues.keys() {
                self.inject_typos(data, field, config.typo_rate);
            }
        }

        if config.inconsistency_rate > 0.0 {
            // Inject cross-field inconsistencies
            self.inject_inconsistencies(data, ("start_date", "end_date"), config.inconsistency_rate);
            self.inject_inconsistencies(data, ("status", "balance"), config.inconsistency_rate);
        }

        if config.temporal_inconsistency_rate > 0.0 {
            self.inject_temporal_inconsistencies(data, "created_at", config.temporal_inconsistency_rate);
            self.inject_temporal_inconsistencies(data, "timestamp", config.temporal_inconsistency_rate);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityMetrics {
    pub total_records: usize,
    pub missing_values: usize,
    pub duplicate_records: usize,
    pub outlier_records: usize,
    pub inconsistent_records: usize,
    pub temporal_issues: usize,
    pub overall_quality_score: f64,
}

pub struct DataQualityAnalyzer;

impl DataQualityAnalyzer {
    pub fn analyze(data: &[HashMap<String, String>]) -> DataQualityMetrics {
        let total = data.len();
        let mut missing_count = 0;
        let mut outlier_count = 0;
        let mut inconsistent_count = 0;
        let mut temporal_issue_count = 0;

        for row in data {
            for (_, value) in row.iter() {
                if value == "NULL" {
                    missing_count += 1;
                }
                if value.contains("OFFSET") {
                    temporal_issue_count += 1;
                }
                if value.contains("INVALID") || value.contains("ANOMALY") || value.contains("FRAUD") {
                    outlier_count += 1;
                }
            }
        }

        let duplicate_count = 0; // Would need additional logic to track

        let quality_score = if total == 0 {
            1.0
        } else {
            let defects = missing_count + outlier_count + inconsistent_count + temporal_issue_count;
            1.0 - (defects as f64 / (total as f64 * 5.0)).min(1.0)
        };

        DataQualityMetrics {
            total_records: total,
            missing_values: missing_count,
            duplicate_records: duplicate_count,
            outlier_records: outlier_count,
            inconsistent_records: inconsistent_count,
            temporal_issues: temporal_issue_count,
            overall_quality_score: quality_score,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_config_defaults() {
        let config = DataQualityConfig::default();
        assert_eq!(config.missing_value_rate, 0.02);
        assert_eq!(config.duplicate_rate, 0.005);
        assert!(config.null_clustering);
    }

    #[test]
    fn test_inject_missing_values() {
        let mut degrader = DataQualityDegradation::new(42);
        let mut data = vec![
            [("name".to_string(), "John".to_string())].iter().cloned().collect(),
            [("name".to_string(), "Jane".to_string())].iter().cloned().collect(),
        ];

        degrader.inject_missing_values(&mut data, 0.5);
        let null_count = data.iter().filter(|r| r.get("name").map_or(false, |v| v == "NULL")).count();
        assert!(null_count > 0);
    }

    #[test]
    fn test_inject_duplicates() {
        let mut degrader = DataQualityDegradation::new(42);
        let original_len = 10;
        let mut data: Vec<HashMap<String, String>> = (0..original_len)
            .map(|i| [("id".to_string(), i.to_string())].iter().cloned().collect())
            .collect();

        degrader.inject_duplicates(&mut data, 0.2);
        assert!(data.len() > original_len);
    }

    #[test]
    fn test_inject_outliers_extreme() {
        let mut degrader = DataQualityDegradation::new(42);
        let mut data = vec![
            [("amount".to_string(), "100".to_string())].iter().cloned().collect(),
        ];

        degrader.inject_outliers(&mut data, "amount", 1.0, &OutlierPattern::Extreme);
        let value = data[0].get("amount").unwrap();
        assert!(value.contains("999999") || value.contains("-999999"));
    }

    #[test]
    fn test_data_quality_analyzer() {
        let mut data = vec![
            [("id".to_string(), "1".to_string()), ("status".to_string(), "active".to_string())]
                .iter()
                .cloned()
                .collect(),
            [("id".to_string(), "2".to_string()), ("status".to_string(), "NULL".to_string())]
                .iter()
                .cloned()
                .collect(),
        ];

        let metrics = DataQualityAnalyzer::analyze(&data);
        assert_eq!(metrics.total_records, 2);
        assert!(metrics.missing_values > 0);
        assert!(metrics.overall_quality_score < 1.0);
    }
}
