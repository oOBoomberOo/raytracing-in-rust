use anyhow::Context;
use serde::Deserialize;
use ultraviolet::Vec3;

use crate::camera::Camera;
use crate::light::directional::DirectionalLight;
use crate::light::point::PointLight;
use crate::light::Light;
use crate::model::plane::Plane;
use crate::model::sphere::Sphere;
use crate::model::Material;
use crate::world::{Entity, Sky, World};

pub fn parse_config(config: &str) -> anyhow::Result<(World, Camera)> {
    let result: WorldConfig = serde_yaml::from_str(config).context("parsing config file")?;

    let mut world = World::new();

    world.reflection_depth = result.reflection_depth;
    world.sky = result.sky;

    for light in result.light {
        world.lights.push(light.build());
    }

    for entity in result.entity {
        world.entities.push(entity.build());
    }

    let camera = result.camera.build();

    println!(
        "loaded world with {} lights, {} entities, and camera {:?}",
        world.lights.len(),
        world.entities.len(),
        camera
    );

    Ok((world, camera))
}

fn reflection_depth() -> usize {
    5
}

#[derive(Debug, Clone, Deserialize)]
struct WorldConfig {
    camera: CameraConfig,
    light: Vec<LightConfig>,
    entity: Vec<EntityConfig>,
    #[serde(default = "reflection_depth")]
    reflection_depth: usize,
    #[serde(default)]
    sky: Sky,
}

#[derive(Debug, Clone, Deserialize)]
struct CameraConfig {
    width: u32,
    height: u32,
    #[serde(default = "CameraConfig::fov")]
    fov: f32,
    #[serde(default)]
    origin: Vec3,
    #[serde(default = "CameraConfig::speed")]
    speed: f32,
}

impl CameraConfig {
    fn build(self) -> Camera {
        Camera {
            viewport_width: self.width as f32,
            viewport_height: self.height as f32,
            fov: Camera::compute_fov(self.fov),
            origin: self.origin,
            speed: self.speed,
        }
    }

    fn fov() -> f32 {
        60.0
    }

    fn speed() -> f32 {
        0.1
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum LightConfig {
    Directional(DirectionalLight),
    Point(PointLight),
}

impl LightConfig {
    fn build(self) -> Box<dyn Light> {
        match self {
            LightConfig::Directional(mut light) => {
                light.direction = light.direction.normalized();
                Box::new(light)
            }
            LightConfig::Point(light) => Box::new(light),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum EntityConfig {
    Sphere {
        material: Material,
        position: Vec3,
        radius: f32,
    },
    Plane {
        material: Material,
        position: Vec3,
        normal: Vec3,
    },
}

impl EntityConfig {
    fn build(self) -> Box<dyn Entity> {
        match self {
            EntityConfig::Sphere {
                position: center,
                radius,
                material,
            } => Box::new(Sphere {
                center,
                radius,
                material,
            }),

            EntityConfig::Plane {
                position,
                normal,
                material,
            } => Box::new(Plane {
                position,
                normal,
                material,
            }),
        }
    }
}
