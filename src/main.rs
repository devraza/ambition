use bevy::{
    core_pipeline::{
        tonemapping::Tonemapping,
    },
    window::*,
    prelude::*,
    winit::WinitSettings,
};
use std::collections::HashMap;
use lazy_static::lazy_static;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKGNAME: &str = env!("CARGO_PKG_NAME");

// Create a map of the Hypernova colorscheme
lazy_static!{
    static ref HYPERNOVA: HashMap<&'static str, Color> = vec![
        ("BLACK", Color::hex("0d0d0f").unwrap()),
        ("DARK_GRAY", Color::hex("151517").unwrap()),
        ("GRAY", Color::hex("27272b").unwrap()),
        ("LIGHT_GRAY", Color::hex("454449").unwrap()),
        ("SUBTEXT", Color::hex("d9d0d7").unwrap()),
        ("WHITE", Color::hex("ece5ea").unwrap()),
        ("RED", Color::hex("f06969").unwrap()),
        ("MAGENTA", Color::hex("e887bb").unwrap()),
        ("PURPLE", Color::hex("a292e8").unwrap()),
        ("BLUE", Color::hex("78b9c4").unwrap()),
        ("CYAN", Color::hex("7ee6ae").unwrap()),
        ("GREEN", Color::hex("91d65c").unwrap()),
        ("YELLOW", Color::hex("d9d564").unwrap()),
    ].iter().copied().collect();
}

fn titlecase(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn main() {
    App::new()
        // Only run the app when there is user input, significantly reducing CPU/GPU usage
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: format!("{} {}", titlecase(PKGNAME), VERSION).into(),
                    mode: WindowMode::Fullscreen,
                    ..default()
                }),
                ..default()
            })
        )
        .add_systems(
            Startup,
            (setup, setup_ui)
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

fn setup_ui(
    server: Res<AssetServer>,
    mut commands: Commands
) {
    let bold_font: Handle<Font> = server.load("fonts/VictorMono-Bold.otf");
    let regular_font: Handle<Font> = server.load("fonts/VictorMono-Regular.otf");

    let title_style = TextStyle {
        font: bold_font.clone(),
        font_size: 70.,
        color: HYPERNOVA.get("WHITE").copied().unwrap(),
    };
    let login_style = TextStyle {
        font: bold_font.clone(),
        font_size: 30.,
        color: HYPERNOVA.get("PURPLE").copied().unwrap(),
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(HYPERNOVA.get("DARK_GRAY").copied().unwrap()),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section("Ambition", title_style.clone())
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    margin: UiRect {
                        top: Val::Vh(10.),
                        ..default()
                    },
                    ..default()
                })
            );
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        width: Val::Percent(50.),
                        height: Val::Percent(40.),
                        margin: UiRect {
                            top: Val::Vh(30.),
                            ..default()
                        },
                        padding: UiRect {
                            top: Val::Percent(2.),
                            ..default()
                        },
                        ..default()
                    },
                    background_color: BackgroundColor(HYPERNOVA.get("GRAY").copied().unwrap()),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section("Login", login_style.clone())
                        .with_text_alignment(TextAlignment::Center)
                        .with_style(Style {
                            position_type: PositionType::Absolute,
                            ..default()
                        }));
                });
        });
}
