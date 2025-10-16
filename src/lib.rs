// Importing bevy & friends
use crate::states::AppState;
use bevy::prelude::*;
use bevy::state::app::AppExtStates;
use rust_embed::RustEmbed;
use tracing_subscriber::{EnvFilter, fmt};

// Making components public
pub mod components;
pub mod resources;
pub mod states;
pub mod systems;

// Handling asset embed into binary
#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct DriftAssets;

pub struct DriftGame;

// Making a basic window
impl DriftGame {
    pub fn run() -> AppExit {
        init_tracing();

        let mut app = App::new();

        app.insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.07)))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Drift".into(),
                    resolution: (960.0, 540.0).into(),
                    resizable: true,
                    ..default()
                }),
                ..default()
            }))
            .add_message::<AppState>()
            .add_systems(Startup, systems::setup_camera)
            .run()
    }
}

fn init_tracing() {
    let filter = EnvFilter::from_default_env()
        .add_directive("wgpu=warn".parse().unwrap())
        .add_directive("bevy_render=warn".parse().unwrap())
        .add_directive("drift=info".parse().unwrap());

    fmt().with_env_filter(filter).init();
}
