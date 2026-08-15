use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub entities: IndexMap<String, Entity>,
    pub relationships: Vec<Relationship>,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub fields: IndexMap<String, Field>,
    #[serde(default)]
    pub primary_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FieldType {
    String,
    Int,
    Float,
    Boolean,
    DateTime,
    Uuid,
    Enum(Vec<String>),
    Json,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    #[serde(default)]
    pub nullable: bool,
    #[serde(default)]
    pub unique: bool,
    #[serde(default)]
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub from_entity: String,
    pub to_entity: String,
    pub from_field: String,
    pub to_field: String,
    pub cardinality: Cardinality,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Cardinality {
    OneToOne,
    OneToMany,
    ManyToMany,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub constraint_type: ConstraintType,
    pub entity: String,
    pub field: Option<String>,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConstraintType {
    Range,
    Length,
    Pattern,
    Custom,
}

impl Schema {
    pub fn new() -> Self {
        Schema {
            entities: IndexMap::new(),
            relationships: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, name: String) {
        self.entities.insert(
            name.clone(),
            Entity {
                name,
                fields: IndexMap::new(),
                primary_key: None,
            },
        );
    }

    pub fn add_field(&mut self, entity: &str, field: Field) -> Result<(), String> {
        self.entities
            .get_mut(entity)
            .ok_or_else(|| format!("Entity {} not found", entity))?
            .fields
            .insert(field.name.clone(), field);
        Ok(())
    }

    pub fn add_relationship(&mut self, relationship: Relationship) {
        self.relationships.push(relationship);
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Check that all entities in relationships exist
        for rel in &self.relationships {
            if !self.entities.contains_key(&rel.from_entity) {
                errors.push(format!("Entity {} not found", rel.from_entity));
            }
            if !self.entities.contains_key(&rel.to_entity) {
                errors.push(format!("Entity {} not found", rel.to_entity));
            }
        }

        // Check that all fields in relationships exist
        for rel in &self.relationships {
            if let Some(entity) = self.entities.get(&rel.from_entity) {
                if !entity.fields.contains_key(&rel.from_field) {
                    errors.push(format!(
                        "Field {} not found in entity {}",
                        rel.from_field, rel.from_entity
                    ));
                }
            }
            if let Some(entity) = self.entities.get(&rel.to_entity) {
                if !entity.fields.contains_key(&rel.to_field) {
                    errors.push(format!(
                        "Field {} not found in entity {}",
                        rel.to_field, rel.to_entity
                    ));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(&self)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }
}

impl Default for Schema {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_creation() {
        let schema = Schema::new();
        assert_eq!(schema.entities.len(), 0);
    }

    #[test]
    fn test_add_entity() {
        let mut schema = Schema::new();
        schema.add_entity("Customer".to_string());
        assert_eq!(schema.entities.len(), 1);
        assert!(schema.entities.contains_key("Customer"));
    }

    #[test]
    fn test_schema_validation_missing_entity() {
        let mut schema = Schema::new();
        schema.relationships.push(Relationship {
            from_entity: "Customer".to_string(),
            to_entity: "Account".to_string(),
            from_field: "id".to_string(),
            to_field: "customer_id".to_string(),
            cardinality: Cardinality::OneToMany,
        });

        let result = schema.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 2);
    }
}
