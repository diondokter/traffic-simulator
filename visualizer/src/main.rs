use std::f32::consts::PI;

use bevy::input::common_conditions::input_toggle_active;
use traffic_simulator::{Simulator, user::RoadUser, road::{RoadNetwork, self}};
use nalgebra::{Point3, Vector3};
use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    const TIME_STEP: f32 = 1.0 / 60.0;
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<ResSim>()
        .add_startup_system(setup)
        .add_system(tick_simulation)
        .add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape))
        )
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .run();
}



#[derive(Component)]
struct Ru(u32);

#[derive(Resource)]
struct ResSim(Simulator);

impl FromWorld for ResSim {
    fn from_world(world: &mut World) -> Self {
        ResSim(Simulator::new(RoadNetwork::new(
            [
                (
                    0,
                    road::Node::new(
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
                    road::Node::new(
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
                    road::Node::new(
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
                    road::Node::new(
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
                    road::Node::new(
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
                    road::Node::new(
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
        )))
    }
}

fn tick_simulation(
    mut commands: Commands,
    mut simulator: Res<ResSim>,
    mut ru_query: Query<(&Transform, Ru)>,
) {
    let (transform, id) = ru_query.single_mut();
    let rn = simulator.0.current_road_users();
    rn.iter().find(|ru| ru.id == id);
    ru_transform = Transform::from_xyz(ru.location().x, ru.location().z, ru.location().y);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut sim: ResMut<ResSim>,
) {
    let mut simulator = &mut sim.0;
    simulator.add_manual_road_users(RoadUser::new(
        Point3::new(0.0, 0.0, 0.0),
        0.0,
        3.5,
        5.0,
        PI / 2.0,
        0,
        5,
        &simulator.road_network(),
    ));
    
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(20.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(500.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // Origin marker
    /*
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
     */      
    let rn = simulator.road_network();

    rn.all_node_ids().for_each(|node_id| {
        let node = rn.find_node(node_id);
        draw_node_marker(&mut commands, &mut meshes, &mut materials, &node);
        node.next_nodes(rn).for_each(|next_node|{
            let from = node;
            let to = next_node;
            draw_road(&mut commands, &mut meshes, &mut materials, &from, &to);
            draw_node_marker(&mut commands, &mut meshes, &mut materials, &next_node);
        });
    });

    let road_users = simulator.current_road_users();
    road_users.iter().for_each(|ru| {
        
        commands.spawn((PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
            transform: Transform::from_xyz(ru.location().x, ru.location().z, ru.location().y),
            ..default()
        }, Ru));
        
    });
    
    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb_u8(201, 188, 164),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });
}

fn draw_road(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    from: &road::Node,
    to: &road::Node,
) {
    let vec_to: Vector3<f32> = from.vector_to(to);
    let road_width = 0.6;
    let road_thickness = 0.05;
    let road_length = vec_to.magnitude();
    let road_box = shape::Box::new(road_width, road_thickness, road_length);
    let move_vec = vec_to * 0.5;
    let translation = from.location() + move_vec;

    let transform = Transform::from_xyz(translation.x, translation.z, translation.y)
        .looking_at((to.location().x, to.location().z, to.location().y).into(), Vec3::Y);

    // Road
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(road_box)),
        material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
        transform,
        ..default()
    });    
}

fn draw_node_marker(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    node: &road::Node
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(node.location().x, node.location().z, node.location().y),
        ..default()
    });
}
