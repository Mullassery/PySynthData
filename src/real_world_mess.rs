use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealWorldMessConfig {
    // Legacy system cruft
    pub legacy_system_mixed_ids: bool,
    pub inconsistent_null_representations: bool,      // NULL, null, N/A, "", -1, 0, "unknown"
    pub duplicate_column_names_with_typos: bool,

    // Human data entry errors
    pub fat_finger_errors: bool,                       // 1 → l, 0 → O
    pub copy_paste_truncation: bool,                   // Text gets cut off randomly
    pub field_swaps: bool,                             // Values in wrong columns
    pub unit_confusion: bool,                          // kg vs lbs, C vs F

    // System integration failures
    pub encoding_mismatches: bool,                     // UTF-8, Latin-1, ASCII mixed
    pub character_corruption: bool,                    // Special chars become garbage
    pub decimal_separator_chaos: bool,                 // 100.5 vs 100,5 vs 1005 (missing decimal)

    // Temporal mess
    pub timezone_chaos: bool,                          // Mixed UTC, local, no timezone
    pub leap_second_errors: bool,
    pub y2k_style_bugs: bool,                          // Year 00, 99, wrapped dates
    pub daylight_savings_doubles: bool,                // 2:30 AM happens twice

    // Batch processing disasters
    pub partial_load_incompleteness: bool,             // Some fields missing in later batches
    pub schema_evolution_incompleteness: bool,         // Old/new schema versions mixed
    pub rollback_artifacts: bool,                      // Partially reverted transactions
    pub retried_duplicates_with_timestamp_shift: bool,

    // Sensor/integration drift
    pub sensor_calibration_drift: bool,                // Values gradually shift
    pub api_rate_limit_truncation: bool,               // Data cut off mid-request
    pub timeout_partial_writes: bool,                  // Some fields written, others not
    pub cache_stale_writes: bool,                      // Old data written over new

    // Migration artifacts
    pub null_from_migration: bool,                     // NULLs where data should exist
    pub duplicate_after_replication: bool,
    pub referential_integrity_broken: bool,            // Foreign keys don't exist

    // Operator errors
    pub accidental_regex_replacements: bool,           // Overly broad find/replace
    pub manual_correction_inconsistencies: bool,       // Corrections made inconsistently
    pub batch_delete_mistakes: bool,                   // Data deleted that shouldn't be
    pub manual_sql_update_typos: bool,

    // Business logic errors in code
    pub rounding_accumulation_errors: bool,            // Rounding errors compound
    pub floating_point_precision_loss: bool,
    pub type_conversion_edge_cases: bool,              // "123abc" as int = 123 or error?
    pub default_value_spill: bool,                     // 0 or "N/A" as actual data vs default

    // Compression of multiple issues
    pub simultaneous_multi_layer_mess: bool,           // Multiple issues at once
    pub cascading_error_propagation: bool,             // One error causes many
    pub error_correction_creates_new_errors: bool,     // Fix attempt makes it worse
}

impl Default for RealWorldMessConfig {
    fn default() -> Self {
        RealWorldMessConfig {
            legacy_system_mixed_ids: true,
            inconsistent_null_representations: true,
            duplicate_column_names_with_typos: true,
            fat_finger_errors: true,
            copy_paste_truncation: true,
            field_swaps: true,
            unit_confusion: true,
            encoding_mismatches: true,
            character_corruption: true,
            decimal_separator_chaos: true,
            timezone_chaos: true,
            leap_second_errors: true,
            y2k_style_bugs: true,
            daylight_savings_doubles: true,
            partial_load_incompleteness: true,
            schema_evolution_incompleteness: true,
            rollback_artifacts: true,
            retried_duplicates_with_timestamp_shift: true,
            sensor_calibration_drift: true,
            api_rate_limit_truncation: true,
            timeout_partial_writes: true,
            cache_stale_writes: true,
            null_from_migration: true,
            duplicate_after_replication: true,
            referential_integrity_broken: true,
            accidental_regex_replacements: true,
            manual_correction_inconsistencies: true,
            batch_delete_mistakes: true,
            manual_sql_update_typos: true,
            rounding_accumulation_errors: true,
            floating_point_precision_loss: true,
            type_conversion_edge_cases: true,
            default_value_spill: true,
            simultaneous_multi_layer_mess: true,
            cascading_error_propagation: true,
            error_correction_creates_new_errors: true,
        }
    }
}

pub struct RealWorldMessGenerator {
    rng: rand::rngs::StdRng,
}

impl RealWorldMessGenerator {
    pub fn new(seed: u64) -> Self {
        use rand::SeedableRng;
        RealWorldMessGenerator {
            rng: rand::rngs::StdRng::seed_from_u64(seed),
        }
    }

    pub fn apply_real_world_mess(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        config: &RealWorldMessConfig,
    ) {
        // Legacy System Cruft
        if config.legacy_system_mixed_ids {
            self.inject_legacy_id_formats(data);
        }

        if config.inconsistent_null_representations {
            self.inject_null_chaos(data);
        }

        // Human Data Entry Errors
        if config.fat_finger_errors {
            self.inject_fat_finger_errors(data);
        }

        if config.copy_paste_truncation {
            self.inject_copy_paste_truncation(data);
        }

        if config.field_swaps {
            self.inject_field_swaps(data);
        }

        if config.unit_confusion {
            self.inject_unit_confusion(data);
        }

        // System Integration Failures
        if config.encoding_mismatches {
            self.inject_encoding_chaos(data);
        }

        if config.decimal_separator_chaos {
            self.inject_decimal_chaos(data);
        }

        // Temporal Mess
        if config.timezone_chaos {
            self.inject_timezone_confusion(data);
        }

        if config.y2k_style_bugs {
            self.inject_y2k_bugs(data);
        }

        // Batch Processing Disasters
        if config.partial_load_incompleteness {
            self.inject_partial_batches(data);
        }

        if config.schema_evolution_incompleteness {
            self.inject_schema_version_mix(data);
        }

        // Sensor/Integration Drift
        if config.sensor_calibration_drift {
            self.inject_sensor_drift(data);
        }

        if config.timeout_partial_writes {
            self.inject_partial_writes(data);
        }

        // Migration Artifacts
        if config.referential_integrity_broken {
            self.inject_orphaned_fks(data);
        }

        // Operator Errors
        if config.manual_sql_update_typos {
            self.inject_update_typos(data);
        }

        // Business Logic Errors
        if config.rounding_accumulation_errors {
            self.inject_rounding_errors(data);
        }

        if config.type_conversion_edge_cases {
            self.inject_type_conversion_failures(data);
        }

        // Multi-layer cascading mess
        if config.cascading_error_propagation {
            self.inject_cascading_errors(data);
        }
    }

    fn inject_legacy_id_formats(&mut self, data: &mut [HashMap<String, String>]) {
        for (i, row) in data.iter_mut().enumerate() {
            let id = match i % 5 {
                0 => format!("ID_{:08}", i),           // Legacy format
                1 => format!("{}", i),                  // New format
                2 => format!("id-{:04x}", i),          // Hex format
                3 => format!("OLD_{}_{}", i, i * 2),  // Duplicated in old system
                _ => format!("{:016}", i),             // Different field length
            };
            row.insert("id".to_string(), id);
        }
    }

    fn inject_null_chaos(&mut self, data: &mut [HashMap<String, String>]) {
        let null_representations = vec!["NULL", "null", "N/A", "", "-1", "0", "unknown", "???", "VOID"];

        for row in data.iter_mut() {
            if self.rng.gen_bool(0.1) {
                let field = if let Some(key) = row.keys().next() {
                    key.clone()
                } else {
                    continue;
                };

                let null_rep = null_representations[self.rng.gen_range(0..null_representations.len())];
                row.insert(field, null_rep.to_string());
            }
        }
    }

    fn inject_fat_finger_errors(&mut self, data: &mut [HashMap<String, String>]) {
        let confusions = vec![('1', 'l'), ('0', 'O'), ('5', 'S'), ('8', 'B')];

        for row in data.iter_mut() {
            if self.rng.gen_bool(0.05) {
                if let Some((_, value)) = row.iter_mut().next() {
                    if !value.is_empty() && value != "NULL" {
                        let (char1, char2) = confusions[self.rng.gen_range(0..confusions.len())];
                        *value = value.replace(char1, &char2.to_string());
                    }
                }
            }
        }
    }

    fn inject_copy_paste_truncation(&mut self, data: &mut [HashMap<String, String>]) {
        for row in data.iter_mut() {
            if self.rng.gen_bool(0.03) {
                if let Some((_, value)) = row.iter_mut().next() {
                    if value.len() > 5 {
                        let truncate_point = self.rng.gen_range(1..value.len());
                        value.truncate(truncate_point);
                    }
                }
            }
        }
    }

    fn inject_field_swaps(&mut self, data: &mut [HashMap<String, String>]) {
        for row in data.iter_mut() {
            if self.rng.gen_bool(0.02) {
                let keys: Vec<_> = row.keys().cloned().collect();
                if keys.len() >= 2 {
                    let idx1 = self.rng.gen_range(0..keys.len());
                    let idx2 = self.rng.gen_range(0..keys.len());

                    if idx1 != idx2 {
                        let val1 = row[&keys[idx1]].clone();
                        let val2 = row[&keys[idx2]].clone();
                        row.insert(keys[idx1].clone(), val2);
                        row.insert(keys[idx2].clone(), val1);
                    }
                }
            }
        }
    }

    fn inject_unit_confusion(&mut self, data: &mut [HashMap<String, String>]) {
        let unit_pairs = vec![
            ("kg", "lbs"),
            ("C", "F"),
            ("m", "ft"),
            ("km", "miles"),
            ("USD", "EUR"),
        ];

        for row in data.iter_mut() {
            if self.rng.gen_bool(0.05) {
                if let Some((_, value)) = row.iter_mut().next() {
                    let (unit1, unit2) = unit_pairs[self.rng.gen_range(0..unit_pairs.len())];
                    if value.contains(unit1) {
                        *value = value.replace(unit1, unit2);
                    }
                }
            }
        }
    }

    fn inject_encoding_chaos(&mut self, data: &mut [HashMap<String, String>]) {
        for row in data.iter_mut() {
            if self.rng.gen_bool(0.02) {
                if let Some((_, value)) = row.iter_mut().next() {
                    // Simulate encoding corruption
                    let bytes: Vec<u8> = value.bytes().collect();
                    if !bytes.is_empty() {
                        let corrupt_idx = self.rng.gen_range(0..bytes.len());
                        let corrupted = format!("{}[CORRUPTED]", &value[..corrupt_idx.min(value.len())]);
                        *value = corrupted;
                    }
                }
            }
        }
    }

    fn inject_decimal_chaos(&mut self, data: &mut [HashMap<String, String>]) {
        for row in data.iter_mut() {
            if self.rng.gen_bool(0.03) {
                if let Some((_, value)) = row.iter_mut().next() {
                    if let Ok(num) = value.parse::<f64>() {
                        let new_val = match self.rng.gen_range(0..3) {
                            0 => format!("{:.2}", num).replace('.', ","), // European format
                            1 => format!("{}", (num as i64)),            // Lost decimal
                            _ => format!("{:.5}", num),                   // Too many decimals
                        };
                        *value = new_val;
                    }
                }
            }
        }
    }

    fn inject_timezone_confusion(&mut self, data: &mut [HashMap<String, String>]) {
        let timezones = vec!["UTC", "EST", "PST", "UTC+0", "UTC+8", "GMT", ""];

        for row in data.iter_mut() {
            if self.rng.gen_bool(0.05) {
                if let Some(timestamp) = row.values_mut().next() {
                    let tz = timezones[self.rng.gen_range(0..timezones.len())];
                    if !timestamp.contains("TZ") && !timestamp.is_empty() {
                        *timestamp = format!("{}_{}", timestamp, tz);
                    }
                }
            }
        }
    }

    fn inject_y2k_bugs(&mut self, data: &mut [HashMap<String, String>]) {
        for row in data.iter_mut() {
            if self.rng.gen_bool(0.02) {
                if let Some(timestamp) = row.values_mut().next() {
                    if timestamp.contains("20") {
                        *timestamp = timestamp.replace("20", "00"); // Year 0000
                    } else if timestamp.contains("19") {
                        *timestamp = timestamp.replace("19", "99"); // Year 9999
                    }
                }
            }
        }
    }

    fn inject_partial_batches(&mut self, data: &mut [HashMap<String, String>]) {
        // In later rows, some fields go missing (batch processing stopped mid-way)
        for (i, row) in data.iter_mut().enumerate() {
            if i > data.len() / 2 && self.rng.gen_bool(0.1) {
                let keys: Vec<_> = row.keys().cloned().collect();
                if !keys.is_empty() {
                    let remove_key = keys[self.rng.gen_range(0..keys.len())].clone();
                    row.remove(&remove_key);
                }
            }
        }
    }

    fn inject_schema_version_mix(&mut self, data: &mut [HashMap<String, String>]) {
        // Old rows have old schema, new rows have new schema
        for (i, row) in data.iter_mut().enumerate() {
            if i % 3 == 0 {
                row.insert("legacy_field_1".to_string(), "OLD_SCHEMA".to_string());
            } else {
                row.insert("new_field_x".to_string(), "NEW_SCHEMA".to_string());
            }
        }
    }

    fn inject_sensor_drift(&mut self, data: &mut [HashMap<String, String>]) {
        // Measurement gradually shifts over time (calibration drift)
        for (i, row) in data.iter_mut().enumerate() {
            if let Some((_, value)) = row.iter_mut().next() {
                if let Ok(num) = value.parse::<f64>() {
                    let drift = (i as f64 / data.len() as f64) * 10.0; // Drift 0-10 units
                    *value = format!("{:.2}", num + drift);
                }
            }
        }
    }

    fn inject_partial_writes(&mut self, data: &mut [HashMap<String, String>]) {
        // Timeout during write: some fields written, others not
        for row in data.iter_mut() {
            if self.rng.gen_bool(0.03) {
                let keys: Vec<_> = row.keys().cloned().collect();
                for key in keys.iter().take(keys.len() / 2) {
                    row.insert(key.clone(), "WRITE_TIMEOUT".to_string());
                }
            }
        }
    }

    fn inject_orphaned_fks(&mut self, data: &mut [HashMap<String, String>]) {
        // Foreign keys that don't exist
        for row in data.iter_mut() {
            if self.rng.gen_bool(0.02) {
                row.insert("customer_id".to_string(), format!("FK_{}", self.rng.gen_range(0..1000000)));
            }
        }
    }

    fn inject_update_typos(&mut self, data: &mut [HashMap<String, String>]) {
        // Manual SQL update had a typo: SET status = 'CLOSED' WHERE 1=1 (updates ALL rows!)
        if self.rng.gen_bool(0.02) {
            for row in data.iter_mut() {
                row.insert("status".to_string(), "INCORRECTLY_BULK_UPDATED".to_string());
            }
        }
    }

    fn inject_rounding_errors(&mut self, data: &mut [HashMap<String, String>]) {
        // Repeated rounding operations cause errors to accumulate
        for row in data.iter_mut() {
            if let Some((_, value)) = row.iter_mut().next() {
                if let Ok(num) = value.parse::<f64>() {
                    let mut acc = num;
                    for _ in 0..10 {
                        acc = (acc * 100.0).round() / 100.0; // Round 10 times
                    }
                    *value = format!("{}", acc);
                }
            }
        }
    }

    fn inject_type_conversion_failures(&mut self, data: &mut [HashMap<String, String>]) {
        // String "123abc" treated as int = 123, losing the "abc" part
        for row in data.iter_mut() {
            if self.rng.gen_bool(0.02) {
                if let Some((_, value)) = row.iter_mut().next() {
                    *value = format!("{}[TYPE_MISMATCH]", value.chars().take(3).collect::<String>());
                }
            }
        }
    }

    fn inject_cascading_errors(&mut self, data: &mut [HashMap<String, String>]) {
        // One error causes a chain reaction
        if self.rng.gen_bool(0.01) {
            for row in data.iter_mut() {
                // Initial error
                row.insert("error_source".to_string(), "cascade_start".to_string());

                // Propagates to related fields
                row.insert("error_level_1".to_string(), "propagated".to_string());
                row.insert("error_level_2".to_string(), "cascaded".to_string());
                row.insert("error_level_3".to_string(), "severe".to_string());
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessAnalysis {
    pub total_records: usize,
    pub records_with_issues: usize,
    pub issue_categories: HashMap<String, usize>,
    pub severity_score: f64,
    pub "realistic_chaos_factor": f64,
}

impl MessAnalysis {
    pub fn new(total: usize, affected: usize, chaos: f64) -> Self {
        MessAnalysis {
            total_records: total,
            records_with_issues: affected,
            issue_categories: HashMap::new(),
            severity_score: affected as f64 / total as f64,
            "realistic_chaos_factor": chaos,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_all_enabled() {
        let config = RealWorldMessConfig::default();
        assert!(config.legacy_system_mixed_ids);
        assert!(config.inconsistent_null_representations);
        assert!(config.fat_finger_errors);
        assert!(config.cascading_error_propagation);
    }

    #[test]
    fn test_legacy_ids() {
        let mut gen = RealWorldMessGenerator::new(42);
        let mut data = vec![HashMap::new(); 5];
        gen.inject_legacy_id_formats(&mut data);

        let id_formats: Vec<String> = data.iter().filter_map(|r| r.get("id").cloned()).collect();
        assert!(!id_formats.is_empty());
        assert!(id_formats.iter().any(|id| id.contains("ID_") || id.contains("old")));
    }

    #[test]
    fn test_null_chaos() {
        let mut gen = RealWorldMessGenerator::new(42);
        let mut data = vec![HashMap::from([("field".to_string(), "value".to_string())]); 10];
        gen.inject_null_chaos(&mut data);

        let null_count = data.iter().filter(|r| r.get("field").map_or(false, |v| v == "NULL" || v == "N/A" || v == "" || v == "-1")).count();
        assert!(null_count > 0);
    }

    #[test]
    fn test_decimal_chaos() {
        let mut gen = RealWorldMessGenerator::new(42);
        let mut data = vec![HashMap::from([("price".to_string(), "123.45".to_string())])];
        gen.inject_decimal_chaos(&mut data);

        let price = data[0].get("price").unwrap();
        assert!(price != "123.45"); // Should be changed
    }
}
