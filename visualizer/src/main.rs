use std::collections::HashMap;

use bevy::input::common_conditions::input_toggle_active;
use traffic_simulator::{Simulator, road::{RoadNetwork, self}};
use nalgebra::{Point3, Vector3};
use bevy::{render::camera::ScalingMode, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape))
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 5.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    let mut nodes = HashMap::new();
    nodes.insert(0, road::Node::new(
        Point3::new(0.0, 0.0, 0.0),
        8.3,
        vec![1],
        None,
        None,
        
    ));

    nodes.insert(1, road::Node::new(
        Point3::new(5.0, 3.0, 0.0),
        8.3,
        vec![1],
        None,
        None,
        
    ));

    let rn = RoadNetwork::new(nodes);
    //let sim = Simulator::new(rn);
    
    let node = rn.find_node(0);
    let node2 = rn.find_node(1);
    let vec_to: Vector3<f32> = node.vector_to(node2);

    let road_width = 3.0;

    let test = vec_to * 0.5;
    
    let transform = Transform::from_xyz(node.location.x, node.location.z, node.location.y)
        .with_translation(Vec3::from_array([test.x, test.z, test.y]))
        .looking_at((node2.location.x, node2.location.z, node2.location.y).into(), Vec3::new(0.0, 1.0, 0.0));
        
    let road_thickness = 0.05;
    let road_length = vec_to.magnitude();
    let road_box = shape::Box::new(road_width, road_thickness, road_length);
    /*
    let road_box = shape::Box {
        min_x: node.location.x,
        max_x: node2.location.x,
        min_y: node.location.z,
        max_y: node2.location.z,
        min_z: node.location.y,
        max_z: node2.location.y,
    };
    */
    // Road
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(road_box)),
        material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
        transform,
        ..default()
    });

    
    // cubes
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(node.location.x, node.location.z, node.location.y),
        ..default()
    });

    // cubes
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(node2.location.x, node2.location.z, node2.location.y),
        ..default()
    });
    
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });
}
