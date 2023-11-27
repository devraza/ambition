use bevy::{
    core_pipeline::{
        tonemapping::Tonemapping,
    },
    prelude::*,
};
use std::collections::HashMap;
use lazy_static::lazy_static;

// Create a map of the Hypernova colorscheme
lazy_static!{
    static ref HYPERNOVA: HashMap<&'static str, Color> = vec![
        ("BLACK", Color::hex("0d0d0f").unwrap()),
        ("DARK_GRAY", Color::hex("151517").unwrap()),
        ("GRAY", Color::hex("27272b").unwrap()),
        ("LIGHT_GRAY", Color::hex("454449").unwrap()),
        ("SUBTEXT", Color::hex("d9d0d7").unwrap()),
        ("WHITE", Color::hex("fefefa").unwrap()),
        ("RED", Color::hex("f06969").unwrap()),
        ("MAGENTA", Color::hex("e887bb").unwrap()),
        ("PURPLE", Color::hex("a292e8").unwrap()),
        ("BLUE", Color::hex("78b9c4").unwrap()),
        ("CYAN", Color::hex("7ee6ae").unwrap()),
        ("GREEN", Color::hex("91d65c").unwrap()),
        ("YELLOW", Color::hex("d9d564").unwrap()),
    ].iter().copied().collect();
}

fn main() {
    App::new()
        .insert_resource(ClearColor(HYPERNOVA.get("DARK_GRAY").copied().unwrap()))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, render_ui)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        }
    );
}

fn render_ui(
    server: Res<AssetServer>,
    mut commands: Commands
) {
    let bold_font: Handle<Font> = server.load("fonts/iosevka-comfy-bold.ttf");
    let regular_font: Handle<Font> = server.load("fonts/iosevka-comfy-regular.ttf");

    let text_style = TextStyle {
        font: bold_font,
        font_size: 60.0,
        color: HYPERNOVA.get("WHITE").copied().unwrap(),
    };
    let text_alignment = TextAlignment::Center;
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("translation", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
    )};
}
