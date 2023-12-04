use bevy::{prelude::*, window::*};

// Define the player component
#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

// Define the player movement system
pub fn movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut windows: Query<&mut Window>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (player, mut transform) = query.single_mut();

    let mut rotation_factor = 0.;
    let mut movement_factor = 0.;
    let mut blink_factor = 0.;

    if keys.pressed(KeyCode::W) {
        movement_factor += 1.;
    }
    if keys.pressed(KeyCode::S) {
        movement_factor -= 1.;
    }
    if keys.pressed(KeyCode::A) {
        rotation_factor += 1.;
    }
    if keys.pressed(KeyCode::D) {
        rotation_factor -= 1.;
    }
    if keys.pressed(KeyCode::Space) {
        blink_factor += 4.;
    }
    if keys.pressed(KeyCode::Space) && keys.just_released(KeyCode::Right) {
        blink_factor += 4.;
    };

    // Get the player's *forward* vector
    let movement_direction = transform.rotation * Vec3::Y;

    // Initialise the movement distance variable (to bring it into scope)
    let movement_distance: f32;

    if blink_factor == 0. {
        movement_distance = movement_factor * player.movement_speed * time.delta_seconds();
        // Change the player rotation around the Z-axis only if not blinking
        transform.rotate_z(rotation_factor * player.rotation_speed * time.delta_seconds());
    } else {
        movement_distance = blink_factor * player.movement_speed * 0.01;
    }

    // Create the translation using the movement direction and distance
    let translation_delta = movement_direction * movement_distance;
    // Update the player translation with the created translation
    transform.translation += translation_delta;

    // Define the bounds of play (the window size)
    let window = windows.single_mut();
    let bounds = Vec3::from((
        Vec2::new(window.resolution.width(), window.resolution.height()) / 2.,
        0.,
    ));
    transform.translation = transform.translation.min(bounds).max(-bounds);
}
