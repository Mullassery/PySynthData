use world_compiler::parser::SchemaParser;
use world_compiler::schema::FieldType;

#[test]
fn test_parse_simple_schema() {
    let yaml = r#"
entities:
  Customer:
    fields:
      id:
        type: uuid
        unique: true
      name:
        type: string
    primary_key: id
"#;

    let schema = SchemaParser::from_yaml_string(yaml).unwrap();
    assert_eq!(schema.entities.len(), 1);
    assert!(schema.entities.contains_key("Customer"));

    let customer = &schema.entities["Customer"];
    assert_eq!(customer.fields.len(), 2);
    assert!(customer.fields.contains_key("id"));
    assert!(customer.fields.contains_key("name"));
}

#[test]
fn test_parse_with_relationships() {
    let yaml = r#"
entities:
  Customer:
    fields:
      id:
        type: uuid
    primary_key: id
  Account:
    fields:
      id:
        type: uuid
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
    assert_eq!(schema.relationships.len(), 1);
}

#[test]
fn test_parse_enum_type() {
    let yaml = r#"
entities:
  Customer:
    fields:
      status:
        type: enum(active, suspended, closed)
    primary_key: status
"#;

    let schema = SchemaParser::from_yaml_string(yaml).unwrap();
    let field = &schema.entities["Customer"].fields["status"];

    match &field.field_type {
        FieldType::Enum(values) => {
            assert_eq!(values.len(), 3);
            assert!(values.contains(&"active".to_string()));
        }
        _ => panic!("Expected Enum type"),
    }
}

#[test]
fn test_parse_invalid_schema() {
    let yaml = r#"
entities:
  Customer:
    fields:
      id:
        type: uuid
relationships:
  - from_entity: Customer
    to_entity: NonExistent
    from_field: id
    to_field: customer_id
    cardinality: 1:n
"#;

    let result = SchemaParser::from_yaml_string(yaml);
    assert!(result.is_err());
}
