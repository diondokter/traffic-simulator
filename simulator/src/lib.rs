use road::RoadNetwork;
use user::RoadUser;

pub mod road;
pub mod user;

pub struct Simulator {
    road_network: RoadNetwork,
    current_road_users: Vec<RoadUser>,
}

impl Simulator {
    pub fn new(road_network: RoadNetwork) -> Self {
        Self {
            road_network,
            current_road_users: Vec::new(),
        }
    }

    pub fn tick(&mut self, delta_time: f32) {
        self.current_road_users
            .retain_mut(|user| user.tick(&self.road_network, delta_time));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::road::Node;
    use nalgebra::Point3;
    use std::f32::consts::PI;

    #[test]
    fn single_node_one_car() {
        let mut simulator = Simulator::new(RoadNetwork::new(
            [
                (
                    0,
                    Node::new(
                        0,
                        Point3::new(8.0, 0.0, 0.0),
                        30.0 / 3.6, // 30kph
                        vec![1],
                        None,
                        None,
                    ),
                ),
                (
                    1,
                    Node::new(
                        1,
                        Point3::new(10.0, 0.0, 0.0),
                        10.0 / 3.6, // 30kph
                        vec![2, 4],
                        None,
                        None,
                    ),
                ),
                (
                    2,
                    Node::new(
                        2,
                        Point3::new(10.5, 1.0, 0.0),
                        5.0 / 3.6, // 30kph
                        vec![3],
                        None,
                        None,
                    ),
                ),
                (
                    3,
                    Node::new(
                        3,
                        Point3::new(11.0, 20.0, 0.0),
                        30.0 / 3.6, // 30kph
                        Vec::new(),
                        None,
                        None,
                    ),
                ),
                (
                    4,
                    Node::new(
                        4,
                        Point3::new(10.5, -1.0, 0.0),
                        5.0 / 3.6, // 30kph
                        vec![5],
                        None,
                        None,
                    ),
                ),
                (
                    5,
                    Node::new(
                        5,
                        Point3::new(11.0, -20.0, 0.0),
                        30.0 / 3.6, // 30kph
                        Vec::new(),
                        None,
                        None,
                    ),
                ),
            ]
            .into(),
        ));

        simulator.current_road_users.push(RoadUser::new(
            Point3::new(0.0, 0.0, 0.0),
            0.0,
            3.5,
            5.0,
            PI / 2.0,
            0,
            5,
            &simulator.road_network,
        ));

        let mut current_time = 0.0;
        const DELTA_TIME: f32 = 0.01;
        for i in 0..2000 {
            if i % 100 == 0 {
                println!("Current time: {current_time}");

                for user in simulator.current_road_users.iter() {
                    println!(
                        "  @{}, => {:?} @ {} m/s",
                        user.location(),
                        user.current_direction(),
                        user.current_speed()
                    );
                }
            }

            simulator.tick(DELTA_TIME);
            current_time += DELTA_TIME;

            if simulator.current_road_users.is_empty() {
                break;
            }
        }
    }
}
