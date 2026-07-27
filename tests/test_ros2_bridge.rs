use pysynthdata::ros2_bridge::*;

#[test]
fn test_ros2_publisher_init() {
    let pub1 = ROS2Publisher::new(
        "/robot/camera".to_string(),
        ROS2MessageType::Image,
        10,
    );

    assert_eq!(pub1.topic, "/robot/camera");
    assert_eq!(pub1.msg_type, ROS2MessageType::Image);
    assert_eq!(pub1.get_all().len(), 0);
}

#[test]
fn test_publish_and_retrieve() {
    let mut pub1 = ROS2Publisher::new(
        "/robot/camera".to_string(),
        ROS2MessageType::Image,
        10,
    );

    pub1.publish(vec![1, 2, 3, 4, 5], 1000);
    pub1.publish(vec![6, 7, 8, 9, 10], 2000);

    assert_eq!(pub1.get_all().len(), 2);
    assert_eq!(pub1.get_latest().unwrap().timestamp, 2000);
}

#[test]
fn test_queue_size_limit() {
    let mut pub1 = ROS2Publisher::new(
        "/robot/camera".to_string(),
        ROS2MessageType::Image,
        3,
    );

    for i in 0..5 {
        pub1.publish(vec![i], 1000 + i as u64);
    }

    assert_eq!(pub1.get_all().len(), 3);
}

#[test]
fn test_ros2_bridge_publisher_management() {
    let mut bridge = ROS2SimulatorBridge::new();

    bridge.create_publisher(
        "/robot/camera".to_string(),
        ROS2MessageType::Image,
        10,
    );
    bridge.create_publisher("/robot/lidar".to_string(), ROS2MessageType::LaserScan, 10);

    let topics = bridge.list_topics();
    assert_eq!(topics.len(), 2);
    assert!(topics.contains(&"/robot/camera".to_string()));
}

#[test]
fn test_publish_via_bridge() {
    let mut bridge = ROS2SimulatorBridge::new();
    bridge.create_publisher(
        "/robot/camera".to_string(),
        ROS2MessageType::Image,
        10,
    );

    let success = bridge.publish_message("/robot/camera", vec![1, 2, 3], 1000);
    assert!(success);

    let count = bridge.get_message_count("/robot/camera");
    assert_eq!(count, 1);
}

#[test]
fn test_path_planner() {
    let planner = PathPlanner::new();
    let path = planner.plan(0.0, 0.0, 10.0, 10.0);

    assert!(path.len() > 1);
    assert_eq!(path[0], (0.0, 0.0));
    assert!(path.last().unwrap().0 >= 9.9);
}

#[test]
fn test_local_controller_moving() {
    let controller = LocalController::new();
    let (linear, _angular) = controller.compute_velocity(0.0, 0.0, 10.0, 10.0);

    assert!(linear > 0.0);
    assert!(linear <= 1.0);
}

#[test]
fn test_local_controller_at_goal() {
    let controller = LocalController::new();
    let (linear, angular) = controller.compute_velocity(0.0, 0.0, 0.05, 0.05);

    assert_eq!(linear, 0.0);
    assert_eq!(angular, 0.0);
}

#[test]
fn test_nav_stack_integration() {
    let nav = NavStack::new();
    let path = nav.plan_path(0.0, 0.0, 10.0, 10.0);

    assert!(path.len() > 1);

    let (linear, angular) = nav.compute_velocity(0.0, 0.0, 5.0, 5.0);
    assert!(linear > 0.0);
}
