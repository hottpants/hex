use bevy::prelude::*;
use std::collections::{HashMap};

pub struct HealthDamagePlugin;

impl Plugin for HealthDamagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DamageInstance>()
            .add_systems(Update,
                         (
                             damage,
                             take_damage
                         ),
            )
        ;
    }
}

#[derive(Component)]
pub struct HP(i32);

impl Default for HP {
    fn default() -> Self {
        HP(10)
    }
}

#[derive(Event)]
pub struct DamageInstance(Entity, i32);

#[derive(Component)]
pub struct Player;


// When E is pressed, damage the player by 3
pub fn damage(input: Res<ButtonInput<KeyCode>>, mut damage_ev: EventWriter<DamageInstance>, query: Query<Entity, With<Player>>) {
    if input.pressed(KeyCode::KeyE) {
        let player_id = query.get_single().expect("There was an error assuming only one entity had the player component");
        damage_ev.send(DamageInstance(player_id, 3));
        println!("Damage event sent to {player_id} for 3 damage");
    }
}

pub fn take_damage(mut damage_ev: EventReader<DamageInstance>, mut query: Query<(Entity, &mut HP)>) {

    // This is rough, prob tanks frame rate. TODO: !OPTIMIZATION Don't make a hash map every frame
    let a: HashMap<&Entity, i32> = damage_ev.read().map(|DamageInstance(e, d)| { (e, *d) }).collect();

    query.par_iter_mut().for_each(|(entity, mut hp)| {
        if let Some(damage) = a.get(&entity) {
            hp.0 -= damage;
            println!("{entity} took {damage} damage. HP {} -> {}", hp.0 + damage, hp.0);
        }
    });
}

// What we did together

// use bevy::prelude::*;
//
// pub struct HealthPlugin;
//
// impl Plugin for HealthPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Update, (print_hp, take_dmg, kill_player));
//     }
// }
//
// #[derive(Component)]
// pub struct Health {
//     health: i32
// }
//
// impl Default for Health {
//     fn default() -> Self {
//         Health
//         {
//             health: 100,
//         }
//     }
// }
//
//
// fn take_dmg(mut query: Query<(&mut Health, &Player)>) {
//
//     for (mut health, _ ) in &mut query {
//         health.health -= 1;
//     }
// }
//
// fn kill_player(mut commands: Commands, mut query: Query<(Entity, &Health, &Player)>) {
//
//     for (balls, health, _ ) in &mut query {
//         if health.health <= 0 {
//             commands.entity(balls).despawn();
//         }
//
//     }
// }
// fn print_hp(mut query: Query<(&Health, &Player)>) {
//
//     for (health, _ ) in &mut query {
//         eprintln!("Adamballs has {} HP.", health.health);
//     }
// }
//
// #[derive(Component)]
// pub struct Player;
//
//
