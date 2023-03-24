use nalgebra::{Point3, Vector3, Quaternion, UnitQuaternion};

use crate::road::RoadNetwork;

pub struct RoadUser {
    location: Point3<f32>,
    current_direction: Vector3<f32>,
    current_speed: f32,

    acceleration: f32, // m/s/s
    deceleration: f32, // m/s/s
    max_steering_angle: f32, // rads/s

    next_node: u32,
    destination_node: u32,
}

impl RoadUser {
    pub fn tick(&mut self, network: &RoadNetwork, delta_time: f32) {
        let next_node = network.find_node(self.next_node);

        let target_speed = next_node.max_speed;

        let speed_difference = target_speed - self.current_speed;
        if self.current_speed < target_speed {
            self.current_speed += (self.acceleration * delta_time).max(speed_difference);
        } else if self.current_speed > target_speed {
            self.current_speed -= (self.deceleration * delta_time).min(speed_difference);
        }

        let target_direction = (self.location - next_node.location).normalize();
        let angle_difference = self.current_direction.xy().angle(&target_direction.xy());
        let steering_angle = angle_difference.max(self.max_steering_angle); // TODO: Something with steering angle acceleration

        // UnitQuaternion::rotation_between(target_direction., b)
    }
}
