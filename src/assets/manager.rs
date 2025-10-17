use bevy::prelude::*;
use crate::assets::loader::load_embedded_texture;

#[derive(Resource)]
pub struct GameAssets {
    pub player: Handle<Image>,
}

#[derive(Debug)]
pub enum GameAsset {
    Player,
}

impl GameAsset {
    pub fn path(&self) -> &'static str {
        match self {
            GameAsset::Player => "player.png",
        }
    }
}

pub fn load_game_assets(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let player = load_embedded_texture(GameAsset::Player.path(), images);
    commands.insert_resource(GameAssets { player });
}
