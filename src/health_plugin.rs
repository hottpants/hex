use bevy::prelude::*;
use std::collections::{HashMap};

/**
# TLDR of how this shit works:

Just send a [`DamageInstance`] and if the thing has a [`HP`] component it'll take damage

# Slightly longer TLDR
there is a damage instance event setup by the plugin, which works as a queue for all instances of damage to be carried out that update tick

Player A damages thing B || Projectile Z realizes it collided with entity U || Death box intersects with Player L? 

"Send" a damage instance event.

```Rust
fn ouch (mut damage_ev: EventWriter<DamageInstance>) {
    damage_ev.send(DamageInstance(player_id, 3));
}
```

A billion events occur, they stack up, and next update tick this plugin takes care of reading/deleting all events sent and reducing the hp component's value

 **/

pub struct HealthDamagePlugin;

impl Plugin for HealthDamagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DamageInstance>()
            .add_systems(Update,
                         (
                             damage_player_tester,
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


// TODO: Delete/move both player component and damage_player_tester, they don't belong here
// This plugin should be as agnostic as possible: it should not care if the damaged or damager is 
// the player
#[derive(Component)]
pub struct Player;


// When E is pressed, damage the player by 3
pub fn damage_player_tester(input: Res<ButtonInput<KeyCode>>, mut damage_ev: EventWriter<DamageInstance>, query: Query<Entity, With<Player>>) {
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
