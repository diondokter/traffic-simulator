use std::fmt::Debug;

pub trait TrafficLight: Debug {
    fn node(&self) -> u32;
    fn tick(&mut self, current_time: f32);
    fn get_state(&self) -> TrafficLightState;
}

#[derive(Debug)]
pub struct TimedTrafficLight {
    node: u32,
    current_state: TrafficLightState,
    schema: Vec<(f32, TrafficLightState)>,
}

impl TimedTrafficLight {
    pub fn new(node: u32, schema: Vec<(f32, TrafficLightState)>) -> Self {
        Self {
            node,
            current_state: schema.first().unwrap().1,
            schema,
        }
    }
}

impl TrafficLight for TimedTrafficLight {
    fn node(&self) -> u32 {
        self.node
    }

    fn tick(&mut self, current_time: f32) {
        let total_schema_time: f32 = self.schema.iter().map(|(duration, _)| *duration).sum();
        let current_time_in_schema = current_time % total_schema_time;

        let mut current_test_time = 0.0;

        for schema_item in self.schema.iter() {
            if current_time_in_schema <= current_test_time + schema_item.0 {
                self.current_state = schema_item.1;
                break;
            } else {
                current_test_time += schema_item.0;
            }
        }
    }

    fn get_state(&self) -> TrafficLightState {
        self.current_state
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrafficLightState {
    Red,
    Orange,
    Green,
}
