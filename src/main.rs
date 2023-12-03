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
        .init_resource::<UiState>()
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, render_ui)
        .run();
}

#[derive(Default, Resource)]
struct UiState {
    username: String,
    password: String,
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
fn render_ui(mut contexts: EguiContexts, mut windows: Query<&mut Window>, mut ui_state: ResMut<UiState>) {
    let window = windows.single_mut();
    let window_width = window.resolution.width();
    let window_height = window.resolution.height();

    egui::Window::new("Login")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0., 0.))
        .resizable(false)
        .title_bar(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.set_width(window_width / 3.);
            ui.set_height(window_height / 3.);

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                let purple = HYPERNOVA.get("PURPLE").unwrap();
                let purple = egui::Color32::from_rgb(purple.0, purple.1, purple.2);

                let black = HYPERNOVA.get("BLACK").unwrap();
                let black = egui::Color32::from_rgb(black.0, black.1, black.2);

                // Define spacing between items (widgets) and add manually add some space
                // between the window border and the heading
                ui.spacing_mut()
                    .item_spacing = egui::vec2(0., 10.);
                ui.spacing_mut()
                    .button_padding = egui::vec2(20., 10.);
                ui.add_space(window_height / 22.);

                ui.heading(egui::RichText::new("Login").size(30.).color(purple)); // The window 'title'

                // Manually add some space between the heading and the text inputs
                ui.add_space(window_height / 28.);

                // The text inputs
                let username = egui::TextEdit::singleline(&mut ui_state.username)
                    .hint_text("Username")
                    .margin(egui::vec2(10., 10.))
                    .desired_width(window_width / 4.)
                    .show(ui);

                let password = egui::TextEdit::singleline(&mut ui_state.password)
                    .password(true)
                    .margin(egui::vec2(10., 10.))
                    .hint_text("Password")
                    .desired_width(window_width / 4.)
                    .show(ui);

                // Manually add some space between the text inputs and the 'confirm' button
                ui.add_space(window_height / 26.);

                let button = ui.add(egui::Button::new("Confirm")
                    .fill(black));

                // Manually add some space between the button and the bottom border of the
                // window...for scaling purposes
                ui.add_space(window_height / 22.);

            });
        });
}
