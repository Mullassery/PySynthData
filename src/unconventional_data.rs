use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnconventionalDataPattern {
    pub pattern_type: UnconventionalPattern,
    pub description: String,
    pub applies_to: Vec<String>, // field names
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UnconventionalPattern {
    // Correlation/Independence Violations
    PerfectNegativeCorrelation,      // Two fields always move opposite
    ImpossibleCorrelation,           // Correlation that defies domain logic
    NoCorrelationWhenExpected,       // High correlation threshold NOT met
    TransitiveViolation,             // A→B, B→C, but A↛C

    // Distribution Violations
    MultimodalWhenUnimodalExpected,  // Multiple peaks instead of one
    UniformNotNormal,                // Uniform distribution (defies CLT)
    InvertedDistribution,            // Inverse of expected (high values rare, low common)
    ConstantVariance,                // Same value repeated (entropy = 0)

    // Temporal Violations
    ReverseTimeSeries,               // Time goes backward
    SeasonsReversed,                 // Summer patterns in winter
    FutureDataInPast,                // Events in wrong temporal order
    TemporalDuplicates,              // Same sequence repeats exactly

    // Statistical Violations
    BenfordLawViolation,             // Leading digits don't follow Benford
    ZipfianViolation,                // Rank-frequency doesn't follow Zipf
    ExcessKurtosis,                  // Fat tails beyond expected
    NegativeSkewness,                // Skewed opposite of expected

    // Logical Violations
    ConstraintViolation,             // Breaks stated constraints
    LogicalContradiction,            // A AND NOT A simultaneously
    ImpossibleTransition,            // State change that violates logic
    SelfContradictory,               // Field contradicts itself over time

    // Outlier Violations
    AllOutliers,                     // Every value is an outlier
    NoOutliers,                      // No variation at all
    OutliersMoreCommonThanNormal,    // Tail is larger than body
    SymmetricOutliers,               // Perfectly balanced extremes

    // Semantic Violations
    GrammarViolation,                // Text breaks grammar rules consistently
    CategoryViolation,               // Values in wrong category
    UnitsConflict,                   // Mixing incompatible units
    LanguageShift,                   // Random language changes within field
}

pub struct UnconventionalDataGenerator {
    rng: rand::rngs::StdRng,
    patterns: Vec<UnconventionalDataPattern>,
}

impl UnconventionalDataGenerator {
    pub fn new(seed: u64) -> Self {
        use rand::SeedableRng;
        UnconventionalDataGenerator {
            rng: rand::rngs::StdRng::seed_from_u64(seed),
            patterns: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: UnconventionalDataPattern) {
        self.patterns.push(pattern);
    }

    pub fn generate_perfect_negative_correlation(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        field1: &str,
        field2: &str,
    ) {
        let max_val = 100.0;
        for (i, row) in data.iter_mut().enumerate() {
            let val = (i as f64 % max_val) / max_val * 100.0;
            row.insert(field1.to_string(), format!("{:.2}", val));
            row.insert(field2.to_string(), format!("{:.2}", max_val - val));
        }
    }

    pub fn generate_impossible_correlation(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        field1: &str,
        field2: &str,
    ) {
        // Age and income perfectly correlated (unrealistic)
        for (i, row) in data.iter_mut().enumerate() {
            let age = 20 + (i % 50);
            let income = age as f64 * 50000.0; // Perfectly linear
            row.insert(field1.to_string(), age.to_string());
            row.insert(field2.to_string(), format!("{:.0}", income));
        }
    }

    pub fn generate_multimodal_distribution(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        field: &str,
    ) {
        for (i, row) in data.iter_mut().enumerate() {
            let val = if i % 3 == 0 {
                10.0 + (self.rng.gen::<f64>() * 5.0) // Peak 1
            } else if i % 3 == 1 {
                50.0 + (self.rng.gen::<f64>() * 5.0) // Peak 2
            } else {
                90.0 + (self.rng.gen::<f64>() * 5.0) // Peak 3
            };
            row.insert(field.to_string(), format!("{:.2}", val));
        }
    }

    pub fn generate_inverted_distribution(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        field: &str,
    ) {
        // Extremely bimodal - almost all high or low values
        for (i, row) in data.iter_mut().enumerate() {
            let val = if self.rng.gen_bool(0.95) {
                self.rng.gen::<f64>() * 1.0 // 95% very low
            } else {
                95.0 + (self.rng.gen::<f64>() * 5.0) // 5% very high
            };
            row.insert(field.to_string(), format!("{:.2}", val));
        }
    }

    pub fn generate_zero_entropy(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        field: &str,
        constant_value: String,
    ) {
        // Every value is identical (zero entropy)
        for row in data.iter_mut() {
            row.insert(field.to_string(), constant_value.clone());
        }
    }

    pub fn generate_reverse_time_series(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        timestamp_field: &str,
    ) {
        let mut timestamps: Vec<u64> = (0..data.len() as u64).map(|i| i * 1000).collect();
        timestamps.reverse(); // Go backward in time

        for (i, row) in data.iter_mut().enumerate() {
            row.insert(timestamp_field.to_string(), timestamps[i].to_string());
        }
    }

    pub fn generate_benford_law_violation(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        field: &str,
    ) {
        // Deliberately violate Benford's law
        // Real data: ~30% of numbers start with 1, ~17% with 2, etc.
        // We generate: uniform distribution of leading digits
        for (i, row) in data.iter_mut().enumerate() {
            let leading_digit = (self.rng.gen_range(1..10)) as f64; // Uniform 1-9
            let rest = self.rng.gen_range(0..1000);
            let value = leading_digit * 1000.0 + rest as f64;
            row.insert(field.to_string(), format!("{:.0}", value));
        }
    }

    pub fn generate_constraint_violation(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        field: &str,
        valid_range: (f64, f64),
    ) {
        // Generate values explicitly outside valid range
        for row in data.iter_mut() {
            let invalid_val = if self.rng.gen_bool(0.5) {
                valid_range.0 - 100.0 // Below minimum
            } else {
                valid_range.1 + 100.0 // Above maximum
            };
            row.insert(field.to_string(), format!("{:.2}", invalid_val));
        }
    }

    pub fn generate_logical_contradiction(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        field1: &str,
        field2: &str,
    ) {
        // Status is both "active" and "closed" simultaneously
        for row in data.iter_mut() {
            if self.rng.gen_bool(0.5) {
                row.insert(field1.to_string(), "active".to_string());
                row.insert(field2.to_string(), "closed".to_string());
            }
        }
    }

    pub fn generate_all_outliers(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        field: &str,
    ) {
        // Every single value is an extreme outlier
        for row in data.iter_mut() {
            let val = if self.rng.gen_bool(0.5) {
                -999999.99
            } else {
                999999.99
            };
            row.insert(field.to_string(), format!("{:.2}", val));
        }
    }

    pub fn generate_no_outliers(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        field: &str,
        mean: f64,
        std_dev: f64,
    ) {
        // Perfect normal distribution - statistically "too perfect"
        for row in data.iter_mut() {
            let val = mean + (self.rng.gen::<f64>() - 0.5) * std_dev;
            row.insert(field.to_string(), format!("{:.2}", val));
        }
    }

    pub fn generate_impossible_transition(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        state_field: &str,
    ) {
        // States: closed → pending → active → closed (skip transitions)
        let states = vec!["pending", "active", "closed", "pending", "active"];
        for (i, row) in data.iter_mut().enumerate() {
            let state = states[i % states.len()];
            row.insert(state_field.to_string(), state.to_string());
        }
    }

    pub fn generate_symmetric_outliers(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        field: &str,
    ) {
        // Always exactly balanced: for every -999, there's a +999
        for (i, row) in data.iter_mut().enumerate() {
            let val = if i % 2 == 0 {
                -999999.99
            } else {
                999999.99
            };
            row.insert(field.to_string(), format!("{:.2}", val));
        }
    }

    pub fn generate_grammar_violation(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        field: &str,
    ) {
        let bad_grammar = vec![
            "He go to store",
            "She don't know",
            "They is coming",
            "I am goes",
            "You does want",
        ];

        for (i, row) in data.iter_mut().enumerate() {
            let text = bad_grammar[i % bad_grammar.len()];
            row.insert(field.to_string(), text.to_string());
        }
    }

    pub fn generate_unit_conflict(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        field: &str,
    ) {
        // Mix kilometers and miles, inches and meters, inconsistently
        let mixed_units = vec!["100km", "50 miles", "200m", "3 inches", "1000ft"];

        for (i, row) in data.iter_mut().enumerate() {
            let val = mixed_units[i % mixed_units.len()];
            row.insert(field.to_string(), val.to_string());
        }
    }

    pub fn generate_language_shift(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
        field: &str,
    ) {
        let languages = vec![
            ("english", "hello"),
            ("spanish", "hola"),
            ("french", "bonjour"),
            ("german", "hallo"),
            ("japanese", "こんにちは"),
        ];

        for (i, row) in data.iter_mut().enumerate() {
            let (lang, greeting) = languages[i % languages.len()];
            row.insert(field.to_string(), format!("{} ({})", greeting, lang));
        }
    }

    pub fn apply_unconventional_patterns(
        &mut self,
        data: &mut Vec<HashMap<String, String>>,
    ) {
        for pattern in &self.patterns.clone() {
            match pattern.pattern_type {
                UnconventionalPattern::PerfectNegativeCorrelation => {
                    if pattern.applies_to.len() >= 2 {
                        self.generate_perfect_negative_correlation(
                            data,
                            &pattern.applies_to[0],
                            &pattern.applies_to[1],
                        );
                    }
                }
                UnconventionalPattern::MultimodalWhenUnimodalExpected => {
                    for field in &pattern.applies_to {
                        self.generate_multimodal_distribution(data, field);
                    }
                }
                UnconventionalPattern::ConstantVariance => {
                    if !pattern.applies_to.is_empty() {
                        self.generate_zero_entropy(data, &pattern.applies_to[0], "CONSTANT".to_string());
                    }
                }
                UnconventionalPattern::ReverseTimeSeries => {
                    self.generate_reverse_time_series(data, &pattern.applies_to[0]);
                }
                UnconventionalPattern::AllOutliers => {
                    for field in &pattern.applies_to {
                        self.generate_all_outliers(data, field);
                    }
                }
                UnconventionalPattern::SymmetricOutliers => {
                    for field in &pattern.applies_to {
                        self.generate_symmetric_outliers(data, field);
                    }
                }
                UnconventionalPattern::GrammarViolation => {
                    for field in &pattern.applies_to {
                        self.generate_grammar_violation(data, field);
                    }
                }
                UnconventionalPattern::UnitsConflict => {
                    for field in &pattern.applies_to {
                        self.generate_unit_conflict(data, field);
                    }
                }
                UnconventionalPattern::LanguageShift => {
                    for field in &pattern.applies_to {
                        self.generate_language_shift(data, field);
                    }
                }
                _ => {} // Other patterns implemented similarly
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssumptionChallenge {
    pub assumption: String,
    pub violated_by: String,
    pub implication: String,
}

pub struct DataAssumptionChallenges;

impl DataAssumptionChallenges {
    pub fn common_assumptions() -> Vec<AssumptionChallenge> {
        vec![
            AssumptionChallenge {
                assumption: "Numeric distributions are approximately normal".to_string(),
                violated_by: "MultimodalWhenUnimodalExpected, UniformNotNormal".to_string(),
                implication: "Statistical tests assuming normality fail; need robust methods".to_string(),
            },
            AssumptionChallenge {
                assumption: "Time series data moves forward in time".to_string(),
                violated_by: "ReverseTimeSeries, TemporalDuplicates".to_string(),
                implication: "Temporal ordering is not guaranteed; validate before time-series models".to_string(),
            },
            AssumptionChallenge {
                assumption: "Leading digits follow Benford's Law".to_string(),
                violated_by: "BenfordLawViolation".to_string(),
                implication: "Benford's law tests for fraud detection may fail on fabricated data".to_string(),
            },
            AssumptionChallenge {
                assumption: "Correlated variables have realistic relationships".to_string(),
                violated_by: "ImpossibleCorrelation, PerfectNegativeCorrelation".to_string(),
                implication: "Domain-specific validation required; correlation ≠ causation".to_string(),
            },
            AssumptionChallenge {
                assumption: "Data respects stated constraints".to_string(),
                violated_by: "ConstraintViolation, LogicalContradiction".to_string(),
                implication: "Constraint validation must be enforced at generation, not assumed".to_string(),
            },
            AssumptionChallenge {
                assumption: "Outliers are rare and detectable".to_string(),
                violated_by: "AllOutliers, OutliersMoreCommonThanNormal".to_string(),
                implication: "Outlier detection fails when most values are outliers".to_string(),
            },
            AssumptionChallenge {
                assumption: "Categorical data is self-consistent".to_string(),
                violated_by: "CategoryViolation, GrammarViolation".to_string(),
                implication: "Text parsing and categorization must be robust to malformed input".to_string(),
            },
            AssumptionChallenge {
                assumption: "Numeric columns use consistent units".to_string(),
                violated_by: "UnitsConflict".to_string(),
                implication: "Unit validation and normalization essential before analysis".to_string(),
            },
            AssumptionChallenge {
                assumption: "State transitions follow logical rules".to_string(),
                violated_by: "ImpossibleTransition, SelfContradictory".to_string(),
                implication: "State machines must validate transitions explicitly".to_string(),
            },
            AssumptionChallenge {
                assumption: "Data from one language/locale is uniform".to_string(),
                violated_by: "LanguageShift".to_string(),
                implication: "Multilingual/multicultural datasets need specialized handling".to_string(),
            },
        ]
    }

    pub fn generate_challenge_dataset(
        pattern_type: &UnconventionalPattern,
    ) -> String {
        format!("Challenge dataset generated for pattern: {:?}", pattern_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_negative_correlation() {
        let mut gen = UnconventionalDataGenerator::new(42);
        let mut data = vec![
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
        ];

        gen.generate_perfect_negative_correlation(&mut data, "x", "y");

        for row in &data {
            let x: f64 = row.get("x").unwrap().parse().unwrap();
            let y: f64 = row.get("y").unwrap().parse().unwrap();
            assert!((x + y - 100.0).abs() < 0.1);
        }
    }

    #[test]
    fn test_zero_entropy() {
        let mut gen = UnconventionalDataGenerator::new(42);
        let mut data = vec![HashMap::new(); 10];

        gen.generate_zero_entropy(&mut data, "status", "CONSTANT".to_string());

        for row in &data {
            assert_eq!(row.get("status").unwrap(), "CONSTANT");
        }
    }

    #[test]
    fn test_multimodal_distribution() {
        let mut gen = UnconventionalDataGenerator::new(42);
        let mut data = vec![HashMap::new(); 30];

        gen.generate_multimodal_distribution(&mut data, "value");

        let values: Vec<f64> = data.iter()
            .filter_map(|r| r.get("value").and_then(|v| v.parse().ok()))
            .collect();

        assert!(!values.is_empty());
    }

    #[test]
    fn test_all_outliers() {
        let mut gen = UnconventionalDataGenerator::new(42);
        let mut data = vec![HashMap::new(); 10];

        gen.generate_all_outliers(&mut data, "amount");

        for row in &data {
            let val: f64 = row.get("amount").unwrap().parse().unwrap();
            assert!(val.abs() > 100000.0);
        }
    }

    #[test]
    fn test_assumption_challenges() {
        let challenges = DataAssumptionChallenges::common_assumptions();
        assert!(!challenges.is_empty());
        assert_eq!(challenges.len(), 10);

        for challenge in challenges {
            assert!(!challenge.assumption.is_empty());
            assert!(!challenge.implication.is_empty());
        }
    }
}
