//! Schema-driven synthetic row generation.
//!
//! Given a validated `Schema`, `WorldGenerator::generate` produces real rows for every
//! entity, respecting field types, nullability, uniqueness, cross-entity relationships
//! (foreign keys), and `Range`/`Length`/`Pattern` constraints. `WorldGenerator::evaluate`
//! then computes an honest quality report (constraint violations + a fidelity score)
//! against the schema, rather than a hardcoded value.

use crate::schema::{Constraint, ConstraintType, Field, FieldType, Schema};
use anyhow::Result;
use chrono::{Duration as ChronoDuration, TimeZone, Utc};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use regex::Regex;
use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet, VecDeque};
use uuid::Uuid;

pub struct WorldGenerator {
    schema: Schema,
}

pub struct GeneratedWorld {
    pub entities: HashMap<String, Vec<Value>>,
    pub metadata: WorldMetadata,
}

pub struct WorldMetadata {
    pub seed: u64,
    pub record_count: usize,
    pub generation_time_ms: u128,
}

/// Real quality signal computed against the schema: how many generated values
/// violate declared nullability/uniqueness/range/length/pattern rules, and an
/// overall fidelity score derived from that (1.0 = zero detected violations).
#[derive(Debug, Clone, Copy)]
pub struct QualityReport {
    pub total_checks: usize,
    pub null_violations: usize,
    pub uniqueness_violations: usize,
    pub constraint_violations: usize,
    pub fidelity_score: f64,
}

impl QualityReport {
    pub fn total_violations(&self) -> usize {
        self.null_violations + self.uniqueness_violations + self.constraint_violations
    }
}

const FIRST_NAMES: &[&str] = &[
    "James",
    "Mary",
    "Robert",
    "Patricia",
    "John",
    "Jennifer",
    "Michael",
    "Linda",
    "David",
    "Elizabeth",
    "William",
    "Barbara",
    "Richard",
    "Susan",
    "Joseph",
    "Jessica",
    "Thomas",
    "Sarah",
    "Charles",
    "Karen",
];
const LAST_NAMES: &[&str] = &[
    "Smith",
    "Johnson",
    "Williams",
    "Brown",
    "Jones",
    "Garcia",
    "Miller",
    "Davis",
    "Rodriguez",
    "Martinez",
    "Hernandez",
    "Lopez",
    "Gonzalez",
    "Wilson",
    "Anderson",
    "Thomas",
    "Taylor",
    "Moore",
    "Jackson",
    "Martin",
];
const CITIES: &[&str] = &[
    "Springfield",
    "Riverside",
    "Franklin",
    "Greenville",
    "Bristol",
    "Clinton",
    "Georgetown",
    "Salem",
    "Fairview",
    "Madison",
    "Arlington",
    "Ashland",
    "Burlington",
    "Manchester",
    "Oxford",
];
const COUNTRIES: &[&str] = &[
    "United States",
    "Canada",
    "United Kingdom",
    "Germany",
    "France",
    "Australia",
    "Japan",
    "Brazil",
    "India",
    "Mexico",
];
const DOMAINS: &[&str] = &[
    "example.com",
    "mail.com",
    "test.org",
    "sample.net",
    "demo.io",
];
const WORDS: &[&str] = &[
    "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel", "india", "juliet",
    "kilo", "lima", "mike", "november", "oscar", "papa", "quebec", "romeo", "sierra", "tango",
];

impl WorldGenerator {
    pub fn new(schema: Schema) -> Self {
        WorldGenerator { schema }
    }

    /// Generate `num_records` rows for every entity in the schema.
    ///
    /// Entities are generated in dependency order (an entity referenced by a
    /// relationship's `from_entity` is generated before the `to_entity` that
    /// points at it), so foreign-key fields are populated with real values
    /// drawn from the already-generated parent rows instead of unrelated
    /// random UUIDs.
    pub fn generate(&self, num_records: usize, seed: u64) -> Result<GeneratedWorld> {
        let start = std::time::Instant::now();
        self.schema
            .validate()
            .map_err(|errors| anyhow::anyhow!(errors.join("; ")))?;

        let mut rng = StdRng::seed_from_u64(seed);
        let order = topological_order(&self.schema);

        let (range_constraints, length_constraints, pattern_constraints) =
            index_constraints(&self.schema.constraints);
        let fk_sources = index_relationships(&self.schema);

        let mut entities: HashMap<String, Vec<Value>> = HashMap::new();

        for entity_name in &order {
            let Some(entity) = self.schema.entities.get(entity_name) else {
                continue;
            };
            let mut rows: Vec<Value> = Vec::with_capacity(num_records);
            let mut unique_seen: HashMap<String, HashSet<String>> = HashMap::new();

            for _ in 0..num_records {
                rows.push(generate_row(
                    &mut rng,
                    entity_name,
                    entity,
                    &range_constraints,
                    &length_constraints,
                    &pattern_constraints,
                    &fk_sources,
                    &entities,
                    &mut unique_seen,
                ));
            }

            entities.insert(entity_name.clone(), rows);
        }

        // Real record count: what was actually generated, not the input echoed back.
        let record_count: usize = entities.values().map(|v| v.len()).sum();
        let generation_time_ms = start.elapsed().as_millis();

        Ok(GeneratedWorld {
            entities,
            metadata: WorldMetadata {
                seed,
                record_count,
                generation_time_ms,
            },
        })
    }

    /// Generate rows in `chunk_size`-sized batches, calling `on_chunk(entity_name,
    /// chunk)` for each batch instead of returning one `GeneratedWorld` with every
    /// row materialized in memory at once.
    ///
    /// Foreign-key sampling needs a referenced parent entity's rows to already
    /// exist in full, so entities that other entities' relationships point at
    /// (`from_entity` in the schema) are still fully retained in memory here --
    /// that's inherent to relational generation, not a bug. The memory-footprint
    /// fix this method provides is for entities that are *not* an FK source
    /// (typically the largest "leaf" tables, e.g. orders/events/transactions
    /// referencing customers): their rows are hashed off to `on_chunk` and
    /// dropped immediately after, so peak memory for those entities is bounded
    /// by `chunk_size`, not `num_records`.
    pub fn generate_streaming(
        &self,
        num_records: usize,
        seed: u64,
        chunk_size: usize,
        mut on_chunk: impl FnMut(&str, &[Value]) -> Result<()>,
    ) -> Result<WorldMetadata> {
        let start = std::time::Instant::now();
        self.schema
            .validate()
            .map_err(|errors| anyhow::anyhow!(errors.join("; ")))?;
        anyhow::ensure!(chunk_size > 0, "chunk_size must be greater than 0");

        let mut rng = StdRng::seed_from_u64(seed);
        let order = topological_order(&self.schema);

        let (range_constraints, length_constraints, pattern_constraints) =
            index_constraints(&self.schema.constraints);
        let fk_sources = index_relationships(&self.schema);

        // Entities referenced as an FK source must be kept fully in memory for
        // downstream entities to sample from; everything else can stream.
        let referenced_entities: HashSet<String> = self
            .schema
            .relationships
            .iter()
            .map(|r| r.from_entity.clone())
            .collect();

        let mut entities: HashMap<String, Vec<Value>> = HashMap::new();
        let mut record_count = 0usize;

        for entity_name in &order {
            let Some(entity) = self.schema.entities.get(entity_name) else {
                continue;
            };
            let must_retain = referenced_entities.contains(entity_name);
            let mut retained_rows: Vec<Value> = if must_retain {
                Vec::with_capacity(num_records)
            } else {
                Vec::new()
            };
            let mut unique_seen: HashMap<String, HashSet<String>> = HashMap::new();
            let mut chunk: Vec<Value> = Vec::with_capacity(chunk_size.min(num_records));

            for _ in 0..num_records {
                let row = generate_row(
                    &mut rng,
                    entity_name,
                    entity,
                    &range_constraints,
                    &length_constraints,
                    &pattern_constraints,
                    &fk_sources,
                    &entities,
                    &mut unique_seen,
                );
                if must_retain {
                    retained_rows.push(row.clone());
                }
                chunk.push(row);
                if chunk.len() >= chunk_size {
                    on_chunk(entity_name, &chunk)?;
                    record_count += chunk.len();
                    chunk.clear();
                }
            }
            if !chunk.is_empty() {
                on_chunk(entity_name, &chunk)?;
                record_count += chunk.len();
            }

            if must_retain {
                entities.insert(entity_name.clone(), retained_rows);
            }
        }

        Ok(WorldMetadata {
            seed,
            record_count,
            generation_time_ms: start.elapsed().as_millis(),
        })
    }

    /// Compute a real quality report for a generated world: counts of
    /// nullability/uniqueness/constraint violations against this schema, and
    /// a fidelity score derived from them (no reference dataset required).
    pub fn evaluate(&self, world: &GeneratedWorld) -> QualityReport {
        let mut total_checks = 0usize;
        let mut null_violations = 0usize;
        let mut uniqueness_violations = 0usize;
        let mut constraint_violations = 0usize;

        for (entity_name, entity) in &self.schema.entities {
            let Some(rows) = world.entities.get(entity_name) else {
                continue;
            };

            for (field_name, field) in &entity.fields {
                let mut seen = HashSet::new();
                for row in rows {
                    total_checks += 1;
                    let value = row.get(field_name);
                    let is_null = matches!(value, None | Some(Value::Null));
                    if is_null && !field.nullable {
                        null_violations += 1;
                    }
                    if field.unique && !is_null {
                        if let Some(v) = value {
                            if !seen.insert(v.to_string()) {
                                uniqueness_violations += 1;
                            }
                        }
                    }
                }
            }
        }

        for constraint in &self.schema.constraints {
            let Some(field_name) = &constraint.field else {
                continue;
            };
            let Some(rows) = world.entities.get(&constraint.entity) else {
                continue;
            };
            for row in rows {
                if !constraint_satisfied(row, field_name, constraint) {
                    constraint_violations += 1;
                }
            }
        }

        let total_violations = null_violations + uniqueness_violations + constraint_violations;
        let fidelity_score = if total_checks == 0 {
            1.0
        } else {
            (1.0 - (total_violations as f64 / total_checks as f64)).clamp(0.0, 1.0)
        };

        QualityReport {
            total_checks,
            null_violations,
            uniqueness_violations,
            constraint_violations,
            fidelity_score,
        }
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }
}

impl GeneratedWorld {
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.entities)?)
    }
}

/// Generate a single row for `entity_name`, identical logic to the inner loop
/// body `generate()` used before it was factored out so `generate_streaming()`
/// could reuse it without duplicating the field-generation/uniqueness/FK-pool
/// handling.
#[allow(clippy::too_many_arguments)]
fn generate_row(
    rng: &mut StdRng,
    entity_name: &str,
    entity: &crate::schema::Entity,
    range_constraints: &RangeConstraints,
    length_constraints: &LengthConstraints,
    pattern_constraints: &PatternConstraints,
    fk_sources: &HashMap<ConstraintKey, (String, String)>,
    entities: &HashMap<String, Vec<Value>>,
    unique_seen: &mut HashMap<String, HashSet<String>>,
) -> Value {
    let mut obj = Map::new();
    for (field_name, field) in &entity.fields {
        let key = (entity_name.to_string(), field_name.clone());

        if field.nullable && rng.gen_bool(0.05) {
            obj.insert(field_name.clone(), Value::Null);
            continue;
        }

        let fk_pool: Option<Vec<Value>> =
            fk_sources.get(&key).and_then(|(src_entity, src_field)| {
                entities.get(src_entity).map(|rows| {
                    rows.iter()
                        .filter_map(|r| r.get(src_field).cloned())
                        .collect::<Vec<Value>>()
                })
            });

        let range = range_constraints.get(&key).copied();
        let length = length_constraints.get(&key).copied();
        let pattern = pattern_constraints.get(&key);

        let mut value =
            generate_field_value(rng, field, range, length, pattern, fk_pool.as_deref());

        if field.unique {
            let seen = unique_seen.entry(field_name.clone()).or_default();
            let mut attempts = 0;
            while seen.contains(&value.to_string()) && attempts < 20 {
                value =
                    generate_field_value(rng, field, range, length, pattern, fk_pool.as_deref());
                attempts += 1;
            }
            if seen.contains(&value.to_string()) {
                value = uniquify(value, seen.len());
            }
            seen.insert(value.to_string());
        }

        obj.insert(field_name.clone(), value);
    }
    Value::Object(obj)
}

/// Order entities so that an entity referenced as `from_entity` in a
/// relationship is generated before the `to_entity` that depends on it.
/// Falls back to schema declaration order for any entity involved in a
/// relationship cycle.
fn topological_order(schema: &Schema) -> Vec<String> {
    let mut in_degree: HashMap<String, usize> =
        schema.entities.keys().cloned().map(|k| (k, 0)).collect();
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();

    for rel in &schema.relationships {
        if rel.from_entity == rel.to_entity {
            continue;
        }
        if !schema.entities.contains_key(&rel.from_entity)
            || !schema.entities.contains_key(&rel.to_entity)
        {
            continue;
        }
        adjacency
            .entry(rel.from_entity.clone())
            .or_default()
            .push(rel.to_entity.clone());
        if let Some(d) = in_degree.get_mut(&rel.to_entity) {
            *d += 1;
        }
    }

    let mut queue: VecDeque<String> = schema
        .entities
        .keys()
        .filter(|k| in_degree.get(*k).copied().unwrap_or(0) == 0)
        .cloned()
        .collect();

    let mut order = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();

    while let Some(node) = queue.pop_front() {
        if !visited.insert(node.clone()) {
            continue;
        }
        order.push(node.clone());
        if let Some(neighbors) = adjacency.get(&node) {
            for n in neighbors {
                if let Some(d) = in_degree.get_mut(n) {
                    if *d > 0 {
                        *d -= 1;
                    }
                    if *d == 0 {
                        queue.push_back(n.clone());
                    }
                }
            }
        }
    }

    for k in schema.entities.keys() {
        if !order.contains(k) {
            order.push(k.clone());
        }
    }

    order
}

type ConstraintKey = (String, String);
type RangeConstraints = HashMap<ConstraintKey, (f64, f64)>;
type LengthConstraints = HashMap<ConstraintKey, (usize, usize)>;
type PatternConstraints = HashMap<ConstraintKey, Regex>;

fn index_constraints(
    constraints: &[Constraint],
) -> (RangeConstraints, LengthConstraints, PatternConstraints) {
    let mut ranges = HashMap::new();
    let mut lengths = HashMap::new();
    let mut patterns = HashMap::new();

    for c in constraints {
        let Some(field_name) = &c.field else {
            continue;
        };
        let key = (c.entity.clone(), field_name.clone());
        match c.constraint_type {
            ConstraintType::Range => {
                if let Some(r) = parse_numeric_range(&c.value) {
                    ranges.insert(key, r);
                }
            }
            ConstraintType::Length => {
                if let Some((min, max)) = parse_numeric_range(&c.value) {
                    lengths.insert(key, (min.max(0.0) as usize, max.max(0.0) as usize));
                }
            }
            ConstraintType::Pattern => {
                if let Ok(re) = Regex::new(&c.value) {
                    patterns.insert(key, re);
                }
            }
            ConstraintType::Custom => {}
        }
    }

    (ranges, lengths, patterns)
}

/// Map (to_entity, to_field) -> (from_entity, from_field) so a foreign-key
/// field can be populated from real parent-row values instead of a random one.
fn index_relationships(schema: &Schema) -> HashMap<ConstraintKey, (String, String)> {
    let mut map = HashMap::new();
    for rel in &schema.relationships {
        map.insert(
            (rel.to_entity.clone(), rel.to_field.clone()),
            (rel.from_entity.clone(), rel.from_field.clone()),
        );
    }
    map
}

/// Parse a "min-max" constraint value, e.g. "0-1000000" or "0.01-50000".
/// Scans for the separating '-' rather than a naive split so it isn't
/// confused by exponent notation (the schemas this tool consumes don't use
/// negative bounds today).
fn parse_numeric_range(raw: &str) -> Option<(f64, f64)> {
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

fn constraint_satisfied(row: &Value, field_name: &str, constraint: &Constraint) -> bool {
    let Some(value) = row.get(field_name) else {
        return true;
    };
    if value.is_null() {
        return true;
    }
    match constraint.constraint_type {
        ConstraintType::Range => {
            let Some(num) = value.as_f64() else {
                return true;
            };
            match parse_numeric_range(&constraint.value) {
                Some((min, max)) => num >= min && num <= max,
                None => true,
            }
        }
        ConstraintType::Length => {
            let Some(s) = value.as_str() else {
                return true;
            };
            match parse_numeric_range(&constraint.value) {
                Some((min, max)) => {
                    let len = s.len() as f64;
                    len >= min && len <= max
                }
                None => true,
            }
        }
        ConstraintType::Pattern => {
            let Some(s) = value.as_str() else {
                return true;
            };
            match Regex::new(&constraint.value) {
                Ok(re) => re.is_match(s),
                Err(_) => true,
            }
        }
        ConstraintType::Custom => true,
    }
}

fn pick<'a>(rng: &mut StdRng, options: &'a [&'a str]) -> &'a str {
    options[rng.gen_range(0..options.len())]
}

/// Generate a UUID (v4-shaped) from the seeded RNG. `Uuid::new_v4()` pulls from
/// the OS random source and would silently break the seed-determinism this
/// generator promises, so bytes are drawn from `rng` instead.
fn generate_uuid(rng: &mut StdRng) -> Uuid {
    let mut bytes = [0u8; 16];
    rng.fill(&mut bytes);
    uuid::Builder::from_random_bytes(bytes).into_uuid()
}

fn generate_field_value(
    rng: &mut StdRng,
    field: &Field,
    range: Option<(f64, f64)>,
    length: Option<(usize, usize)>,
    pattern: Option<&Regex>,
    fk_pool: Option<&[Value]>,
) -> Value {
    if let Some(pool) = fk_pool {
        if !pool.is_empty() {
            if let Some(v) = pool.choose(rng) {
                return v.clone();
            }
        }
    }

    match &field.field_type {
        FieldType::Uuid => Value::String(generate_uuid(rng).to_string()),
        FieldType::String => {
            Value::String(generate_string_value(rng, &field.name, length, pattern))
        }
        FieldType::Int => {
            let (min, max) = range.unwrap_or((0.0, 1_000_000.0));
            let (min, max) = (min as i64, max.max(min) as i64);
            Value::from(rng.gen_range(min..=max))
        }
        FieldType::Float => {
            let (min, max) = range.unwrap_or((0.0, 10_000.0));
            let max = max.max(min);
            let v = if (max - min).abs() < f64::EPSILON {
                min
            } else {
                rng.gen_range(min..=max)
            };
            let rounded = (v * 100.0).round() / 100.0;
            serde_json::Number::from_f64(rounded)
                .map(Value::Number)
                .unwrap_or(Value::Null)
        }
        FieldType::Boolean => Value::Bool(rng.gen_bool(0.5)),
        FieldType::DateTime => Value::String(generate_datetime(rng)),
        FieldType::Enum(values) => {
            if values.is_empty() {
                Value::String("unknown".to_string())
            } else {
                Value::String(values[rng.gen_range(0..values.len())].clone())
            }
        }
        FieldType::Json => generate_json_value(rng),
    }
}

fn generate_string_value(
    rng: &mut StdRng,
    field_name: &str,
    length: Option<(usize, usize)>,
    pattern: Option<&Regex>,
) -> String {
    let lname = field_name.to_lowercase();
    let mut value = if lname.contains("email") {
        format!(
            "{}.{}@{}",
            pick(rng, FIRST_NAMES).to_lowercase(),
            pick(rng, LAST_NAMES).to_lowercase(),
            pick(rng, DOMAINS)
        )
    } else if lname.contains("phone") {
        format!(
            "+1-{:03}-{:03}-{:04}",
            rng.gen_range(200..999),
            rng.gen_range(200..999),
            rng.gen_range(0..9999)
        )
    } else if lname.contains("first_name") {
        pick(rng, FIRST_NAMES).to_string()
    } else if lname.contains("last_name") {
        pick(rng, LAST_NAMES).to_string()
    } else if lname == "name" || lname.ends_with("_name") {
        format!("{} {}", pick(rng, FIRST_NAMES), pick(rng, LAST_NAMES))
    } else if lname.contains("city") {
        pick(rng, CITIES).to_string()
    } else if lname.contains("country") {
        pick(rng, COUNTRIES).to_string()
    } else if lname.contains("address") {
        format!(
            "{} {} St, {}",
            rng.gen_range(100..9999),
            pick(rng, WORDS),
            pick(rng, CITIES)
        )
    } else if lname.contains("url") || lname.contains("website") {
        format!("https://{}", pick(rng, DOMAINS))
    } else if lname.contains("zip") || lname.contains("postal") {
        format!("{:05}", rng.gen_range(0..99999))
    } else {
        let word_count = rng.gen_range(1..=3);
        (0..word_count)
            .map(|_| pick(rng, WORDS))
            .collect::<Vec<_>>()
            .join("-")
    };

    if let Some(re) = pattern {
        let mut attempts = 0;
        while !re.is_match(&value) && attempts < 10 {
            value = (0..rng.gen_range(4..10))
                .map(|_| (b'a' + rng.gen_range(0..26)) as char)
                .collect();
            attempts += 1;
        }
    }

    if let Some((min_len, max_len)) = length {
        while value.len() < min_len {
            value.push('x');
        }
        if max_len > 0 && value.len() > max_len {
            value.truncate(max_len);
        }
    }

    value
}

/// A fixed window (not "now") so DateTime generation is fully determined by
/// the seed. Anchoring to `Utc::now()` would silently break the
/// seed-determinism contract: two calls to `generate()` with the same seed
/// made a few milliseconds apart (e.g. two independently-parsed `Schema`s in
/// the same process) would then produce different timestamps.
fn generate_datetime(rng: &mut StdRng) -> String {
    let window_start = Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0).unwrap();
    let window_days: i64 = 365 * 10;
    let day_offset = rng.gen_range(0..window_days);
    let second_offset = rng.gen_range(0..86_400);
    let dt =
        window_start + ChronoDuration::days(day_offset) + ChronoDuration::seconds(second_offset);
    dt.to_rfc3339()
}

fn generate_json_value(rng: &mut StdRng) -> Value {
    let mut map = Map::new();
    map.insert(
        "tag".to_string(),
        Value::String(pick(rng, WORDS).to_string()),
    );
    map.insert("priority".to_string(), Value::from(rng.gen_range(1..=5)));
    Value::Object(map)
}

fn uniquify(value: Value, salt: usize) -> Value {
    match value {
        Value::String(s) => Value::String(format!("{}-{}", s, salt + 1)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::from(i + salt as i64 + 1)
            } else if let Some(f) = n.as_f64() {
                Value::from(f + salt as f64 + 1.0)
            } else {
                Value::String(format!("{}-{}", n, salt + 1))
            }
        }
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{Cardinality, Relationship};
    use indexmap::IndexMap;

    fn create_test_schema() -> Schema {
        let mut schema = Schema::new();
        schema.add_entity("Customer".to_string());

        let mut fields = IndexMap::new();
        fields.insert(
            "id".to_string(),
            Field {
                name: "id".to_string(),
                field_type: FieldType::Uuid,
                nullable: false,
                unique: true,
                constraints: vec![],
            },
        );
        fields.insert(
            "name".to_string(),
            Field {
                name: "name".to_string(),
                field_type: FieldType::String,
                nullable: false,
                unique: false,
                constraints: vec![],
            },
        );
        fields.insert(
            "age".to_string(),
            Field {
                name: "age".to_string(),
                field_type: FieldType::Int,
                nullable: false,
                unique: false,
                constraints: vec![],
            },
        );

        if let Some(entity) = schema.entities.get_mut("Customer") {
            entity.fields = fields;
            entity.primary_key = Some("id".to_string());
        }

        schema.add_constraint(Constraint {
            constraint_type: ConstraintType::Range,
            entity: "Customer".to_string(),
            field: Some("age".to_string()),
            value: "18-90".to_string(),
        });

        schema
    }

    #[test]
    fn test_generator_creation() {
        let schema = create_test_schema();
        let gen = WorldGenerator::new(schema);
        assert!(gen.schema().entities.contains_key("Customer"));
    }

    #[test]
    fn test_generate_basic() {
        let schema = create_test_schema();
        let gen = WorldGenerator::new(schema);
        let world = gen.generate(100, 42).unwrap();
        assert_eq!(world.metadata.record_count, 100);
        assert_eq!(world.metadata.seed, 42);
        assert_eq!(world.entities["Customer"].len(), 100);
    }

    #[test]
    fn test_generate_respects_range_constraint() {
        let schema = create_test_schema();
        let gen = WorldGenerator::new(schema);
        let world = gen.generate(200, 7).unwrap();
        for row in &world.entities["Customer"] {
            let age = row["age"].as_i64().unwrap();
            assert!((18..=90).contains(&age), "age {} out of range", age);
        }
    }

    #[test]
    fn test_generate_unique_ids() {
        let schema = create_test_schema();
        let gen = WorldGenerator::new(schema);
        let world = gen.generate(300, 1).unwrap();
        let ids: HashSet<String> = world.entities["Customer"]
            .iter()
            .map(|r| r["id"].as_str().unwrap().to_string())
            .collect();
        assert_eq!(ids.len(), 300);
    }

    #[test]
    fn test_generate_deterministic_with_seed() {
        let schema = create_test_schema();
        let gen = WorldGenerator::new(schema);
        let world1 = gen.generate(20, 99).unwrap();
        let world2 = gen.generate(20, 99).unwrap();
        assert_eq!(world1.to_json().unwrap(), world2.to_json().unwrap());
    }

    #[test]
    fn test_evaluate_reports_no_violations_on_clean_generation() {
        let schema = create_test_schema();
        let gen = WorldGenerator::new(schema);
        let world = gen.generate(150, 5).unwrap();
        let report = gen.evaluate(&world);
        assert_eq!(report.total_violations(), 0);
        assert_eq!(report.fidelity_score, 1.0);
        assert!(report.total_checks > 0);
    }

    #[test]
    fn test_generate_streaming_yields_same_rows_as_generate() {
        let schema = create_test_schema();
        let gen = WorldGenerator::new(schema);

        let world = gen.generate(250, 11).unwrap();

        let mut streamed_rows: Vec<Value> = Vec::new();
        let mut chunk_sizes: Vec<usize> = Vec::new();
        let metadata = gen
            .generate_streaming(250, 11, 32, |entity_name, chunk| {
                assert_eq!(entity_name, "Customer");
                chunk_sizes.push(chunk.len());
                streamed_rows.extend_from_slice(chunk);
                Ok(())
            })
            .unwrap();

        // Same seed -> identical rows, whether generated all at once or in chunks.
        assert_eq!(world.entities["Customer"], streamed_rows);
        assert_eq!(metadata.record_count, 250);
        // 250 rows in chunks of 32 -> seven full chunks + one partial (26).
        assert_eq!(chunk_sizes, vec![32, 32, 32, 32, 32, 32, 32, 26]);
    }

    #[test]
    fn test_generate_streaming_retains_fk_parent_but_not_leaf_entity() {
        let mut schema = Schema::new();
        schema.add_entity("Customer".to_string());
        schema.add_entity("Order".to_string());

        let mut customer_fields = IndexMap::new();
        customer_fields.insert(
            "id".to_string(),
            Field {
                name: "id".to_string(),
                field_type: FieldType::Uuid,
                nullable: false,
                unique: true,
                constraints: vec![],
            },
        );
        if let Some(e) = schema.entities.get_mut("Customer") {
            e.fields = customer_fields;
            e.primary_key = Some("id".to_string());
        }

        let mut order_fields = IndexMap::new();
        order_fields.insert(
            "id".to_string(),
            Field {
                name: "id".to_string(),
                field_type: FieldType::Uuid,
                nullable: false,
                unique: true,
                constraints: vec![],
            },
        );
        order_fields.insert(
            "customer_id".to_string(),
            Field {
                name: "customer_id".to_string(),
                field_type: FieldType::Uuid,
                nullable: false,
                unique: false,
                constraints: vec![],
            },
        );
        if let Some(e) = schema.entities.get_mut("Order") {
            e.fields = order_fields;
            e.primary_key = Some("id".to_string());
        }

        schema.add_relationship(Relationship {
            from_entity: "Customer".to_string(),
            to_entity: "Order".to_string(),
            from_field: "id".to_string(),
            to_field: "customer_id".to_string(),
            cardinality: Cardinality::OneToMany,
        });

        let gen = WorldGenerator::new(schema);

        let mut streamed_order_customer_ids: HashSet<String> = HashSet::new();
        let mut streamed_customer_ids: HashSet<String> = HashSet::new();
        gen.generate_streaming(100, 5, 10, |entity_name, chunk| {
            for row in chunk {
                match entity_name {
                    "Customer" => {
                        streamed_customer_ids.insert(row["id"].as_str().unwrap().to_string());
                    }
                    "Order" => {
                        streamed_order_customer_ids
                            .insert(row["customer_id"].as_str().unwrap().to_string());
                    }
                    other => panic!("unexpected entity {other}"),
                }
            }
            Ok(())
        })
        .unwrap();

        // Every streamed Order.customer_id must reference a real streamed Customer.id --
        // proves the FK-source (Customer) entity was still fully available in memory
        // for sampling even though Order itself streamed without full retention.
        assert!(streamed_order_customer_ids.is_subset(&streamed_customer_ids));
        assert!(!streamed_customer_ids.is_empty());
    }

    #[test]
    fn test_foreign_key_generation_uses_parent_values() {
        let mut schema = Schema::new();
        schema.add_entity("Customer".to_string());
        schema.add_entity("Account".to_string());

        let mut customer_fields = IndexMap::new();
        customer_fields.insert(
            "id".to_string(),
            Field {
                name: "id".to_string(),
                field_type: FieldType::Uuid,
                nullable: false,
                unique: true,
                constraints: vec![],
            },
        );
        if let Some(entity) = schema.entities.get_mut("Customer") {
            entity.fields = customer_fields;
            entity.primary_key = Some("id".to_string());
        }

        let mut account_fields = IndexMap::new();
        account_fields.insert(
            "id".to_string(),
            Field {
                name: "id".to_string(),
                field_type: FieldType::Uuid,
                nullable: false,
                unique: true,
                constraints: vec![],
            },
        );
        account_fields.insert(
            "customer_id".to_string(),
            Field {
                name: "customer_id".to_string(),
                field_type: FieldType::Uuid,
                nullable: false,
                unique: false,
                constraints: vec![],
            },
        );
        if let Some(entity) = schema.entities.get_mut("Account") {
            entity.fields = account_fields;
            entity.primary_key = Some("id".to_string());
        }

        schema.add_relationship(Relationship {
            from_entity: "Customer".to_string(),
            to_entity: "Account".to_string(),
            from_field: "id".to_string(),
            to_field: "customer_id".to_string(),
            cardinality: Cardinality::OneToMany,
        });

        let gen = WorldGenerator::new(schema);
        let world = gen.generate(50, 3).unwrap();

        let customer_ids: HashSet<String> = world.entities["Customer"]
            .iter()
            .map(|r| r["id"].as_str().unwrap().to_string())
            .collect();

        for row in &world.entities["Account"] {
            let customer_id = row["customer_id"].as_str().unwrap();
            assert!(
                customer_ids.contains(customer_id),
                "Account.customer_id {} does not reference a real Customer.id",
                customer_id
            );
        }
    }
}
