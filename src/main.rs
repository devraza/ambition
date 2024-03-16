use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*, window::*};

use bevy_egui::EguiPlugin;

use lazy_static::lazy_static;
use std::collections::HashMap;

// Load modules from other files
mod helpers;
use crate::helpers::*;
mod player;
use crate::player::*;
mod ui;
use crate::ui::*;

// Version information
const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKGNAME: &str = env!("CARGO_PKG_NAME");

// Create a map of the Hypernova colorscheme
lazy_static! {
    static ref KAGAYAKI: HashMap<&'static str, (u8, u8, u8)> = vec![
        ("BLACK", (13, 13, 15)),
        ("DARK_GRAY", (21, 21, 23)),
        ("GRAY", (39, 39, 43)),
        ("LIGHT_GRAY", (69, 68, 73)),
        ("SUBTEXT", (217, 208, 215)),
        ("WHITE", (236, 229, 234)),
        ("RED", (240, 105, 105)),
        ("MAGENTA", (232, 135, 187)),
        ("PURPLE", (162, 146, 232)),
        ("BLUE", (120, 175, 196)),
        ("CYAN", (127, 230, 174)),
        ("GREEN", (145, 214, 92)),
        ("YELLOW", (217, 213, 100)),
    ]
    .iter()
    .copied()
    .collect();
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: format!("{} {}", titlecase(PKGNAME), VERSION),
                    mode: WindowMode::Fullscreen,
                    ..default()
                }),
                ..default()
            }),
            EguiPlugin,
        ))
        .init_resource::<UiState>()
        .init_resource::<OpenWindows>()
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, (render_ui, movement, camera_follow, player_regen, attack))
        .run();
}

// Bevy engine setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("player/player-4x.png"),
        transform: Transform {
            scale: Vec3::splat(0.2),
            ..default()
        },
        ..default()
    });

    // Spawn the 2D camera
    commands.spawn(Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface,
        ..default()
    });

    // Spawn the player
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("player/player-4x.png"),
            transform: Transform {
                scale: Vec3::splat(0.2),
                ..default()
            },
            ..default()
        })
        .insert(Player {
            movement_speed: 512.,
            rotation_speed: f32::to_radians(360.),

            health: 1.,
            health_max: 10.,
            stamina: 1.,
            stamina_max: 10.,
        });
}
