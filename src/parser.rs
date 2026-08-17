use crate::schema::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// `IndexMap` (not `HashMap`) so entity/field declaration order from the YAML
// source is preserved deterministically. `HashMap`'s iteration order is
// randomized per-instance, which previously meant parsing the *same* YAML
// twice could produce a different field generation order in `WorldGenerator`
// and therefore different rows even with an identical seed — silently
// breaking the seed-determinism the generator promises.
#[derive(Debug, Serialize, Deserialize)]
struct YamlSchema {
    entities: Option<IndexMap<String, YamlEntity>>,
    relationships: Option<Vec<YamlRelationship>>,
    constraints: Option<Vec<YamlConstraint>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct YamlEntity {
    fields: IndexMap<String, YamlField>,
    #[serde(rename = "primary_key")]
    primary_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct YamlField {
    #[serde(rename = "type")]
    field_type: String,
    nullable: Option<bool>,
    unique: Option<bool>,
    constraints: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct YamlRelationship {
    from_entity: String,
    to_entity: String,
    from_field: String,
    to_field: String,
    cardinality: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct YamlConstraint {
    constraint_type: String,
    entity: String,
    field: Option<String>,
    value: String,
}

pub struct SchemaParser;

impl SchemaParser {
    pub fn from_yaml<P: AsRef<Path>>(path: P) -> Result<Schema, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let yaml_schema: YamlSchema = serde_yaml::from_str(&contents)?;
        Self::from_parsed(yaml_schema)
    }

    pub fn from_yaml_string(yaml_str: &str) -> Result<Schema, Box<dyn std::error::Error>> {
        let yaml_schema: YamlSchema = serde_yaml::from_str(yaml_str)?;
        Self::from_parsed(yaml_schema)
    }

    fn from_parsed(yaml_schema: YamlSchema) -> Result<Schema, Box<dyn std::error::Error>> {
        let mut schema = Schema::new();

        // Parse entities
        if let Some(entities) = yaml_schema.entities {
            for (entity_name, entity_def) in entities {
                schema.add_entity(entity_name.clone());

                // Parse fields
                for (field_name, field_def) in entity_def.fields {
                    let field_type = Self::parse_field_type(&field_def.field_type)?;
                    let field = Field {
                        name: field_name,
                        field_type,
                        nullable: field_def.nullable.unwrap_or(false),
                        unique: field_def.unique.unwrap_or(false),
                        constraints: field_def.constraints.unwrap_or_default(),
                    };
                    schema.add_field(&entity_name, field)?;
                }

                // Set primary key if specified
                if let Some(pk) = entity_def.primary_key {
                    if let Some(entity) = schema.entities.get_mut(&entity_name) {
                        entity.primary_key = Some(pk);
                    }
                }
            }
        }

        // Parse relationships
        if let Some(relationships) = yaml_schema.relationships {
            for rel in relationships {
                let cardinality = match rel.cardinality.to_lowercase().as_str() {
                    "1:1" | "one_to_one" => Cardinality::OneToOne,
                    "1:n" | "one_to_many" => Cardinality::OneToMany,
                    "n:m" | "many_to_many" => Cardinality::ManyToMany,
                    _ => return Err(format!("Unknown cardinality: {}", rel.cardinality).into()),
                };

                schema.add_relationship(Relationship {
                    from_entity: rel.from_entity,
                    to_entity: rel.to_entity,
                    from_field: rel.from_field,
                    to_field: rel.to_field,
                    cardinality,
                });
            }
        }

        // Parse constraints
        if let Some(constraints) = yaml_schema.constraints {
            for constraint in constraints {
                let constraint_type = match constraint.constraint_type.to_lowercase().as_str() {
                    "range" => ConstraintType::Range,
                    "length" => ConstraintType::Length,
                    "pattern" => ConstraintType::Pattern,
                    "custom" => ConstraintType::Custom,
                    _ => {
                        return Err(format!(
                            "Unknown constraint type: {}",
                            constraint.constraint_type
                        )
                        .into())
                    }
                };

                schema.add_constraint(Constraint {
                    constraint_type,
                    entity: constraint.entity,
                    field: constraint.field,
                    value: constraint.value,
                });
            }
        }

        schema
            .validate()
            .map_err(|errors| Box::<dyn std::error::Error>::from(errors.join("; ")))?;
        Ok(schema)
    }

    fn parse_field_type(type_str: &str) -> Result<FieldType, Box<dyn std::error::Error>> {
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
                    .trim_end_matches(")")
                    .split(',')
                    .map(|v| v.trim().to_string())
                    .collect();
                Ok(FieldType::Enum(values))
            }
            _ => Err(format!("Unknown field type: {}", type_str).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_field_type_primitives() {
        assert_eq!(
            SchemaParser::parse_field_type("string").unwrap(),
            FieldType::String
        );
        assert_eq!(
            SchemaParser::parse_field_type("int").unwrap(),
            FieldType::Int
        );
        assert_eq!(
            SchemaParser::parse_field_type("float").unwrap(),
            FieldType::Float
        );
        assert_eq!(
            SchemaParser::parse_field_type("boolean").unwrap(),
            FieldType::Boolean
        );
    }

    #[test]
    fn test_parse_from_yaml_string() {
        let yaml = r#"
entities:
  Customer:
    fields:
      id:
        type: uuid
        unique: true
      name:
        type: string
      email:
        type: string
        unique: true
    primary_key: id
  Account:
    fields:
      id:
        type: uuid
        unique: true
      customer_id:
        type: uuid
    primary_key: id
relationships:
  - from_entity: Customer
    to_entity: Account
    from_field: id
    to_field: customer_id
    cardinality: 1:n
"#;

        let schema = SchemaParser::from_yaml_string(yaml).unwrap();
        assert_eq!(schema.entities.len(), 2);
        assert!(schema.entities.contains_key("Customer"));
        assert!(schema.entities.contains_key("Account"));
        assert_eq!(schema.relationships.len(), 1);
    }
}
