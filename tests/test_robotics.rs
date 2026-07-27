use pysynthdata::robotics::*;

#[test]
fn test_create_fleet_simulation() {
    let env = Environment {
        name: "Test Warehouse".to_string(),
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
fn test_add_multiple_robots() {
    let env = Environment {
        name: "Test Warehouse".to_string(),
        env_type: EnvironmentType::Warehouse,
        width: 100.0,
        height: 100.0,
        obstacles: vec![],
        landmarks: vec![],
    };

    let mut sim = FleetSimulation::new(env);

    for i in 0..10 {
        let robot = Robot {
            id: format!("robot_{}", i),
            robot_type: RobotType::MobileBase,
            base_x: i as f64 * 10.0,
            base_y: i as f64 * 10.0,
            theta: 0.0,
            battery_level: 100.0,
            status: RobotStatus::Idle,
        };
        sim.add_robot(robot);
    }

    assert_eq!(sim.num_robots(), 10);
}

#[test]
fn test_task_allocation() {
    let env = Environment {
        name: "Test Warehouse".to_string(),
        env_type: EnvironmentType::Warehouse,
        width: 100.0,
        height: 100.0,
        obstacles: vec![],
        landmarks: vec![],
    };

    let mut sim = FleetSimulation::new(env);

    let robot = Robot {
        id: "robot_0".to_string(),
        robot_type: RobotType::MobileBase,
        base_x: 0.0,
        base_y: 0.0,
        theta: 0.0,
        battery_level: 100.0,
        status: RobotStatus::Idle,
    };
    sim.add_robot(robot);

    let task = Task {
        id: "task_0".to_string(),
        robot_id: "".to_string(),
        task_type: TaskType::NavigateTo,
        target_x: 50.0,
        target_y: 50.0,
        status: TaskStatus::Pending,
        priority: 1,
        created_at: 0,
        completed_at: None,
    };

    let coordinator = FleetCoordinator::new(AllocationStrategy::Greedy);
    let allocation = coordinator.allocate_tasks(&sim, &[task]);

    assert!(!allocation.is_empty());
}

#[test]
fn test_collision_detection_true() {
    let detector = CollisionDetector::new(1.0);

    let robot = Robot {
        id: "robot_0".to_string(),
        robot_type: RobotType::MobileBase,
        base_x: 5.0,
        base_y: 5.0,
        theta: 0.0,
        battery_level: 100.0,
        status: RobotStatus::Idle,
    };

    let obstacle = Obstacle {
        id: "obs_0".to_string(),
        x: 5.5,
        y: 5.5,
        width: 1.0,
        height: 1.0,
        obstacle_type: ObstacleType::Static,
    };

    assert!(detector.detect_collision(&robot, &obstacle));
}

#[test]
fn test_collision_detection_false() {
    let detector = CollisionDetector::new(1.0);

    let robot = Robot {
        id: "robot_0".to_string(),
        robot_type: RobotType::MobileBase,
        base_x: 0.0,
        base_y: 0.0,
        theta: 0.0,
        battery_level: 100.0,
        status: RobotStatus::Idle,
    };

    let obstacle = Obstacle {
        id: "obs_0".to_string(),
        x: 50.0,
        y: 50.0,
        width: 1.0,
        height: 1.0,
        obstacle_type: ObstacleType::Static,
    };

    assert!(!detector.detect_collision(&robot, &obstacle));
}

#[test]
fn test_path_collision() {
    let detector = CollisionDetector::new(1.0);

    let obstacle = Obstacle {
        id: "obs_0".to_string(),
        x: 5.0,
        y: 5.0,
        width: 1.0,
        height: 1.0,
        obstacle_type: ObstacleType::Static,
    };

    let collision = detector.check_path_collision(0.0, 0.0, 10.0, 10.0, &obstacle);
    assert!(collision);
}
