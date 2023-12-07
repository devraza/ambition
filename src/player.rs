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
}

// Define the player movement system
pub fn movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
) {
    let (mut player, mut transform) = player_query.single_mut();

    let mut rotation_factor = 0.;
    let mut movement_factor = 0.;

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

    // Initialise the movement distance variable (to bring it into scope)
    let mut movement_distance: f32 = 0.;
    // Player is not dashing by default
    let mut is_dashing = false;

    // Dash on space key press if the player has the stamina
    if keys.just_pressed(KeyCode::Space) && player.stamina >= 0.3 {
        is_dashing = true;
        player.stamina -= 0.3;
        movement_distance = 256.;
    }

    if keys.pressed(KeyCode::Left) {
        transform.rotation = Quat::from_rotation_z((90_f32).to_radians());
        movement_factor = 1.;
    } else if keys.pressed(KeyCode::Right) {
        transform.rotation = Quat::from_rotation_z((270_f32).to_radians());
        movement_factor = 1.;
    }

    if keys.pressed(KeyCode::Up) {
        transform.rotation = Quat::from_rotation_z((0_f32).to_radians());
        movement_factor = 1.;
    } else if keys.pressed(KeyCode::Down) {
        transform.rotation = Quat::from_rotation_z((180_f32).to_radians());
        movement_factor = 1.;
    }

    // Get the player's *forward* vector
    let movement_direction = transform.rotation * Vec3::Y;

    if !is_dashing {
        movement_distance = movement_factor * player.movement_speed * time.delta_seconds();
        // Change the player rotation around the Z-axis only if not dashing
        transform.rotate_z(rotation_factor * player.rotation_speed * time.delta_seconds());
    }

    // Update the player translation with the translation
    transform.translation += movement_direction * movement_distance;
}

// Function to make the camera follow the plaeyr
pub fn camera_follow(
    mut player: Query<(&Player, &mut Transform)>,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let (_, transform) = player.single_mut();
    let pos = transform.translation;

    for mut camera_transform in &mut cameras {
        camera_transform.translation.x = pos.x;
        camera_transform.translation.y = pos.y;
    }
}

pub fn player_regen(mut player_query: Query<&mut Player, With<Player>>, time: Res<Time>) {
    let mut player = player_query.single_mut();
    println!("{}", player.stamina);
    if player.stamina < 1. {
        player.stamina += 0.1 * time.delta_seconds();
    }
}
