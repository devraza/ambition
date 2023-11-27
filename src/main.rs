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
        .add_systems(
            Startup,
            (setup, render_ui)
        )
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
    let bold_font: Handle<Font> = server.load("fonts/VictorMono-Bold.otf");
    let regular_font: Handle<Font> = server.load("fonts/VictorMono-Regular.otf");

    let text_style = TextStyle {
        font: bold_font.clone(),
        font_size: 80.0,
        color: Color::WHITE,
    };
    commands
        .spawn(NodeBundle {
            style: Style {
                // fill the entire window
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(
                TextBundle::from_section("Ambition", text_style.clone())
                    .with_text_alignment(TextAlignment::Center)
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        margin: UiRect {
                            top: Val::Percent(10.),
                            ..default()
                        },
                        ..default()
                    }),
                );
        }
    );
}
