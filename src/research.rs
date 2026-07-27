use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainResearch {
    pub domain: String,
    pub entities: Vec<EntityArchetype>,
    pub relationships: Vec<RelationshipPattern>,
    pub behaviors: Vec<BehaviorPattern>,
    pub constraints: Vec<ConstraintPattern>,
    pub edge_cases: Vec<EdgeCasePattern>,
    pub metadata: ResearchMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityArchetype {
    pub name: String,
    pub description: String,
    pub typical_fields: Vec<FieldArchetype>,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldArchetype {
    pub name: String,
    pub field_type: String,
    pub description: String,
    pub typical_distribution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipPattern {
    pub from_entity: String,
    pub to_entity: String,
    pub cardinality: String,
    pub description: String,
    pub constraint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorPattern {
    pub entity: String,
    pub behavior_name: String,
    pub description: String,
    pub state_machine: Option<Vec<StateTransition>>,
    pub frequency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    pub from_state: String,
    pub to_state: String,
    pub probability: f64,
    pub trigger: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintPattern {
    pub entity: String,
    pub field: Option<String>,
    pub constraint_type: String,
    pub description: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeCasePattern {
    pub name: String,
    pub description: String,
    pub severity: EdgeCaseSeverity,
    pub affected_entity: String,
    pub trigger_condition: String,
    pub frequency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EdgeCaseSeverity {
    Minor,
    Moderate,
    Severe,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchMetadata {
    pub sources: Vec<String>,
    pub confidence_score: f64,
    pub last_updated: u64,
}

pub struct DomainKnowledgeBase {
    domains: HashMap<String, DomainResearch>,
}

impl DomainKnowledgeBase {
    pub fn new() -> Self {
        let mut kb = DomainKnowledgeBase {
            domains: HashMap::new(),
        };
        kb.initialize_default_domains();
        kb
    }

    fn initialize_default_domains(&mut self) {
        self.add_banking_domain();
        self.add_insurance_domain();
        self.add_healthcare_domain();
        self.add_manufacturing_domain();
        self.add_robotics_domain();
    }

    fn add_banking_domain(&mut self) {
        let banking = DomainResearch {
            domain: "Banking".to_string(),
            entities: vec![
                EntityArchetype {
                    name: "Customer".to_string(),
                    description: "Bank customer".to_string(),
                    typical_fields: vec![
                        FieldArchetype {
                            name: "id".to_string(),
                            field_type: "uuid".to_string(),
                            description: "Unique customer ID".to_string(),
                            typical_distribution: "uniform".to_string(),
                        },
                        FieldArchetype {
                            name: "status".to_string(),
                            field_type: "enum".to_string(),
                            description: "Customer status".to_string(),
                            typical_distribution: "active:95%, suspended:4%, closed:1%".to_string(),
                        },
                    ],
                    examples: vec!["John Doe".to_string(), "Jane Smith".to_string()],
                },
                EntityArchetype {
                    name: "Account".to_string(),
                    description: "Bank account".to_string(),
                    typical_fields: vec![
                        FieldArchetype {
                            name: "balance".to_string(),
                            field_type: "float".to_string(),
                            description: "Account balance".to_string(),
                            typical_distribution: "lognormal".to_string(),
                        },
                    ],
                    examples: vec!["Checking".to_string(), "Savings".to_string()],
                },
            ],
            relationships: vec![RelationshipPattern {
                from_entity: "Customer".to_string(),
                to_entity: "Account".to_string(),
                cardinality: "1:n".to_string(),
                description: "Customer owns accounts".to_string(),
                constraint: Some("1-10 accounts per customer".to_string()),
            }],
            behaviors: vec![BehaviorPattern {
                entity: "Customer".to_string(),
                behavior_name: "Churn".to_string(),
                description: "Customer closes account".to_string(),
                state_machine: Some(vec![
                    StateTransition {
                        from_state: "active".to_string(),
                        to_state: "closed".to_string(),
                        probability: 0.001,
                        trigger: Some("inactivity".to_string()),
                    },
                ]),
                frequency: Some("0.1% per month".to_string()),
            }],
            constraints: vec![ConstraintPattern {
                entity: "Account".to_string(),
                field: Some("balance".to_string()),
                constraint_type: "range".to_string(),
                description: "Balance cannot be negative".to_string(),
                value: Some("0-1000000".to_string()),
            }],
            edge_cases: vec![EdgeCasePattern {
                name: "Fraud".to_string(),
                description: "Fraudulent transactions".to_string(),
                severity: EdgeCaseSeverity::Critical,
                affected_entity: "Transaction".to_string(),
                trigger_condition: "high_amount_unusual_merchant".to_string(),
                frequency: 0.001,
            }],
            metadata: ResearchMetadata {
                sources: vec!["Banking regulations".to_string()],
                confidence_score: 0.95,
                last_updated: 0,
            },
        };

        self.domains.insert("Banking".to_string(), banking);
    }

    fn add_insurance_domain(&mut self) {
        let insurance = DomainResearch {
            domain: "Insurance".to_string(),
            entities: vec![EntityArchetype {
                name: "Policy".to_string(),
                description: "Insurance policy".to_string(),
                typical_fields: vec![FieldArchetype {
                    name: "premium".to_string(),
                    field_type: "float".to_string(),
                    description: "Monthly premium".to_string(),
                    typical_distribution: "lognormal".to_string(),
                }],
                examples: vec!["Auto".to_string(), "Home".to_string()],
            }],
            relationships: vec![],
            behaviors: vec![],
            constraints: vec![],
            edge_cases: vec![],
            metadata: ResearchMetadata {
                sources: vec!["Insurance industry standards".to_string()],
                confidence_score: 0.90,
                last_updated: 0,
            },
        };

        self.domains.insert("Insurance".to_string(), insurance);
    }

    fn add_healthcare_domain(&mut self) {
        let healthcare = DomainResearch {
            domain: "Healthcare".to_string(),
            entities: vec![EntityArchetype {
                name: "Patient".to_string(),
                description: "Healthcare patient".to_string(),
                typical_fields: vec![],
                examples: vec![],
            }],
            relationships: vec![],
            behaviors: vec![],
            constraints: vec![],
            edge_cases: vec![],
            metadata: ResearchMetadata {
                sources: vec!["HIPAA regulations".to_string()],
                confidence_score: 0.85,
                last_updated: 0,
            },
        };

        self.domains.insert("Healthcare".to_string(), healthcare);
    }

    fn add_manufacturing_domain(&mut self) {
        let manufacturing = DomainResearch {
            domain: "Manufacturing".to_string(),
            entities: vec![EntityArchetype {
                name: "Equipment".to_string(),
                description: "Manufacturing equipment".to_string(),
                typical_fields: vec![],
                examples: vec![],
            }],
            relationships: vec![],
            behaviors: vec![],
            constraints: vec![],
            edge_cases: vec![],
            metadata: ResearchMetadata {
                sources: vec!["Industry standards".to_string()],
                confidence_score: 0.88,
                last_updated: 0,
            },
        };

        self.domains.insert("Manufacturing".to_string(), manufacturing);
    }

    fn add_robotics_domain(&mut self) {
        let robotics = DomainResearch {
            domain: "Robotics".to_string(),
            entities: vec![
                EntityArchetype {
                    name: "Robot".to_string(),
                    description: "Mobile/autonomous robot".to_string(),
                    typical_fields: vec![FieldArchetype {
                        name: "battery_level".to_string(),
                        field_type: "float".to_string(),
                        description: "Battery percentage".to_string(),
                        typical_distribution: "normal".to_string(),
                    }],
                    examples: vec!["Mobile base".to_string(), "Manipulator".to_string()],
                },
                EntityArchetype {
                    name: "Task".to_string(),
                    description: "Robot task/goal".to_string(),
                    typical_fields: vec![],
                    examples: vec!["Navigate".to_string(), "Pick".to_string()],
                },
            ],
            relationships: vec![RelationshipPattern {
                from_entity: "Robot".to_string(),
                to_entity: "Task".to_string(),
                cardinality: "1:n".to_string(),
                description: "Robot executes tasks".to_string(),
                constraint: None,
            }],
            behaviors: vec![BehaviorPattern {
                entity: "Robot".to_string(),
                behavior_name: "Navigation".to_string(),
                description: "Robot navigates to target".to_string(),
                state_machine: Some(vec![StateTransition {
                    from_state: "idle".to_string(),
                    to_state: "executing".to_string(),
                    probability: 1.0,
                    trigger: Some("task_assigned".to_string()),
                }]),
                frequency: None,
            }],
            constraints: vec![],
            edge_cases: vec![EdgeCasePattern {
                name: "Localization failure".to_string(),
                description: "Robot loses position estimate".to_string(),
                severity: EdgeCaseSeverity::Severe,
                affected_entity: "Robot".to_string(),
                trigger_condition: "feature_poor_environment".to_string(),
                frequency: 0.01,
            }],
            metadata: ResearchMetadata {
                sources: vec!["ROS documentation".to_string(), "Robotics research".to_string()],
                confidence_score: 0.92,
                last_updated: 0,
            },
        };

        self.domains.insert("Robotics".to_string(), robotics);
    }

    pub fn get_domain(&self, name: &str) -> Option<&DomainResearch> {
        self.domains.get(name)
    }

    pub fn list_domains(&self) -> Vec<String> {
        self.domains.keys().cloned().collect()
    }

    pub fn add_custom_domain(&mut self, domain: DomainResearch) {
        self.domains.insert(domain.domain.clone(), domain);
    }
}

pub struct SchemaInferenceEngine;

impl SchemaInferenceEngine {
    pub fn infer_from_description(description: &str, kb: &DomainKnowledgeBase) -> Option<DomainResearch> {
        let description_lower = description.to_lowercase();

        for domain_name in kb.list_domains() {
            if description_lower.contains(&domain_name.to_lowercase()) {
                return kb.get_domain(&domain_name).cloned();
            }
        }

        None
    }

    pub fn infer_entities_from_text(text: &str) -> Vec<String> {
        let keywords = vec![
            "customer", "user", "account", "transaction", "order", "product",
            "robot", "task", "sensor", "environment", "patient", "doctor",
            "policy", "claim", "equipment", "factory",
        ];

        let text_lower = text.to_lowercase();
        let mut entities = Vec::new();

        for keyword in keywords {
            if text_lower.contains(keyword) {
                entities.push(keyword.to_string());
            }
        }

        entities
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knowledge_base_creation() {
        let kb = DomainKnowledgeBase::new();
        assert!(kb.get_domain("Banking").is_some());
        assert!(kb.get_domain("Robotics").is_some());
    }

    #[test]
    fn test_list_domains() {
        let kb = DomainKnowledgeBase::new();
        let domains = kb.list_domains();
        assert!(domains.contains(&"Banking".to_string()));
        assert!(domains.contains(&"Insurance".to_string()));
    }

    #[test]
    fn test_schema_inference_banking() {
        let kb = DomainKnowledgeBase::new();
        let research = SchemaInferenceEngine::infer_from_description("Tier-1 bank", &kb);
        assert!(research.is_some());
        let r = research.unwrap();
        assert_eq!(r.domain, "Banking");
    }

    #[test]
    fn test_entity_inference() {
        let entities = SchemaInferenceEngine::infer_entities_from_text(
            "Create a system with customers, accounts, and transactions",
        );
        assert!(entities.contains(&"customer".to_string()));
        assert!(entities.contains(&"account".to_string()));
    }
}
