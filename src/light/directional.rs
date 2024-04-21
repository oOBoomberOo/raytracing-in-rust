use serde::Deserialize;
use ultraviolet::Vec3;

use crate::color::Color;
use crate::light::Light;
use crate::ray::{Intersectable, Intersection, Ray};
use crate::world::World;

#[derive(Debug, Clone, Deserialize)]
pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Color,
    pub intensity: f32,
}

impl Light for DirectionalLight {
    fn shading(&self, world: &World, intersection: &Intersection) -> Color {
        let light_direction = -self.direction;

        let shadow_ray = Ray::shadow(intersection, light_direction);

        let in_light = world.intersect(&shadow_ray).is_none();
        let intensity = if in_light { self.intensity } else { 0.0 };

        let light_power = intersection.contact.normal.dot(light_direction) * intensity;
        let light_reflected = intersection.contact.material.albedo / std::f32::consts::PI;
        let color = intersection.color() * self.color * light_power * light_reflected;
        color.clamp()
    }
}
