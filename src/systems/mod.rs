pub mod movement;
pub mod orbital_decay;
pub mod repair;

use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub use movement::*;
pub use orbital_decay::*;
pub use repair::*;
