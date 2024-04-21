use std::path::PathBuf;

use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Deserializer};
use serde::de::{Error, MapAccess, Visitor};

use crate::color::Color;
use crate::model::TextureCoord;

#[derive(Debug, Clone)]
pub enum Coloration {
    Color(Color),
    Texture(ImageTexture),
}

impl Coloration {
    pub fn get(&self, uv: &TextureCoord) -> Color {
        match self {
            Coloration::Color(color) => *color,
            Coloration::Texture(texture) => texture.get(uv),
        }
    }

    pub fn flat(hex: u32) -> Coloration {
        Coloration::Color(Color::from_hex(hex))
    }

    pub fn texture(scale: f32, path: impl Into<PathBuf>) -> Coloration {
        Coloration::Texture(ImageTexture::new(scale, path))
    }
}

impl<'de> serde::Deserialize<'de> for Coloration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match ColorationData::deserialize(deserializer)? {
            ColorationData::Color(c) => Coloration::Color(c),
            ColorationData::Texture { path, scale } => Coloration::texture(scale, path),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ColorationData {
    Color(Color),
    Texture { scale: f32, path: PathBuf },
}

impl Default for Coloration {
    fn default() -> Self {
        Coloration::flat(0x0A0A0A)
    }
}

#[derive(Debug, Clone)]
pub struct ImageTexture {
    texture: DynamicImage,
    scale: f32,
}

impl ImageTexture {
    pub fn new(scale: f32, path: impl Into<PathBuf>) -> ImageTexture {
        let path = path.into();
        let texture = image::open(&path).expect("open texture file");
        ImageTexture { texture, scale }
    }

    fn get(&self, uv: &TextureCoord) -> Color {
        let wrapped_x = wrap(uv.x / self.scale, self.texture.width());
        let wrapped_y = wrap(uv.y / self.scale, self.texture.height());

        let pixel = self.texture.get_pixel(wrapped_x, wrapped_y);
        Color::from(pixel)
    }
}

fn wrap(coord: f32, bound: u32) -> u32 {
    let signed_bound = bound as i32;
    let coord = coord * bound as f32;
    let wrapped = (coord as i32) % signed_bound;

    if wrapped < 0 {
        (wrapped + signed_bound) as u32
    } else {
        wrapped as u32
    }
}
