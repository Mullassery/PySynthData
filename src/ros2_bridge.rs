use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ROS2Message {
    pub topic: String,
    pub msg_type: ROS2MessageType,
    pub timestamp: u64,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ROS2MessageType {
    Image,
    LaserScan,
    Imu,
    Odometry,
    Transform,
    Image32FC1,
    PointCloud2,
    CompressedImage,
}

pub struct ROS2Publisher {
    pub topic: String,
    pub msg_type: ROS2MessageType,
    queue_size: usize,
    messages: Vec<ROS2Message>,
}

impl ROS2Publisher {
    pub fn new(topic: String, msg_type: ROS2MessageType, queue_size: usize) -> Self {
        ROS2Publisher {
            topic,
            msg_type,
            queue_size,
            messages: Vec::new(),
        }
    }

    pub fn publish(&mut self, data: Vec<u8>, timestamp: u64) {
        let message = ROS2Message {
            topic: self.topic.clone(),
            msg_type: self.msg_type.clone(),
            timestamp,
            data,
        };

        self.messages.push(message);

        if self.messages.len() > self.queue_size {
            self.messages.remove(0);
        }
    }

    pub fn get_latest(&self) -> Option<&ROS2Message> {
        self.messages.last()
    }

    pub fn get_all(&self) -> &[ROS2Message] {
        &self.messages
    }
}

pub struct ROS2SimulatorBridge {
    publishers: HashMap<String, ROS2Publisher>,
    subscribers: Vec<String>,
}

impl Default for ROS2SimulatorBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl ROS2SimulatorBridge {
    pub fn new() -> Self {
        ROS2SimulatorBridge {
            publishers: HashMap::new(),
            subscribers: Vec::new(),
        }
    }

    pub fn create_publisher(
        &mut self,
        topic: String,
        msg_type: ROS2MessageType,
        queue_size: usize,
    ) {
        let publisher = ROS2Publisher::new(topic.clone(), msg_type, queue_size);
        self.publishers.insert(topic, publisher);
    }

    pub fn subscribe(&mut self, topic: String) {
        self.subscribers.push(topic);
    }

    pub fn publish_message(&mut self, topic: &str, data: Vec<u8>, timestamp: u64) -> bool {
        if let Some(publisher) = self.publishers.get_mut(topic) {
            publisher.publish(data, timestamp);
            true
        } else {
            false
        }
    }

    pub fn get_publisher(&self, topic: &str) -> Option<&ROS2Publisher> {
        self.publishers.get(topic)
    }

    pub fn list_topics(&self) -> Vec<String> {
        self.publishers.keys().cloned().collect()
    }

    pub fn get_message_count(&self, topic: &str) -> usize {
        self.publishers
            .get(topic)
            .map(|p| p.get_all().len())
            .unwrap_or(0)
    }
}

pub struct NavStack {
    pub planner: PathPlanner,
    pub controller: LocalController,
}

impl Default for NavStack {
    fn default() -> Self {
        Self::new()
    }
}

impl NavStack {
    pub fn new() -> Self {
        NavStack {
            planner: PathPlanner::new(),
            controller: LocalController::new(),
        }
    }

    pub fn plan_path(
        &self,
        start_x: f64,
        start_y: f64,
        goal_x: f64,
        goal_y: f64,
    ) -> Vec<(f64, f64)> {
        self.planner.plan(start_x, start_y, goal_x, goal_y)
    }

    pub fn compute_velocity(
        &self,
        current_x: f64,
        current_y: f64,
        target_x: f64,
        target_y: f64,
    ) -> (f64, f64) {
        self.controller
            .compute_velocity(current_x, current_y, target_x, target_y)
    }
}

pub struct PathPlanner {
    grid_resolution: f64,
}

impl Default for PathPlanner {
    fn default() -> Self {
        Self::new()
    }
}

impl PathPlanner {
    pub fn new() -> Self {
        PathPlanner {
            grid_resolution: 0.1,
        }
    }

    pub fn plan(&self, start_x: f64, start_y: f64, goal_x: f64, goal_y: f64) -> Vec<(f64, f64)> {
        let mut path = vec![(start_x, start_y)];

        let num_steps = ((goal_x - start_x).abs() / self.grid_resolution).ceil() as usize;

        for i in 1..=num_steps {
            let t = i as f64 / num_steps as f64;
            let x = start_x + (goal_x - start_x) * t;
            let y = start_y + (goal_y - start_y) * t;
            path.push((x, y));
        }

        path
    }
}

pub struct LocalController {
    max_linear_velocity: f64,
    max_angular_velocity: f64,
}

impl Default for LocalController {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalController {
    pub fn new() -> Self {
        LocalController {
            max_linear_velocity: 1.0,
            max_angular_velocity: 1.57,
        }
    }

    pub fn compute_velocity(
        &self,
        current_x: f64,
        current_y: f64,
        target_x: f64,
        target_y: f64,
    ) -> (f64, f64) {
        let dx = target_x - current_x;
        let dy = target_y - current_y;

        let distance = (dx * dx + dy * dy).sqrt();

        if distance < 0.1 {
            (0.0, 0.0)
        } else {
            let linear_vel = (distance / 1.0).min(self.max_linear_velocity);
            let desired_angle = dy.atan2(dx);
            let angular_vel = desired_angle.min(self.max_angular_velocity);

            (linear_vel, angular_vel)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ros2_publisher_creation() {
        let publisher = ROS2Publisher::new("/camera/image".to_string(), ROS2MessageType::Image, 10);
        assert_eq!(publisher.topic, "/camera/image");
        assert_eq!(publisher.msg_type, ROS2MessageType::Image);
    }

    #[test]
    fn test_publish_message() {
        let mut publisher =
            ROS2Publisher::new("/camera/image".to_string(), ROS2MessageType::Image, 10);

        publisher.publish(vec![1, 2, 3], 1000);
        assert_eq!(publisher.get_all().len(), 1);
        assert_eq!(publisher.get_latest().unwrap().timestamp, 1000);
    }

    #[test]
    fn test_ros2_bridge_creation() {
        let bridge = ROS2SimulatorBridge::new();
        assert_eq!(bridge.list_topics().len(), 0);
    }

    #[test]
    fn test_path_planner() {
        let planner = PathPlanner::new();
        let path = planner.plan(0.0, 0.0, 10.0, 10.0);
        assert!(path.len() > 1);
        assert_eq!(path[0], (0.0, 0.0));
    }

    #[test]
    fn test_local_controller() {
        let controller = LocalController::new();
        let (linear, angular) = controller.compute_velocity(0.0, 0.0, 1.0, 1.0);
        assert!(linear > 0.0);
        assert!(angular >= 0.0);
    }
}
