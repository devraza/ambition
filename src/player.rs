use bevy::prelude::*;

// Define the player component
#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
    pub rotation_speed: f32,

    pub health: f32,
    pub health_max: f32,
    pub stamina: f32,
    pub stamina_max: f32,

    pub defence: f32,
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
    } else if keys.pressed(KeyCode::S) {
        movement_factor -= 1.;
    } 

    if keys.pressed(KeyCode::A) {
        rotation_factor += 1.;
    } else if keys.pressed(KeyCode::D) {
        rotation_factor -= 1.;
    } 

    if keys.pressed(KeyCode::Space) {
        blink_factor += 4.;
    }

    if keys.pressed(KeyCode::Up) {
        transform.rotation = Quat::from_rotation_z((0_f32).to_radians());
        movement_factor = 1.;
    }
    if keys.pressed(KeyCode::Down) {
        transform.rotation = Quat::from_rotation_z((180_f32).to_radians());
        movement_factor = 1.;
    }
    if keys.pressed(KeyCode::Left) {
        transform.rotation = Quat::from_rotation_z((90_f32).to_radians());
        movement_factor = 1.;
    }
    if keys.pressed(KeyCode::Right) {
        transform.rotation = Quat::from_rotation_z((270_f32).to_radians());
        movement_factor = 1.;
    }

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

    // Update the player translation with the translation
    transform.translation += movement_direction * movement_distance;

    // Define the bounds of play (the window size)
    let window = windows.single_mut();
    let bounds = Vec3::from((
        Vec2::new(window.resolution.width(), window.resolution.height()) / 2.,
        0.,
    ));
    transform.translation = transform.translation.min(bounds).max(-bounds);
}
