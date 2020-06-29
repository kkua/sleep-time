use rust_embed::RustEmbed;
use std::borrow::Cow;
#[derive(RustEmbed)]
#[folder = "resources/assets/"]
pub struct Assets;

#[inline]
pub fn get(filename: &str) -> Option<Cow<'static, [u8]>> {
    Assets::get(filename)
}
