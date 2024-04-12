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
    pub mana: f32,
    pub mana_max: f32,
}

// Define the attacking component
#[derive(Component)]
pub struct Attack {
    pub velocity: f32,
    pub damage: f32,
}

// Define the player movement system
pub fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
    camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let (mut player, mut transform) = player_query.single_mut();

    let mut rotation_factor = 0.;
    let mut movement_factor = 0.;

    if keys.pressed(KeyCode::KeyW) {
        movement_factor += 1.;
    } else if keys.pressed(KeyCode::KeyS) {
        movement_factor -= 1.;
    }
    if keys.pressed(KeyCode::KeyA) {
        rotation_factor += 1.;
    } else if keys.pressed(KeyCode::KeyD) {
        rotation_factor -= 1.;
    }

    // Initialise the movement distance variable (to bring it into scope)
    let mut movement_distance: f32 = 0.;
    // Player is not dashing by default
    let mut is_dashing = false;

    // Dash on space key press if the player has the stamina
    if keys.just_pressed(KeyCode::Space) && player.stamina >= 1. {
        is_dashing = true;
        player.stamina -= 1.;
        movement_distance = 256.;
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

    camera_follow(
        camera_query,
        transform.translation.x,
        transform.translation.y,
    )
}

#[allow(clippy::type_complexity)]
pub fn player_attack(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Transform, &mut Player), With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let (transform, mut player) = player_query.single_mut();
    let attack_position = transform.translation + (transform.rotation * Vec3::Y * 100.);

    if keys.just_pressed(KeyCode::Enter) {
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("attacks/stone_cannon.png"),
                transform: Transform {
                    scale: Vec3::splat(0.3),
                    translation: attack_position,
                    rotation: transform.rotation,
                },
                ..default()
            })
            .insert(Attack {
                velocity: 20.,
                damage: 20.,
            });

        player.mana -= 1.;
    }
}

pub fn attack_movement(mut attack_query: Query<(&mut Transform, Option<&Attack>), With<Attack>>) {
    for (mut transform, attack) in attack_query.iter_mut() {
        if let Some(attack) = attack {
            let direction = transform.rotation * Vec3::Y;
            transform.translation += direction * attack.velocity;
        }
    }
}

// Function to make the camera follow the plaeyr
fn camera_follow(
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
    player_x: f32,
    player_y: f32,
) {
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_x;
    camera_transform.translation.y = player_y;
}

pub fn player_regen(mut player_query: Query<&mut Player, With<Player>>, time: Res<Time>) {
    let mut player = player_query.single_mut();
    if (player.stamina / player.stamina_max) < 1. {
        player.stamina += 0.1 * time.delta_seconds();
    }
}
