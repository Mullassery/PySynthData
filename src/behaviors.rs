use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;
use rand::SeedableRng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachine {
    pub name: String,
    pub states: Vec<String>,
    pub transitions: HashMap<String, Vec<StateTransition>>,
    pub initial_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    pub from: String,
    pub to: String,
    pub probability: f64,
    pub condition: Option<String>,
    pub trigger: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorModel {
    pub entity: String,
    pub state_machine: Option<StateMachine>,
    pub events: Vec<EventPattern>,
    pub temporal_rules: Vec<TemporalRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPattern {
    pub event_type: String,
    pub frequency: FrequencyDistribution,
    pub attributes: HashMap<String, AttributeSpec>,
    pub triggered_by: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FrequencyDistribution {
    Poisson { lambda: f64 },
    Exponential { mean: f64 },
    Normal { mean: f64, std_dev: f64 },
    Uniform { min: f64, max: f64 },
    Fixed { interval: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeSpec {
    pub attr_type: String,
    pub distribution: String,
    pub range: Option<(f64, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalRule {
    pub name: String,
    pub condition: String,
    pub consequences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityTimeSeries {
    pub entity_id: String,
    pub events: Vec<TimedEvent>,
    pub state_history: Vec<StateSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimedEvent {
    pub timestamp: u64,
    pub event_type: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub timestamp: u64,
    pub state: String,
    pub attributes: HashMap<String, f64>,
}

pub struct BehaviorSimulator {
    models: HashMap<String, BehaviorModel>,
    random: rand::rngs::StdRng,
}

impl BehaviorSimulator {
    pub fn new(seed: u64) -> Self {
        use rand::SeedableRng;
        BehaviorSimulator {
            models: HashMap::new(),
            random: rand::rngs::StdRng::seed_from_u64(seed),
        }
    }

    pub fn add_model(&mut self, model: BehaviorModel) {
        self.models.insert(model.entity.clone(), model);
    }

    pub fn simulate_entity(
        &mut self,
        entity_id: &str,
        entity_type: &str,
        duration: u64,
    ) -> Option<EntityTimeSeries> {
        let model = self.models.get(entity_type).cloned()?;

        let mut time_series = EntityTimeSeries {
            entity_id: entity_id.to_string(),
            events: Vec::new(),
            state_history: Vec::new(),
        };

        let mut current_state = model
            .state_machine
            .as_ref()
            .map(|sm| sm.initial_state.clone())
            .unwrap_or_else(|| "default".to_string());

        time_series.state_history.push(StateSnapshot {
            timestamp: 0,
            state: current_state.clone(),
            attributes: HashMap::new(),
        });

        let mut current_time = 0u64;

        while current_time < duration {
            if let Some(event) = self.generate_next_event(&model, &mut current_time, duration) {
                time_series.events.push(event.clone());

                if let Some(sm) = &model.state_machine {
                    if let Some(transitions) = sm.transitions.get(&current_state) {
                        if let Some(next_state) = self.sample_transition(transitions) {
                            current_state = next_state;
                            time_series.state_history.push(StateSnapshot {
                                timestamp: current_time,
                                state: current_state.clone(),
                                attributes: HashMap::new(),
                            });
                        }
                    }
                }
            } else {
                current_time += 1;
            }
        }

        Some(time_series)
    }

    fn generate_next_event(
        &mut self,
        model: &BehaviorModel,
        current_time: &mut u64,
        max_time: u64,
    ) -> Option<TimedEvent> {
        for event_pattern in &model.events {
            let should_generate = match &event_pattern.frequency {
                FrequencyDistribution::Poisson { lambda } => {
                    self.random.gen::<f64>() < lambda / 1000.0
                }
                FrequencyDistribution::Exponential { mean } => {
                    self.random.gen::<f64>() < 1.0 / (mean / 100.0)
                }
                FrequencyDistribution::Fixed { interval } => {
                    *current_time % interval == 0
                }
                _ => false,
            };

            if should_generate && *current_time < max_time {
                *current_time += 1;
                return Some(TimedEvent {
                    timestamp: *current_time,
                    event_type: event_pattern.event_type.clone(),
                    attributes: HashMap::new(),
                });
            }
        }

        None
    }

    fn sample_transition(&mut self, transitions: &[StateTransition]) -> Option<String> {
        let total_prob: f64 = transitions.iter().map(|t| t.probability).sum();

        if total_prob <= 0.0 {
            return None;
        }

        let rand_val = self.random.gen::<f64>() * total_prob;
        let mut accumulated = 0.0;

        for transition in transitions {
            accumulated += transition.probability;
            if rand_val <= accumulated {
                return Some(transition.to.clone());
            }
        }

        transitions.last().map(|t| t.to.clone())
    }
}

pub struct EdgeCaseGenerator {
    seed: u64,
}

impl EdgeCaseGenerator {
    pub fn new(seed: u64) -> Self {
        EdgeCaseGenerator { seed }
    }

    pub fn generate_edge_cases(&self, entity_type: &str, num_cases: usize) -> Vec<EdgeCase> {
        let mut cases = Vec::new();
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed);

        for i in 0..num_cases {
            let case_type = match entity_type {
                "Customer" => self.generate_customer_edge_case(i, &mut rng),
                "Account" => self.generate_account_edge_case(i, &mut rng),
                "Transaction" => self.generate_transaction_edge_case(i, &mut rng),
                "Robot" => self.generate_robot_edge_case(i, &mut rng),
                _ => self.generate_generic_edge_case(i, &mut rng),
            };

            cases.push(case_type);
        }

        cases
    }

    fn generate_customer_edge_case(&self, index: usize, rng: &mut rand::rngs::StdRng) -> EdgeCase {
        use rand::seq::SliceRandom;

        let case_types = vec![
            "dormant_account",
            "rapid_churn",
            "fraud_pattern",
            "high_net_worth",
            "bankruptcy",
        ];

        let case_type = case_types.choose(rng).unwrap().to_string();

        EdgeCase {
            id: format!("edge_{}", index),
            entity_type: "Customer".to_string(),
            case_type,
            severity: (index % 5) as u32,
            description: "Customer edge case".to_string(),
            attributes: HashMap::new(),
        }
    }

    fn generate_account_edge_case(&self, index: usize, _rng: &mut rand::rngs::StdRng) -> EdgeCase {
        EdgeCase {
            id: format!("edge_{}", index),
            entity_type: "Account".to_string(),
            case_type: "zero_balance".to_string(),
            severity: (index % 3) as u32,
            description: "Account edge case".to_string(),
            attributes: HashMap::new(),
        }
    }

    fn generate_transaction_edge_case(
        &self,
        index: usize,
        _rng: &mut rand::rngs::StdRng,
    ) -> EdgeCase {
        EdgeCase {
            id: format!("edge_{}", index),
            entity_type: "Transaction".to_string(),
            case_type: if index % 2 == 0 {
                "high_value".to_string()
            } else {
                "rapid_sequence".to_string()
            },
            severity: (index % 5) as u32,
            description: "Transaction edge case".to_string(),
            attributes: HashMap::new(),
        }
    }

    fn generate_robot_edge_case(&self, index: usize, _rng: &mut rand::rngs::StdRng) -> EdgeCase {
        EdgeCase {
            id: format!("edge_{}", index),
            entity_type: "Robot".to_string(),
            case_type: if index % 3 == 0 {
                "localization_failure".to_string()
            } else if index % 3 == 1 {
                "battery_critical".to_string()
            } else {
                "collision_imminent".to_string()
            },
            severity: (index % 4) as u32,
            description: "Robot edge case".to_string(),
            attributes: HashMap::new(),
        }
    }

    fn generate_generic_edge_case(&self, index: usize, _rng: &mut rand::rngs::StdRng) -> EdgeCase {
        EdgeCase {
            id: format!("edge_{}", index),
            entity_type: "Unknown".to_string(),
            case_type: "generic".to_string(),
            severity: (index % 5) as u32,
            description: "Generic edge case".to_string(),
            attributes: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeCase {
    pub id: String,
    pub entity_type: String,
    pub case_type: String,
    pub severity: u32,
    pub description: String,
    pub attributes: HashMap<String, String>,
}

pub struct ScenarioBranch {
    pub scenario_id: String,
    pub base_scenario: String,
    pub interventions: HashMap<String, String>,
    pub outcomes: HashMap<String, String>,
}

impl ScenarioBranch {
    pub fn new(scenario_id: String, base: String) -> Self {
        ScenarioBranch {
            scenario_id,
            base_scenario: base,
            interventions: HashMap::new(),
            outcomes: HashMap::new(),
        }
    }

    pub fn add_intervention(&mut self, key: String, value: String) {
        self.interventions.insert(key, value);
    }

    pub fn add_outcome(&mut self, key: String, value: String) {
        self.outcomes.insert(key, value);
    }
}

pub struct ScenarioSimulator;

impl ScenarioSimulator {
    pub fn branch_scenario(
        base_id: &str,
        interventions: HashMap<String, String>,
    ) -> ScenarioBranch {
        let mut branch = ScenarioBranch::new(
            format!("{}_branch", base_id),
            base_id.to_string(),
        );

        for (key, value) in interventions {
            branch.add_intervention(key, value);
        }

        branch
    }

    pub fn simulate_intervention(
        intervention: &str,
        _parameter: &str,
    ) -> (f64, f64) {
        match intervention {
            "recession" => (0.9, 1.3),
            "inflation" => (0.95, 1.15),
            "market_crash" => (0.5, 1.5),
            "new_competitor" => (0.8, 1.2),
            _ => (1.0, 1.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine_creation() {
        let mut transitions = HashMap::new();
        transitions.insert(
            "active".to_string(),
            vec![StateTransition {
                from: "active".to_string(),
                to: "closed".to_string(),
                probability: 0.01,
                condition: None,
                trigger: None,
            }],
        );

        let sm = StateMachine {
            name: "Customer".to_string(),
            states: vec!["active".to_string(), "closed".to_string()],
            transitions,
            initial_state: "active".to_string(),
        };

        assert_eq!(sm.initial_state, "active");
        assert_eq!(sm.states.len(), 2);
    }

    #[test]
    fn test_behavior_simulator() {
        let mut sim = BehaviorSimulator::new(42);

        let model = BehaviorModel {
            entity: "Customer".to_string(),
            state_machine: None,
            events: vec![],
            temporal_rules: vec![],
        };

        sim.add_model(model);
        assert!(sim.simulate_entity("cust_1", "Customer", 100).is_some());
    }

    #[test]
    fn test_edge_case_generation() {
        let gen = EdgeCaseGenerator::new(42);
        let cases = gen.generate_edge_cases("Customer", 10);

        assert_eq!(cases.len(), 10);
        assert!(cases.iter().all(|c| c.entity_type == "Customer"));
    }

    #[test]
    fn test_scenario_branching() {
        let mut interventions = HashMap::new();
        interventions.insert("market".to_string(), "recession".to_string());

        let branch = ScenarioSimulator::branch_scenario("base_scenario", interventions);
        assert_eq!(branch.base_scenario, "base_scenario");
        assert!(branch.interventions.contains_key("market"));
    }

    #[test]
    fn test_scenario_intervention() {
        let (factor1, factor2) = ScenarioSimulator::simulate_intervention("recession", "revenue");
        assert!(factor1 < 1.0);
        assert!(factor2 > 1.0);
    }
}
