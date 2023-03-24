use std::f32::consts::PI;

use nalgebra::{Point3, Rotation2, Vector3};
use ordered_float::OrderedFloat;

use crate::{
    road::RoadNetwork,
    traffic_light::{TrafficLight, TrafficLightState},
};

#[derive(Debug)]
pub struct RoadUser {
    pub id: u32,

    location: Point3<f32>,
    current_direction: Vector3<f32>,
    current_speed: f32,

    acceleration: f32,       // m/s/s
    deceleration: f32,       // m/s/s
    max_steering_angle: f32, // rads/s

    next_nodes: Vec<u32>,
    destination_node: u32,
}

impl RoadUser {
    pub fn new(
        id: u32,
        location: Point3<f32>,
        current_speed: f32,
        acceleration: f32,
        deceleration: f32,
        max_steering_angle: f32,
        first_node: u32,
        destination_node: u32,
        network: &RoadNetwork,
    ) -> Self {
        Self {
            id,
            location,
            current_direction: (network.find_node(first_node).location() - location).normalize(),
            current_speed,
            acceleration,
            deceleration,
            max_steering_angle,
            next_nodes: vec![first_node],
            destination_node,
        }
    }

    pub fn tick(
        &mut self,
        network: &RoadNetwork,
        traffic_lights: &[Box<dyn TrafficLight>],
        delta_time: f32,
    ) -> bool {
        let next_node = network.find_node(self.next_nodes[0]);
        let second_next_node = self.next_nodes.get(1).map(|id| network.find_node(*id));

        let mut target_speed = next_node.max_speed();

        let target_direction = (self.location - next_node.location()).normalize();

        'corner_speed: {
            if let Some(second_next_node) = second_next_node {
                let next_target_direction = next_node.direction_to(second_next_node);
                let next_target_distance = next_node.distance_to(second_next_node);

                let expected_angle = Rotation2::rotation_between(
                    &target_direction.xy(),
                    &next_target_direction.xy(),
                )
                .angle();

                if expected_angle < PI / 10000.0 {
                    break 'corner_speed;
                }

                let min_seconds_required = expected_angle / self.max_steering_angle;
                let max_corner_speed = next_target_distance / min_seconds_required;

                let current_speed_difference_too_fast = max_corner_speed - self.current_speed;

                if current_speed_difference_too_fast > 0.0 {
                    let breaking_time_required =
                        current_speed_difference_too_fast / (self.deceleration / 2.0);
                    let current_speed_breaking_distance_required =
                        self.current_speed * breaking_time_required;

                    if (self.location - next_node.location()).magnitude()
                        < current_speed_breaking_distance_required
                    {
                        target_speed = target_speed.min(max_corner_speed);
                    }
                }
            }
        }

        let is_stopping_for_traffic_light = 'traffic_light_speed: {
            let Some(first_next_traffic_light) = self.next_nodes.iter().find_map(|node| {
                            traffic_lights
                                .iter()
                                .find_map(|light| (light.node() == *node).then_some(light))
                        }) else {
                break 'traffic_light_speed false;
            };

            if first_next_traffic_light.get_state() == TrafficLightState::Green {
                break 'traffic_light_speed false;
            }

            let distance_to_traffic_light =
                (self.location - network.find_node(self.next_nodes[0]).location()).magnitude()
                    + self
                        .next_nodes
                        .windows(2)
                        .take_while(|next_nodes| next_nodes[1] == first_next_traffic_light.node())
                        .map(|next_nodes| {
                            network
                                .find_node(next_nodes[0])
                                .distance_to(network.find_node(next_nodes[1]))
                        })
                        .sum::<f32>();

            let time_desired_to_break = self.current_speed / (self.deceleration / 1.5);
            let distance_desired_to_break = self.current_speed / 2.0 * time_desired_to_break;

            let time_required_to_break = self.current_speed / self.deceleration;
            let distance_required_to_break = self.current_speed / 2.0 * time_required_to_break;

            if distance_to_traffic_light >= distance_desired_to_break {
                break 'traffic_light_speed false;
            }

            if distance_to_traffic_light < distance_required_to_break
                && first_next_traffic_light.get_state() == TrafficLightState::Orange
            {
                break 'traffic_light_speed false;
            }

            if distance_to_traffic_light < distance_required_to_break {
                target_speed = 0.0;
            } else {
                target_speed = distance_to_traffic_light / time_desired_to_break;
            }

            println!("{target_speed}, distance_to_traffic_light: {distance_to_traffic_light}, distance_desired_to_break: {distance_desired_to_break}, distance_required_to_break: {distance_required_to_break}");

            true
        };

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

        let speed_difference = target_speed - self.current_speed;
        if self.current_speed < target_speed {
            self.current_speed += (self.acceleration * delta_time).min(speed_difference);
        } else if self.current_speed > target_speed {
            self.current_speed -= (self.deceleration * delta_time).max(speed_difference);
        }

        self.location += self.current_direction * self.current_speed * delta_time;

        if !is_stopping_for_traffic_light && (self.location - next_node.location()).magnitude() < 0.5 {
            if self.next_nodes.first() == Some(&self.destination_node) {
                println!("Reached destination");
                return false;
            }

            self.recalculate_path(network);

            if self.next_nodes.is_empty() {
                println!("Could not find a path");
                return false;
            };
        }

        true
    }

    fn recalculate_path(&mut self, network: &RoadNetwork) {
        let current_node = network.find_node(*self.next_nodes.first().unwrap());
        let destination_node = network.find_node(self.destination_node);

        let (mut next_path, _) = pathfinding::directed::astar::astar(
            &current_node,
            |test_node| {
                test_node
                    .next_nodes(network)
                    .map(|next_node| (next_node, OrderedFloat(current_node.distance_to(next_node))))
            },
            |test_node| OrderedFloat(test_node.distance_to(destination_node)),
            |test_node| *test_node == destination_node,
        )
        .unwrap_or_default();

        if !next_path.is_empty() {
            next_path.remove(0);
        }

        self.next_nodes = next_path.into_iter().map(|node| node.id).collect();
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
