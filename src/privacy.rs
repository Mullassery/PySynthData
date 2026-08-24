//! Differential privacy budget tracking and noise injection.
//!
//! Real epsilon-delta accounting (sequential composition: each `spend()` call
//! subtracts from what's left, and errors once the total is exhausted) plus a
//! genuine Laplace-mechanism noise injector for numeric fields. This is the
//! feature `check_privacy_compliance()` used to accept an `epsilon` parameter
//! for and then discard, returning `not_implemented` -- this module is what
//! backs a real implementation of that.

use anyhow::{ensure, Result};
use rand::rngs::StdRng;
use rand::Rng;
use serde_json::Value;
use std::collections::HashMap;

use crate::schema::{FieldType, Schema};

/// Tracks how much of a fixed (epsilon, delta) privacy budget has been spent.
///
/// Uses basic sequential composition: the epsilon/delta cost of each
/// `spend()` call is simply added to a running total, and `spend()` refuses
/// once that total would exceed the budget declared at construction. This is
/// the standard (if conservative -- advanced composition can do better, but
/// isn't implemented here) way to account for privacy loss across multiple
/// mechanism invocations against the same data.
#[derive(Debug, Clone, Copy)]
pub struct PrivacyBudget {
    epsilon: f64,
    delta: f64,
    spent_epsilon: f64,
    spent_delta: f64,
}

impl PrivacyBudget {
    /// Create a new budget. `epsilon` must be positive; `delta` must be in
    /// `[0, 1)` (typically a small number like `1e-5`, or `0.0` for pure
    /// epsilon-differential-privacy with no delta slack).
    pub fn new(epsilon: f64, delta: f64) -> Result<Self> {
        ensure!(epsilon > 0.0, "epsilon must be > 0, got {epsilon}");
        ensure!(
            (0.0..1.0).contains(&delta),
            "delta must be in [0, 1), got {delta}"
        );
        Ok(PrivacyBudget {
            epsilon,
            delta,
            spent_epsilon: 0.0,
            spent_delta: 0.0,
        })
    }

    pub fn total_epsilon(&self) -> f64 {
        self.epsilon
    }

    pub fn total_delta(&self) -> f64 {
        self.delta
    }

    pub fn spent_epsilon(&self) -> f64 {
        self.spent_epsilon
    }

    pub fn spent_delta(&self) -> f64 {
        self.spent_delta
    }

    pub fn remaining_epsilon(&self) -> f64 {
        (self.epsilon - self.spent_epsilon).max(0.0)
    }

    pub fn remaining_delta(&self) -> f64 {
        (self.delta - self.spent_delta).max(0.0)
    }

    pub fn is_exhausted(&self) -> bool {
        self.remaining_epsilon() <= 0.0
    }

    /// Record spending `epsilon`/`delta` against this budget. Errors (and
    /// leaves the budget unchanged) if that would exceed what's left --
    /// callers should check this *before* running the mechanism the spend is
    /// for, not after, since privacy loss from a mechanism that already ran
    /// can't be un-spent.
    pub fn spend(&mut self, epsilon: f64, delta: f64) -> Result<()> {
        ensure!(epsilon >= 0.0, "epsilon spend must be >= 0, got {epsilon}");
        ensure!(delta >= 0.0, "delta spend must be >= 0, got {delta}");
        ensure!(
            self.spent_epsilon + epsilon <= self.epsilon + f64::EPSILON,
            "privacy budget exhausted: {} epsilon requested, only {} remaining of {} total",
            epsilon,
            self.remaining_epsilon(),
            self.epsilon
        );
        ensure!(
            self.spent_delta + delta <= self.delta + f64::EPSILON,
            "privacy budget exhausted: {} delta requested, only {} remaining of {} total",
            delta,
            self.remaining_delta(),
            self.delta
        );
        self.spent_epsilon += epsilon;
        self.spent_delta += delta;
        Ok(())
    }
}

/// Sample noise from a Laplace(0, scale) distribution via inverse-CDF
/// sampling from a uniform variable -- avoids pulling in a new dependency
/// for a single distribution. `scale` must be positive.
fn sample_laplace(rng: &mut StdRng, scale: f64) -> f64 {
    // u in (-0.5, 0.5), avoiding the endpoints where ln() would blow up.
    let u: f64 = rng.gen_range(-0.5 + f64::EPSILON..0.5 - f64::EPSILON);
    -scale * u.signum() * (1.0 - 2.0 * u.abs()).ln()
}

/// Add calibrated Laplace-mechanism noise to `value`, scaled by
/// `sensitivity / epsilon` (the standard Laplace mechanism calibration for
/// epsilon-differential-privacy on a numeric query with the given
/// sensitivity). Larger sensitivity or smaller epsilon means more noise.
pub fn privatize_numeric(rng: &mut StdRng, value: f64, sensitivity: f64, epsilon: f64) -> f64 {
    if sensitivity <= 0.0 || epsilon <= 0.0 {
        return value;
    }
    value + sample_laplace(rng, sensitivity / epsilon)
}

/// Report of what `privatize_world` actually did, so callers (and
/// `check_privacy_compliance()`) can see real, computed numbers instead of a
/// hardcoded confirmation.
#[derive(Debug, Clone)]
pub struct PrivacyReport {
    pub epsilon_spent: f64,
    pub delta_spent: f64,
    pub fields_privatized: Vec<(String, String)>, // (entity, field)
    pub values_perturbed: usize,
}

/// Add Laplace noise to every Int/Float field's values across `entities`,
/// spending `epsilon` from `budget` (checked and reserved up front -- this
/// errors, without touching any data, if the budget can't cover it).
///
/// The total `epsilon` is split evenly across every numeric field being
/// privatized (basic sequential composition: the sum of per-field epsilons
/// equals the total spent). Each field's sensitivity is the width of its
/// schema-declared range constraint if one exists, otherwise the observed
/// max-min spread of that field's *generated* values in this batch -- a real,
/// computed bound, not a guess. Noised numeric values are clamped back to the
/// field's declared range constraint when one exists; clamping to a
/// schema-public (not data-dependent) bound is a valid post-processing step
/// under differential privacy's post-processing theorem, so it doesn't cost
/// additional budget.
pub fn privatize_world(
    rng: &mut StdRng,
    schema: &Schema,
    entities: &mut HashMap<String, Vec<Value>>,
    budget: &mut PrivacyBudget,
    epsilon: f64,
) -> Result<PrivacyReport> {
    let numeric_fields: Vec<(String, String)> = schema
        .entities
        .iter()
        .flat_map(|(entity_name, entity)| {
            entity
                .fields
                .values()
                .filter(|field| matches!(field.field_type, FieldType::Int | FieldType::Float))
                .map(move |field| (entity_name.clone(), field.name.clone()))
        })
        .collect();

    ensure!(
        !numeric_fields.is_empty(),
        "no Int/Float fields in this schema to privatize"
    );

    budget.spend(epsilon, 0.0)?;
    let per_field_epsilon = epsilon / numeric_fields.len() as f64;

    let range_constraints: HashMap<(String, String), (f64, f64)> = schema
        .constraints
        .iter()
        .filter_map(|c| {
            let field = c.field.clone()?;
            if !matches!(c.constraint_type, crate::schema::ConstraintType::Range) {
                return None;
            }
            let (min, max) = parse_range(&c.value)?;
            Some(((c.entity.clone(), field), (min, max)))
        })
        .collect();

    let mut values_perturbed = 0usize;

    for (entity_name, field_name) in &numeric_fields {
        let Some(rows) = entities.get_mut(entity_name) else {
            continue;
        };

        let declared_range = range_constraints.get(&(entity_name.clone(), field_name.clone()));
        let sensitivity = match declared_range {
            Some((min, max)) => (max - min).abs(),
            None => {
                let observed: Vec<f64> = rows
                    .iter()
                    .filter_map(|r| r.get(field_name).and_then(|v| v.as_f64()))
                    .collect();
                observed_spread(&observed)
            }
        };

        for row in rows.iter_mut() {
            let Some(obj) = row.as_object_mut() else {
                continue;
            };
            let Some(current) = obj.get(field_name).and_then(|v| v.as_f64()) else {
                continue;
            };
            let mut noised = privatize_numeric(rng, current, sensitivity, per_field_epsilon);
            if let Some((min, max)) = declared_range {
                noised = noised.clamp(*min, *max);
            }
            let was_int = obj
                .get(field_name)
                .map(|v| v.is_i64() || v.is_u64())
                .unwrap_or(false);
            let new_value = if was_int {
                Value::from(noised.round() as i64)
            } else {
                serde_json::Number::from_f64((noised * 100.0).round() / 100.0)
                    .map(Value::Number)
                    .unwrap_or(Value::Null)
            };
            obj.insert(field_name.clone(), new_value);
            values_perturbed += 1;
        }
    }

    Ok(PrivacyReport {
        epsilon_spent: epsilon,
        delta_spent: 0.0,
        fields_privatized: numeric_fields,
        values_perturbed,
    })
}

fn observed_spread(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 1.0; // no signal to compute a spread from; avoid a zero-sensitivity no-op.
    }
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    (max - min).abs().max(1.0)
}

fn parse_range(raw: &str) -> Option<(f64, f64)> {
    let raw = raw.trim();
    let bytes = raw.as_bytes();
    for i in 1..bytes.len() {
        if bytes[i] != b'-' {
            continue;
        }
        let prev = bytes[i - 1];
        if prev == b'e' || prev == b'E' {
            continue;
        }
        let (min_str, rest) = raw.split_at(i);
        let max_str = &rest[1..];
        if let (Ok(min), Ok(max)) = (min_str.parse::<f64>(), max_str.parse::<f64>()) {
            return Some((min, max));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn budget_tracks_spend_and_remaining() {
        let mut budget = PrivacyBudget::new(1.0, 1e-5).unwrap();
        assert_eq!(budget.remaining_epsilon(), 1.0);
        budget.spend(0.4, 0.0).unwrap();
        assert!((budget.remaining_epsilon() - 0.6).abs() < 1e-9);
        assert!((budget.spent_epsilon() - 0.4).abs() < 1e-9);
    }

    #[test]
    fn budget_rejects_overspend() {
        let mut budget = PrivacyBudget::new(1.0, 0.0).unwrap();
        budget.spend(0.7, 0.0).unwrap();
        let result = budget.spend(0.5, 0.0);
        assert!(result.is_err());
        // Failed spend must not partially apply.
        assert!((budget.spent_epsilon() - 0.7).abs() < 1e-9);
    }

    #[test]
    fn budget_rejects_invalid_construction() {
        assert!(PrivacyBudget::new(0.0, 0.0).is_err());
        assert!(PrivacyBudget::new(-1.0, 0.0).is_err());
        assert!(PrivacyBudget::new(1.0, 1.0).is_err());
        assert!(PrivacyBudget::new(1.0, -0.1).is_err());
    }

    #[test]
    fn laplace_noise_is_zero_centered_over_many_samples() {
        use rand::SeedableRng;
        let mut rng = StdRng::seed_from_u64(42);
        let samples: Vec<f64> = (0..10_000).map(|_| sample_laplace(&mut rng, 1.0)).collect();
        let mean: f64 = samples.iter().sum::<f64>() / samples.len() as f64;
        assert!(mean.abs() < 0.1, "mean {mean} should be close to 0");
    }

    #[test]
    fn privatize_numeric_returns_unmodified_value_for_invalid_params() {
        use rand::SeedableRng;
        let mut rng = StdRng::seed_from_u64(1);
        assert_eq!(privatize_numeric(&mut rng, 5.0, 0.0, 1.0), 5.0);
        assert_eq!(privatize_numeric(&mut rng, 5.0, 1.0, 0.0), 5.0);
    }
}
