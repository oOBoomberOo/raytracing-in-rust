use crate::color::Color;
use crate::texture::Coloration;
use serde::Deserialize;
use ultraviolet::{Vec2, Vec3};

pub mod plane;
pub mod sphere;

pub trait Texture {
    fn texture_coord(&self, contact: Vec3) -> TextureCoord;
}

#[derive(Debug, Clone, Deserialize)]
pub struct Material {
    #[serde(default)]
    pub color: Coloration,
    #[serde(default = "Material::default_albedo")]
    pub albedo: f32,
    #[serde(default)]
    pub surface: SurfaceType,
}

impl Material {
    pub fn color(&self, coord: TextureCoord) -> Color {
        self.color.get(&coord)
    }
    
    fn default_albedo() -> f32 {
        0.5
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Coloration::Color(Color::new(1.0, 0.0, 0.0)),
            albedo: 0.0,
            surface: SurfaceType::Diffuse,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(untagged)]
pub enum SurfaceType {
    #[default]
    Diffuse,
    Reflective { reflectivity: f32 }
}

pub type TextureCoord = Vec2;
