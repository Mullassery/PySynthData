use pysynthdata::schema::{Cardinality, Field, FieldType, Relationship, Schema};

#[test]
fn test_schema_add_entity() {
    let mut schema = Schema::new();
    schema.add_entity("Customer".to_string());

    assert_eq!(schema.entities.len(), 1);
    assert!(schema.entities.contains_key("Customer"));
}

#[test]
fn test_schema_add_field() {
    let mut schema = Schema::new();
    schema.add_entity("Customer".to_string());

    let field = Field {
        name: "id".to_string(),
        field_type: FieldType::Uuid,
        nullable: false,
        unique: true,
        constraints: vec![],
    };

    schema.add_field("Customer", field).unwrap();

    let customer = &schema.entities["Customer"];
    assert_eq!(customer.fields.len(), 1);
    assert!(customer.fields.contains_key("id"));
}

#[test]
fn test_schema_add_relationship() {
    let mut schema = Schema::new();
    schema.add_entity("Customer".to_string());
    schema.add_entity("Account".to_string());

    schema.add_relationship(Relationship {
        from_entity: "Customer".to_string(),
        to_entity: "Account".to_string(),
        from_field: "id".to_string(),
        to_field: "customer_id".to_string(),
        cardinality: Cardinality::OneToMany,
    });

    assert_eq!(schema.relationships.len(), 1);
}

#[test]
fn test_schema_validation_success() {
    let mut schema = Schema::new();
    schema.add_entity("Customer".to_string());

    let field = Field {
        name: "id".to_string(),
        field_type: FieldType::Uuid,
        nullable: false,
        unique: true,
        constraints: vec![],
    };

    schema.add_field("Customer", field).unwrap();
    assert!(schema.validate().is_ok());
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
}

#[test]
fn test_schema_yaml_serialization() {
    let mut schema = Schema::new();
    schema.add_entity("Customer".to_string());

    let yaml = schema.to_yaml();
    assert!(yaml.is_ok());
}
