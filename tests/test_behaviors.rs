use pysynthdata::behaviors::*;
use std::collections::HashMap;

#[test]
fn test_frequency_distribution() {
    let freq = FrequencyDistribution::Poisson { lambda: 0.5 };
    match freq {
        FrequencyDistribution::Poisson { lambda } => assert_eq!(lambda, 0.5),
        _ => panic!("Wrong frequency type"),
    }
}

#[test]
fn test_behavior_model_creation() {
    let model = BehaviorModel {
        entity: "Customer".to_string(),
        state_machine: None,
        events: vec![],
        temporal_rules: vec![],
    };

    assert_eq!(model.entity, "Customer");
}

#[test]
fn test_behavior_simulator_init() {
    let mut sim = BehaviorSimulator::new(42);

    let model = BehaviorModel {
        entity: "TestEntity".to_string(),
        state_machine: None,
        events: vec![EventPattern {
            event_type: "purchase".to_string(),
            frequency: FrequencyDistribution::Poisson { lambda: 0.1 },
            attributes: HashMap::new(),
            triggered_by: None,
        }],
        temporal_rules: vec![],
    };

    sim.add_model(model);
    let series = sim.simulate_entity("entity_1", "TestEntity", 1000);

    assert!(series.is_some());
    let s = series.unwrap();
    assert_eq!(s.entity_id, "entity_1");
}

#[test]
fn test_state_machine_simulation() {
    let mut transitions = HashMap::new();
    transitions.insert(
        "active".to_string(),
        vec![StateTransition {
            from: "active".to_string(),
            to: "inactive".to_string(),
            probability: 0.1,
            condition: None,
            trigger: None,
        }],
    );

    let sm = StateMachine {
        name: "CustomerStatus".to_string(),
        states: vec!["active".to_string(), "inactive".to_string()],
        transitions,
        initial_state: "active".to_string(),
    };

    let model = BehaviorModel {
        entity: "Customer".to_string(),
        state_machine: Some(sm),
        events: vec![],
        temporal_rules: vec![],
    };

    let mut sim = BehaviorSimulator::new(42);
    sim.add_model(model);

    let series = sim.simulate_entity("cust_1", "Customer", 500);
    assert!(series.is_some());

    let s = series.unwrap();
    assert!(!s.state_history.is_empty());
}

#[test]
fn test_edge_case_generation_customer() {
    let gen = EdgeCaseGenerator::new(42);
    let cases = gen.generate_edge_cases("Customer", 5);

    assert_eq!(cases.len(), 5);
    for case in cases {
        assert_eq!(case.entity_type, "Customer");
        assert!(!case.case_type.is_empty());
    }
}

#[test]
fn test_edge_case_generation_robot() {
    let gen = EdgeCaseGenerator::new(42);
    let cases = gen.generate_edge_cases("Robot", 5);

    assert_eq!(cases.len(), 5);
    for case in cases {
        assert_eq!(case.entity_type, "Robot");
        let valid_types = [
            "localization_failure",
            "battery_critical",
            "collision_imminent",
        ];
        assert!(valid_types.contains(&case.case_type.as_str()));
    }
}

#[test]
fn test_scenario_creation() {
    let mut interventions = HashMap::new();
    interventions.insert("inflation".to_string(), "1.2x".to_string());
    interventions.insert("unemployment".to_string(), "1.5x".to_string());

    let scenario = ScenarioSimulator::branch_scenario("economic_crisis", interventions);

    assert_eq!(scenario.base_scenario, "economic_crisis");
    assert_eq!(scenario.interventions.len(), 2);
}

#[test]
fn test_intervention_simulation() {
    let (shrink_factor, growth_factor) =
        ScenarioSimulator::simulate_intervention("recession", "gdp");

    assert!(shrink_factor < 1.0);
    assert!(growth_factor > 1.0);
}

#[test]
fn test_timed_event() {
    let mut attrs = HashMap::new();
    attrs.insert("amount".to_string(), "100".to_string());

    let event = TimedEvent {
        timestamp: 1000,
        event_type: "purchase".to_string(),
        attributes: attrs,
    };

    assert_eq!(event.timestamp, 1000);
    assert_eq!(event.event_type, "purchase");
}

#[test]
fn test_state_snapshot() {
    let mut attrs = HashMap::new();
    attrs.insert("balance".to_string(), 5000.0);

    let snapshot = StateSnapshot {
        timestamp: 500,
        state: "active".to_string(),
        attributes: attrs,
    };

    assert_eq!(snapshot.state, "active");
    assert_eq!(snapshot.attributes.get("balance"), Some(&5000.0));
}
