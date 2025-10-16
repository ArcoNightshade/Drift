use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AssetHandles {
    pub player_sprite: Option<Handle<Image>>,
}
