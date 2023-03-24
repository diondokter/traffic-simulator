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
        
    }
}
