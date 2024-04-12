use bevy::prelude::*;
use std::f32::consts::PI;

use crate::player::*;

// Define the enemy component
#[derive(Component)]
pub struct Enemy {
    pub name: String,
    pub movement_speed: f32,
    /*
    pub rotation_speed: f32,

    pub health: f32,
    pub health_max: f32,
    pub stamina: f32,
    pub stamina_max: f32,
    pub mana: f32,
    pub mana_max: f32,
    */
}

// Define the enemy movement system
#[allow(clippy::type_complexity)]
pub fn enemy_movement(
    time: Res<Time>,
    mut set: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<Enemy>>,
    )>,
    mut enemy_query: Query<&Enemy>,
) {
    let enemy = enemy_query.single_mut();

    // Bring the player translation into scope
    let mut player_translation = Vec3::ZERO;
    for player_transform in set.p0().iter_mut() {
        player_translation = player_transform.translation;
    }

    // Get the enemy's movement distance (based on movement speed)
    let movement_distance = enemy.movement_speed * time.delta_seconds();

    for mut enemy_transform in set.p1().iter_mut() {
        // Calculate the angle from the enemy to the player's position
        let difference = player_translation - enemy_transform.translation;
        let angle = difference.y.atan2(difference.x) - PI / 2.; // Subtract PI/2. to orient the enemy correctly

        // Return a Quat for the enemy's movement direction based on the angle
        let movement_direction = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);

        // Update the rotation and translation of the enemy
        enemy_transform.rotation = movement_direction;
        enemy_transform.translation += movement_direction * Vec3::Y * movement_distance;
    }
}

pub fn change_enemy_color(mut query: Query<&mut Sprite, With<Enemy>>) {
    for mut sprite in query.iter_mut() {
        sprite.color = Color::rgb(0.5, 1.0, 0.5);
    }
}
