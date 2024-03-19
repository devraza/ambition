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
pub fn movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
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
}

pub fn attack(
    keys: Res<ButtonInput<KeyCode>>,
    mut set: ParamSet<(
        Query<&mut Transform, With<Attack>>,
        Query<&Transform, With<Player>>
    )>,
    mut player_query: Query<&mut Player>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut player = player_query.single_mut();

    for player_transform in set.p1().iter_mut() {
        let attack_position = player_transform.translation + ((player_transform.rotation * Vec3::Y) * 100.);

        if keys.just_pressed(KeyCode::Enter) {
     commands
        .spawn(SpriteBundle {
            texture: asset_server.load("attacks/stone_cannon.png"),
            transform: Transform {
                scale: Vec3::splat(0.3),
                translation: attack_position,
                rotation: player_transform.rotation,
            },
            ..default()
        })
        .insert(Attack {
            velocity: 10.,
            damage: 20.,
        });

        player.mana -= 1.;
    }
    }

    for mut attack_transform in set.p0().iter_mut() {
        let direction = attack_transform.rotation * Vec3::Y;
        attack_transform.translation += direction * 20.;
    }
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
    if (player.stamina / player.stamina_max) < 1. {
        player.stamina += 0.1 * time.delta_seconds();
    }
}
