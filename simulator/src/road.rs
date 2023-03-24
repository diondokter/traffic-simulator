use nalgebra::{Point3, Vector3};
use std::collections::HashMap;

pub struct RoadNetwork {
    nodes: HashMap<u32, Node>,
}

impl RoadNetwork {
    pub fn new(
        nodes: HashMap<u32, Node>,
    ) -> Self {
        Self {
            nodes,
        }
    }

    pub fn find_node(&self, id: u32) -> &Node {
        self.nodes.get(&id).unwrap()
    }

    pub fn all_node_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.nodes.keys().copied()
    }
}

#[derive(Debug)]
pub struct Node {
    pub location: Point3<f32>, // 1 unit = 1 meter
    pub max_speed: f32, // m/s

    next_nodes: Vec<u32>,
    adjacent_node_right: Option<u32>,
    adjacent_node_left: Option<u32>,
}

impl Node {
    pub fn new(
        location: Point3<f32>,
        max_speed: f32,

        next_nodes: Vec<u32>,
        adjacent_node_right: Option<u32>,
        adjacent_node_left: Option<u32>,
    ) -> Self {
        Self {
            location,
            max_speed,

            next_nodes,
            adjacent_node_right,
            adjacent_node_left,
        }
    }

    pub fn next_nodes<'s, 'rn: 's>(
        &'s self,
        network: &'rn RoadNetwork,
    ) -> impl Iterator<Item = &'rn Node> + '_ {
        self.next_nodes.iter().map(move |id| network.find_node(*id))
    }

    pub fn adjacent_node_right<'rn>(&self, network: &'rn RoadNetwork) -> Option<&'rn Node> {
        self.adjacent_node_right.map(|id| network.find_node(id))
    }

    pub fn adjacent_node_left<'rn>(&self, network: &'rn RoadNetwork) -> Option<&'rn Node> {
        self.adjacent_node_left.map(|id| network.find_node(id))
    }

    /// The distance from this node to the given node
    pub fn distance_to(&self, other: &Node) -> f32 {
        self.vector_to(other).magnitude()
    }

    /// The full vector from this node to the given node
    pub fn vector_to(&self, other: &Node) -> Vector3<f32> {
        other.location - self.location
    }

    /// The direction (normalized vector) from this node to the given node
    pub fn direction_to(&self, other: &Node) -> Vector3<f32> {
        self.vector_to(other).normalize()
    }
}
