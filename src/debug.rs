use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crate::health_plugin::{DamageInstance, Player};

/**
Use for random types/systems/events that are needed for testing/building things that are not permeate
 **/

pub struct DebugPlugin;


impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            shoot_on_press,
            onhit_damage),
        );
    }
}
#[derive(Component)]
pub struct Damager(pub i32);
fn shoot_on_press(input: Res<ButtonInput<KeyCode>>, mut commands: Commands,mut meshes: ResMut<Assets<Mesh>>,
                  mut materials: ResMut<Assets<ColorMaterial>>) {
    if input.just_pressed(KeyCode::KeyQ) {
        println!("fired");
        commands.spawn((

            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle { radius: 25.0 })),
                material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
                transform: Transform::from_xyz(100.0, -100.0, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::circle(25.0),
            Damager(5)
        )
        );
    }
}

fn onhit_damage(input: Res<ButtonInput<KeyCode>>, mut damage_ev: EventWriter<DamageInstance>, query: Query<(&Damager, &CollidingEntities)>) {
    query.iter().for_each(|(Damager(damage), colliding_entities)| {damage_ev.send_batch(colliding_entities.iter().map(|t| {DamageInstance(*t, *damage)}));});
}

/*
fn print_collisions(mut collision_event_reader: EventReader<Collision>) {
    return;
    for Collision(contacts) in collision_event_reader.read() {
        println!(
            "Entities {:?} and {:?} are colliding",
            contacts.entity1,
            contacts.entity2,
        );
    }
}

 */