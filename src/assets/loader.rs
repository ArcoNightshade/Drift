use bevy::prelude::*;
use bevy::render::texture::{ImageSamplerDescriptor, ImageType};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "embedded_assets/"]
pub struct EmbeddedAssets;

pub fn load_embedded_texture(
    path: &str,
    mut images: ResMut<Assets<Image>>,
) -> Handle<Image> {
    let bytes = EmbeddedAssets::get(path).expect("Failed to load embedded asset");
    let image = Image::from_buffer(
        &bytes.data,
        ImageType::Extension("png"),
        ImageSamplerDescriptor::nearest(),
        true,
        ImageSamplerDescriptor::nearest(),
        bevy::render::render_asset::RenderAssetUsages::all(),
    ).expect("Failed to decode embedded image");
    images.add(image)
}
