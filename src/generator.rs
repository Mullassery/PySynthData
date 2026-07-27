use crate::schema::Schema;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

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

impl WorldGenerator {
    pub fn new(schema: Schema) -> Self {
        WorldGenerator { schema }
    }

    pub fn generate(&self, num_records: usize, seed: u64) -> Result<GeneratedWorld> {
        let start = std::time::Instant::now();
        self.schema.validate().map_err(|errors| {
            anyhow::anyhow!(errors.join("; "))
        })?;

        let mut entities: HashMap<String, Vec<Value>> = HashMap::new();
        for entity_name in self.schema.entities.keys() {
            entities.insert(entity_name.clone(), Vec::new());
        }

        let generation_time_ms = start.elapsed().as_millis();

        Ok(GeneratedWorld {
            entities,
            metadata: WorldMetadata {
                seed,
                record_count: num_records,
                generation_time_ms,
            },
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{Field, FieldType};
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

        if let Some(entity) = schema.entities.get_mut("Customer") {
            entity.fields = fields;
            entity.primary_key = Some("id".to_string());
        }

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
    }
}
