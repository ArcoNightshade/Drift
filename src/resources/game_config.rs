use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameConfig {
    pub debug_mode: bool,
}
