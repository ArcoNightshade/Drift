use bevy::prelude::*;
use bevy::render::texture::Image;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "embedded_assets/"]
pub struct EmbeddedAssets;

pub fn load_embedded_texture(
    path: &str,
    mut images: ResMut<Assets<Image>>,
) -> Handle<Image> {
    let bytes = EmbeddedAssets::get(path)
        .unwrap_or_else(|| panic!("Missing embedded asset: {}", path));

    let image = Image::from_buffer(
        &bytes.data,
        ImageType::Extension("png"),
        bevy::render::texture::ImageSampler::nearest(),
        true,
    )
    .expect("Failed to decode embedded PNG");

    images.add(image)
}
