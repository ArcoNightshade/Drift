use bevy::prelude::*;
use bevy::render::camera::Camera2dBundle;

pub mod assets;
pub mod states;

use assets::manager::load_game_assets;
use states::app_state::AppState;

pub fn run() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.07)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Drift".into(),
                resolution: (960.0, 540.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_systems(Startup, (setup_camera, load_game_assets))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
