use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RobotType {
    MobileBase,
    MobileManipulator,
    AutonomousVehicle,
    Drone,
    HumanoidArm,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Robot {
    pub id: String,
    pub robot_type: RobotType,
    pub base_x: f64,
    pub base_y: f64,
    pub theta: f64,
    pub battery_level: f64,
    pub status: RobotStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RobotStatus {
    Idle,
    Executing,
    Failed,
    Charging,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub robot_id: String,
    pub task_type: TaskType,
    pub target_x: f64,
    pub target_y: f64,
    pub status: TaskStatus,
    pub priority: i32,
    pub created_at: u64,
    pub completed_at: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskType {
    NavigateTo,
    PickPlace,
    Inspect,
    Deliver,
    Charge,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    pub id: String,
    pub robot_id: String,
    pub timestamp: u64,
    pub sensor_type: SensorType,
    pub data: SensorData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SensorType {
    Camera,
    Lidar,
    Imu,
    Gps,
    Encoder,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    pub values: Vec<f64>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub name: String,
    pub env_type: EnvironmentType,
    pub width: f64,
    pub height: f64,
    pub obstacles: Vec<Obstacle>,
    pub landmarks: Vec<Landmark>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnvironmentType {
    Warehouse,
    Factory,
    Street,
    Building,
    OpenField,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obstacle {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub obstacle_type: ObstacleType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ObstacleType {
    Static,
    Dynamic,
    Human,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Landmark {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetSimulation {
    pub id: String,
    pub robots: HashMap<String, Robot>,
    pub tasks: Vec<Task>,
    pub sensor_readings: Vec<SensorReading>,
    pub environment: Environment,
    pub current_time: u64,
}

impl FleetSimulation {
    pub fn new(env: Environment) -> Self {
        FleetSimulation {
            id: Uuid::new_v4().to_string(),
            robots: HashMap::new(),
            tasks: Vec::new(),
            sensor_readings: Vec::new(),
            environment: env,
            current_time: 0,
        }
    }

    pub fn add_robot(&mut self, robot: Robot) {
        self.robots.insert(robot.id.clone(), robot);
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn add_sensor_reading(&mut self, reading: SensorReading) {
        self.sensor_readings.push(reading);
    }

    pub fn get_robot(&self, id: &str) -> Option<&Robot> {
        self.robots.get(id)
    }

    pub fn get_robot_mut(&mut self, id: &str) -> Option<&mut Robot> {
        self.robots.get_mut(id)
    }

    pub fn num_robots(&self) -> usize {
        self.robots.len()
    }

    pub fn num_tasks(&self) -> usize {
        self.tasks.len()
    }

    pub fn num_readings(&self) -> usize {
        self.sensor_readings.len()
    }
}

pub struct FleetCoordinator {
    allocation_strategy: AllocationStrategy,
}

#[derive(Debug, Clone, Copy)]
pub enum AllocationStrategy {
    Greedy,
    AuctionBased,
    ConsensusBased,
}

impl FleetCoordinator {
    pub fn new(strategy: AllocationStrategy) -> Self {
        FleetCoordinator {
            allocation_strategy: strategy,
        }
    }

    pub fn allocate_tasks(
        &self,
        simulation: &mut FleetSimulation,
        tasks: &[Task],
    ) -> HashMap<String, Vec<String>> {
        match self.allocation_strategy {
            AllocationStrategy::Greedy => self.allocate_greedy(simulation, tasks),
            AllocationStrategy::AuctionBased => self.allocate_auction(simulation, tasks),
            AllocationStrategy::ConsensusBased => self.allocate_consensus(simulation, tasks),
        }
    }

    fn allocate_greedy(
        &self,
        simulation: &FleetSimulation,
        tasks: &[Task],
    ) -> HashMap<String, Vec<String>> {
        let mut allocation = HashMap::new();

        for task in tasks {
            if let Some(best_robot_id) = self.find_nearest_available_robot(simulation) {
                allocation
                    .entry(best_robot_id)
                    .or_insert_with(Vec::new)
                    .push(task.id.clone());
            }
        }

        allocation
    }

    fn allocate_auction(
        &self,
        _simulation: &FleetSimulation,
        tasks: &[Task],
    ) -> HashMap<String, Vec<String>> {
        let mut allocation = HashMap::new();

        for task in tasks {
            let robot_id = format!("robot_{}", task.id);
            allocation
                .entry(robot_id)
                .or_insert_with(Vec::new)
                .push(task.id.clone());
        }

        allocation
    }

    fn allocate_consensus(
        &self,
        _simulation: &FleetSimulation,
        tasks: &[Task],
    ) -> HashMap<String, Vec<String>> {
        let mut allocation = HashMap::new();

        for (i, task) in tasks.iter().enumerate() {
            let robot_idx = i % 10;
            let robot_id = format!("robot_{}", robot_idx);
            allocation
                .entry(robot_id)
                .or_insert_with(Vec::new)
                .push(task.id.clone());
        }

        allocation
    }

    fn find_nearest_available_robot(&self, simulation: &FleetSimulation) -> Option<String> {
        simulation
            .robots
            .iter()
            .find(|(_, robot)| robot.status == RobotStatus::Idle)
            .map(|(id, _)| id.clone())
    }
}

pub struct CollisionDetector {
    grid_size: f64,
}

impl CollisionDetector {
    pub fn new(grid_size: f64) -> Self {
        CollisionDetector { grid_size }
    }

    pub fn detect_collision(&self, robot: &Robot, obstacle: &Obstacle) -> bool {
        let robot_cell_x = (robot.base_x / self.grid_size) as i32;
        let robot_cell_y = (robot.base_y / self.grid_size) as i32;

        let obstacle_cell_x = (obstacle.x / self.grid_size) as i32;
        let obstacle_cell_y = (obstacle.y / self.grid_size) as i32;

        (robot_cell_x - obstacle_cell_x).abs() <= 1 && (robot_cell_y - obstacle_cell_y).abs() <= 1
    }

    pub fn check_path_collision(
        &self,
        start_x: f64,
        start_y: f64,
        end_x: f64,
        end_y: f64,
        obstacle: &Obstacle,
    ) -> bool {
        let steps = 10;
        for i in 0..=steps {
            let t = i as f64 / steps as f64;
            let x = start_x + (end_x - start_x) * t;
            let y = start_y + (end_y - start_y) * t;

            let cell_x = (x / self.grid_size) as i32;
            let cell_y = (y / self.grid_size) as i32;

            let obs_cell_x = (obstacle.x / self.grid_size) as i32;
            let obs_cell_y = (obstacle.y / self.grid_size) as i32;

            if (cell_x - obs_cell_x).abs() <= 1 && (cell_y - obs_cell_y).abs() <= 1 {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fleet_simulation_creation() {
        let env = Environment {
            name: "Warehouse".to_string(),
            env_type: EnvironmentType::Warehouse,
            width: 100.0,
            height: 100.0,
            obstacles: vec![],
            landmarks: vec![],
        };

        let sim = FleetSimulation::new(env);
        assert_eq!(sim.num_robots(), 0);
        assert_eq!(sim.num_tasks(), 0);
    }

    #[test]
    fn test_add_robot() {
        let env = Environment {
            name: "Warehouse".to_string(),
            env_type: EnvironmentType::Warehouse,
            width: 100.0,
            height: 100.0,
            obstacles: vec![],
            landmarks: vec![],
        };

        let mut sim = FleetSimulation::new(env);
        let robot = Robot {
            id: "robot_1".to_string(),
            robot_type: RobotType::MobileBase,
            base_x: 10.0,
            base_y: 10.0,
            theta: 0.0,
            battery_level: 100.0,
            status: RobotStatus::Idle,
        };

        sim.add_robot(robot);
        assert_eq!(sim.num_robots(), 1);
    }

    #[test]
    fn test_collision_detection() {
        let detector = CollisionDetector::new(1.0);

        let robot = Robot {
            id: "robot_1".to_string(),
            robot_type: RobotType::MobileBase,
            base_x: 5.0,
            base_y: 5.0,
            theta: 0.0,
            battery_level: 100.0,
            status: RobotStatus::Idle,
        };

        let obstacle = Obstacle {
            id: "obs_1".to_string(),
            x: 5.5,
            y: 5.5,
            width: 1.0,
            height: 1.0,
            obstacle_type: ObstacleType::Static,
        };

        assert!(detector.detect_collision(&robot, &obstacle));
    }

    #[test]
    fn test_fleet_coordinator_greedy() {
        let coordinator = FleetCoordinator::new(AllocationStrategy::Greedy);

        let env = Environment {
            name: "Warehouse".to_string(),
            env_type: EnvironmentType::Warehouse,
            width: 100.0,
            height: 100.0,
            obstacles: vec![],
            landmarks: vec![],
        };

        let mut sim = FleetSimulation::new(env);

        let robot = Robot {
            id: "robot_1".to_string(),
            robot_type: RobotType::MobileBase,
            base_x: 10.0,
            base_y: 10.0,
            theta: 0.0,
            battery_level: 100.0,
            status: RobotStatus::Idle,
        };
        sim.add_robot(robot);

        let task = Task {
            id: "task_1".to_string(),
            robot_id: "".to_string(),
            task_type: TaskType::NavigateTo,
            target_x: 50.0,
            target_y: 50.0,
            status: TaskStatus::Pending,
            priority: 1,
            created_at: 0,
            completed_at: None,
        };

        let allocation = coordinator.allocate_tasks(&mut sim, &[task]);
        assert!(!allocation.is_empty());
    }
}
