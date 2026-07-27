use pysynthdata::research::*;

#[test]
fn test_knowledge_base_init() {
    let kb = DomainKnowledgeBase::new();
    let domains = kb.list_domains();

    assert!(domains.contains(&"Banking".to_string()));
    assert!(domains.contains(&"Insurance".to_string()));
    assert!(domains.contains(&"Healthcare".to_string()));
    assert!(domains.contains(&"Manufacturing".to_string()));
    assert!(domains.contains(&"Robotics".to_string()));
}

#[test]
fn test_get_banking_domain() {
    let kb = DomainKnowledgeBase::new();
    let banking = kb.get_domain("Banking");

    assert!(banking.is_some());
    let b = banking.unwrap();
    assert_eq!(b.domain, "Banking");
    assert!(!b.entities.is_empty());
}

#[test]
fn test_get_robotics_domain() {
    let kb = DomainKnowledgeBase::new();
    let robotics = kb.get_domain("Robotics");

    assert!(robotics.is_some());
    let r = robotics.unwrap();
    assert_eq!(r.domain, "Robotics");
    assert!(r.relationships.len() > 0);
}

#[test]
fn test_schema_inference_banking() {
    let kb = DomainKnowledgeBase::new();
    let research = SchemaInferenceEngine::infer_from_description("Create a bank", &kb);

    assert!(research.is_some());
    let r = research.unwrap();
    assert_eq!(r.domain, "Banking");
}

#[test]
fn test_schema_inference_robotics() {
    let kb = DomainKnowledgeBase::new();
    let research = SchemaInferenceEngine::infer_from_description("Warehouse with robots", &kb);

    assert!(research.is_some());
    let r = research.unwrap();
    assert_eq!(r.domain, "Robotics");
}

#[test]
fn test_entity_inference_from_text() {
    let entities = SchemaInferenceEngine::infer_entities_from_text(
        "System with customers, accounts, and transactions",
    );

    assert!(entities.contains(&"customer".to_string()));
    assert!(entities.contains(&"account".to_string()));
    assert!(entities.contains(&"transaction".to_string()));
}

#[test]
fn test_add_custom_domain() {
    let mut kb = DomainKnowledgeBase::new();
    let custom = DomainResearch {
        domain: "Retail".to_string(),
        entities: vec![],
        relationships: vec![],
        behaviors: vec![],
        constraints: vec![],
        edge_cases: vec![],
        metadata: ResearchMetadata {
            sources: vec![],
            confidence_score: 0.8,
            last_updated: 0,
        },
    };

    kb.add_custom_domain(custom);
    let retail = kb.get_domain("Retail");
    assert!(retail.is_some());
    assert_eq!(retail.unwrap().domain, "Retail");
}

#[test]
fn test_edge_case_severity() {
    let edge_case = EdgeCasePattern {
        name: "Critical error".to_string(),
        description: "System crash".to_string(),
        severity: EdgeCaseSeverity::Critical,
        affected_entity: "System".to_string(),
        trigger_condition: "out_of_memory".to_string(),
        frequency: 0.0001,
    };

    assert_eq!(edge_case.severity, EdgeCaseSeverity::Critical);
}
