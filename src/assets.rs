use rust_embed::RustEmbed;
use std::borrow::Cow;
#[derive(RustEmbed)]
#[folder = "resources/assets/"]
pub struct Assets;

#[inline]
pub fn get(filename: &str) -> Option<rust_embed::EmbeddedFile> {
    Assets::get(filename)
}
