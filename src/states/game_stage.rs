use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameStage {
    pub current: u32,
}
