pub mod components;
pub mod systems;
pub mod states;
pub mod assets;

use bevy::prelude::*;
use components::{Player, Velocity};
use assets::load_embedded_texture;

pub fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2dBundle::default());

    // Load the embedded player sprite
    let texture_handle = load_embedded_texture("player.png", images);

    commands.spawn((
        SpriteBundle {
            texture: texture_handle,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            ..default()
        },
        Player,
        Velocity { x: 1.0, y: 1.0 },
    ));
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
