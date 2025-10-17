use bevy::prelude::*;
use crate::assets::load_embedded_texture;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameAsset {
    Player,
    Tile,
}

impl GameAsset {
    pub fn path(&self) -> &'static str {
        match self {
            GameAsset::Player => "player.png",
            GameAsset::Tile => "tile.png",
        }
    }
}

#[derive(Resource)]
pub struct AssetHandles {
    pub player: Handle<Image>,
    pub tile: Handle<Image>,
}

pub fn load_all_assets(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let player = load_embedded_texture(GameAsset::Player.path(), images.as_mut());
    let tile = load_embedded_texture(GameAsset::Tile.path(), images);

    commands.insert_resource(AssetHandles { player, tile });
}
