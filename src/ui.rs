use bevy::prelude::*;

use bevy_egui::{egui, EguiContexts};

use crate::Player;
use crate::HYPERNOVA;

// Define UI resources
#[derive(Default, Resource)]
pub struct UiState {
    pub username: String,
    pub password: String,
}

#[derive(Default, Resource)]
pub struct OpenWindows {
    pub login_open: bool,
}

// On startup: setup some UI components
pub fn setup_ui(mut contexts: EguiContexts) {
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
pub fn render_ui(
    mut contexts: EguiContexts,
    mut windows: Query<&mut Window>,
    mut ui_state: ResMut<UiState>,
    mut open_windows: ResMut<OpenWindows>,
    mut player_query: Query<&mut Player, With<Player>>,
) {
    // Query the player information and put it into scope
    let player = player_query.single_mut();

    let window = windows.single_mut();
    let window_width = window.resolution.width();
    let window_height = window.resolution.height();

    let ctx = contexts.ctx_mut();

    let purple = HYPERNOVA.get("PURPLE").unwrap();
    let purple = egui::Color32::from_rgb(purple.0, purple.1, purple.2);

    let black = HYPERNOVA.get("BLACK").unwrap();
    let black = egui::Color32::from_rgb(black.0, black.1, black.2);

    egui::Window::new("Login")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0., 0.))
        .resizable(false)
        .title_bar(false)
        .open(&mut open_windows.login_open)
        .show(ctx, |ui| {
            ui.set_width(window_width / 3.);
            ui.set_height(window_height / 3.);

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                // Define spacing between items (widgets) and add manually add some space
                // between the window border and the heading
                ui.spacing_mut().item_spacing = egui::vec2(0., 10.);
                ui.spacing_mut().button_padding = egui::vec2(20., 10.);
                ui.add_space(window_height / 22.);

                ui.heading(egui::RichText::new("Login").size(30.).color(purple)); // The window 'title'

                // Manually add some space between the heading and the text inputs
                ui.add_space(window_height / 28.);

                // The text inputs
                egui::TextEdit::singleline(&mut ui_state.username)
                    .hint_text("Username")
                    .margin(egui::vec2(10., 10.))
                    .desired_width(window_width / 4.)
                    .show(ui);

                egui::TextEdit::singleline(&mut ui_state.password)
                    .password(true)
                    .margin(egui::vec2(10., 10.))
                    .hint_text("Password")
                    .desired_width(window_width / 4.)
                    .show(ui);

                // Manually add some space between the text inputs and the 'confirm' button
                ui.add_space(window_height / 26.);

                ui.add(egui::Button::new("Confirm").fill(black));

                // Manually add some space between the button and the bottom border of the
                // window...for scaling purposes
                ui.add_space(window_height / 22.);
            });
        });

    egui::CentralPanel::default()
        .frame(egui::containers::Frame {
            fill: egui::Color32::TRANSPARENT,
            inner_margin: egui::style::Margin::same(10.),
            ..default()
        })
        .show(ctx, |ui| {
            ui.visuals_mut().menu_rounding = egui::Rounding::ZERO;
            ui.visuals_mut().selection.bg_fill = purple;

            // Show player stats
            egui::Grid::new("Stats")
                .spacing(egui::Vec2::new(20., 10.))
                .show(ui, |ui| {
                    let health_bar = egui::widgets::ProgressBar::new(player.health)
                        .desired_width(window_width / 10.);
                    let mut stamina_bar = egui::widgets::ProgressBar::new(player.stamina)
                        .desired_width(window_width / 10.);

                    // Show the stamina bar to be empty if the player has no stamina
                    if player.stamina <= 0. {
                        stamina_bar =
                            stamina_bar.fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0));
                    };

                    // Show the progress bars!
                    ui.label(egui::RichText::new("Health").color(purple));
                    ui.add(health_bar);
                    ui.end_row();

                    ui.label(egui::RichText::new("Stamina").color(purple));
                    ui.add(stamina_bar);
                    ui.end_row();
                });
        });
}
