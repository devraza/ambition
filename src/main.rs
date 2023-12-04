use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*, window::*, winit::WinitSettings};

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
    static ref HYPERNOVA: HashMap<&'static str, (u8, u8, u8)> = vec![
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
        // Only run the app when there is user input, reducing resource usage
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, (render_ui, movement))
        .run();
}

// Bevy engine setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface,
        ..default()
    });
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player/player.png"),
            ..default()
        },
        Player {
            movement_speed: 1024.,
            rotation_speed: f32::to_radians(360.),
        },
    ));
}
