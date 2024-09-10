#![windows_subsystem = "windows"]
mod character_plugin;
mod health_plugin;
mod debug;

use bevy::{
    prelude::*,
    render::{render_asset::RenderAssetUsages,render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle
};

use avian2d::{math::*, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_egui::egui::Color32;

use character_plugin::*;
use health_plugin::{HealthDamagePlugin, HP, Player};
use debug::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "HEX".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(
            (
                EguiPlugin,
                PhysicsPlugins::default().with_length_unit(10.0),
                
                DebugPlugin,
                CharacterControllerPlugin,
                HealthDamagePlugin
            )
        )
        .insert_resource(Gravity(Vector::new(0.0,-1000.0)))
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Startup, setup)
        .add_systems(Update, ui_example_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {

    // Player
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Capsule2d::new(12.5, 20.0)).into(),
            material: materials.add(Color::srgb(0.2, 0.7, 0.9)),
            transform: Transform::from_xyz(0.0, -100.0, 0.0),
            ..default()
        },
        CharacterControllerBundle::new(Collider::capsule(12.5, 20.0), Vector::NEG_Y * 1500.0)
            .with_movement(1250.0, 0.92, 400.0, (30.0 as Scalar).to_radians()),
        Player,
        HP(100)
    )
    
    
    );

    // Default Blender Cube
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 0.4, 0.7),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_xyz(50.0, -100.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::rectangle(30.0, 30.0),
    ));

    // Platforms
    spawn_platform(&mut commands, Vec3::new(0.0, -175.0, 0.0), Vec2::new(1100.0, 50.0));
    spawn_platform(&mut commands, Vec3::new(175.0, -35.0, 0.0), Vec2::new(300.0, 25.0));
    spawn_platform(&mut commands, Vec3::new(-175.0, 0.0, 0.0), Vec2::new(300.0, 25.0));
    spawn_platform(&mut commands, Vec3::new(475.0, -110.0, 0.0), Vec2::new(150.0, 80.0));
    spawn_platform(&mut commands, Vec3::new(-475.0, -110.0, 0.0), Vec2::new(150.0, 80.0));

    // Ramps (NOTE: JANK. Hand built using vertices)
    let mut ramp_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    ramp_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[-125.0, 80.0, 0.0], [-125.0, 0.0, 0.0], [125.0, 0.0, 0.0]],
    );

    let ramp_collider = Collider::triangle(
        Vector::new(-125.0, 80.0),
        Vector::NEG_X * 125.0,
        Vector::X * 125.0,
    );

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(ramp_mesh).into(),
            material: materials.add(Color::srgb(0.4, 0.4, 0.5)),
            transform: Transform::from_xyz(-275.0, -150.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        ramp_collider,
    ));

    let mut ramp_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    ramp_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[20.0, -40.0, 0.0], [20.0, 40.0, 0.0], [-20.0, -40.0, 0.0]],
    );

    let ramp_collider = Collider::triangle(
        Vector::new(20.0, -40.0),
        Vector::new(20.0, 40.0),
        Vector::new(-20.0, -40.0),
    );

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(ramp_mesh).into(),
            material: materials.add(Color::srgb(0.4, 0.4, 0.5)),
            transform: Transform::from_xyz(380.0, -110.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        ramp_collider,
    ));

    // Camera
    commands.spawn(Camera2dBundle::default());

    // Fix UI
    contexts.ctx_mut().set_visuals(egui::Visuals {
        panel_fill: Color32::TRANSPARENT,
        ..default()
    } );

}

fn spawn_platform (commands: &mut Commands, loc: Vec3, size: Vec2) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.7, 0.7, 0.8),
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(loc),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(size.x as Scalar, size.y as Scalar),
    ));
}


fn ui_example_system(mut contexts: EguiContexts) {

    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("Mission: Jump");
        });
    });
}