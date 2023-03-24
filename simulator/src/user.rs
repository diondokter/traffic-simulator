use nalgebra::{Point3, Rotation2, Vector3};
use ordered_float::OrderedFloat;

use crate::road::RoadNetwork;

#[derive(Debug)]
pub struct RoadUser {
    location: Point3<f32>,
    current_direction: Vector3<f32>,
    current_speed: f32,

    acceleration: f32,       // m/s/s
    deceleration: f32,       // m/s/s
    max_steering_angle: f32, // rads/s

    next_node: u32,
    destination_node: u32,
}

impl RoadUser {
    pub fn new(
        location: Point3<f32>,
        current_speed: f32,
        acceleration: f32,
        deceleration: f32,
        max_steering_angle: f32,
        next_node: u32,
        destination_node: u32,
        network: &RoadNetwork,
    ) -> Self {
        Self {
            location,
            current_direction: (network.find_node(next_node).location() - location).normalize(),
            current_speed,
            acceleration,
            deceleration,
            max_steering_angle,
            next_node,
            destination_node,
        }
    }

    pub fn tick(&mut self, network: &RoadNetwork, delta_time: f32) -> bool {
        let next_node = network.find_node(self.next_node);

        let target_speed = next_node.max_speed();

        let speed_difference = target_speed - self.current_speed;
        if self.current_speed < target_speed {
            self.current_speed += (self.acceleration * delta_time).min(speed_difference);
        } else if self.current_speed > target_speed {
            self.current_speed -= (self.deceleration * delta_time).max(speed_difference);
        }

        let target_direction = (self.location - next_node.location()).normalize();

        let total_rotation =
            Rotation2::rotation_between(&target_direction.xy(), &self.current_direction.xy());

        let total_rotation_angle = total_rotation.angle();
        let max_steering_angle = self.max_steering_angle * delta_time;

        self.current_direction = nalgebra::Rotation3::new(Vector3::new(
            0.0,
            0.0,
            total_rotation_angle
                .min(max_steering_angle)
                .max(-max_steering_angle),
        ))
        .transform_vector(&self.current_direction);
        self.current_direction.z = target_direction.z;
        self.current_direction = self.current_direction.normalize();

        self.location += self.current_direction * self.current_speed * delta_time;

        if (self.location - next_node.location()).magnitude() < 0.5 {
            if self.next_node == self.destination_node {
                println!("Reached destination");
                return false;
            }

            print!("{} @ {} => ", next_node.id, next_node.location());

            let Some(next_node) = self.find_next_path_node(network) else {
                println!("Could not find a path");
                return false;
            };

            self.next_node = next_node;
        }

        true
    }

    fn find_next_path_node(&self, network: &RoadNetwork) -> Option<u32> {
        let current_node = network.find_node(self.next_node);
        let destination_node = network.find_node(self.destination_node);

        let (next_path, _) = pathfinding::directed::astar::astar(
            &current_node,
            |test_node| {
                test_node
                    .next_nodes(network)
                    .map(|next_node| (next_node, OrderedFloat(current_node.distance_to(next_node))))
            },
            |test_node| {
                OrderedFloat(test_node.distance_to(destination_node))
            },
            |test_node| {
                *test_node == destination_node
            }
        )?;

        let next_path_node = next_path.get(1)?;
        println!("{} @ {}", next_path_node.id, next_path_node.location());

        Some(next_path_node.id)
    }

    pub fn location(&self) -> Point3<f32> {
        self.location
    }

    pub fn current_direction(&self) -> Vector3<f32> {
        self.current_direction
    }

    pub fn current_speed(&self) -> f32 {
        self.current_speed
    }
}
