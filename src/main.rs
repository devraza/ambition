use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*, window::*};

use bevy_egui::{egui, EguiContexts, EguiPlugin};

use lazy_static::lazy_static;
use std::collections::HashMap;

mod helpers;
use crate::helpers::*;

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
        ("CYAN", (126, 230, 174)),
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
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, render_ui)
        .run();
}

// Bevy engine setup
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface,
        ..default()
    });
}

// On startup: setup some UI components
fn setup_ui(mut contexts: EguiContexts) {
    // Set Victor Mono as the default custom font
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "victor_mono".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/VictorMono-Regular.otf")),
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "victor_mono".to_owned());
    contexts.ctx_mut().set_fonts(fonts);
}

// On update: render the UI
fn render_ui(mut contexts: EguiContexts, mut windows: Query<&mut Window>) {
    let window = windows.single_mut();
    let window_width = window.resolution.width();
    let window_height = window.resolution.height();

    egui::Window::new("Login")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0., 0.))
        .resizable(false)
        .title_bar(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.set_width(window_width / 2.5);
            ui.set_height(window_height / 3.);

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                let purple = HYPERNOVA.get("PURPLE").unwrap();
                let purple = egui::Color32::from_rgb(purple.0, purple.1, purple.2);

                ui.heading(egui::RichText::new("Login").size(30.).color(purple));
            });
        });
}
