use serde::Deserialize;
use ultraviolet::Vec3;
use crate::color::Color;
use crate::light::Light;
use crate::ray::{Intersectable, Intersection, Ray};
use crate::world::World;

#[derive(Debug, Clone, Deserialize)]
pub struct PointLight {
    position: Vec3,
    color: Color,
    intensity: f32,
}

impl Light for PointLight {
    fn shading(&self, world: &World, intersection: &Intersection) -> Color {
        let light_direction = (self.position - intersection.hit).normalized();

        let distance = (self.position - intersection.hit).mag_sq();
        let intensity = self.intensity / (4.0 * std::f32::consts::PI * distance);

        let in_light = {
            let shadow_ray = Ray::shadow(intersection, light_direction);
            let shadow_intersect = world.intersect(&shadow_ray);

            match shadow_intersect {
                Some(shadow_intersection) => shadow_intersection.hit.mag_sq() > (self.position - intersection.hit).mag_sq(),
                None => true,
            }
        };

        let intensity = if in_light { intensity } else { 0.0 };
        let light_power = intersection.contact.normal.dot(light_direction) * intensity;
        let light_reflected = intersection.contact.material.albedo / std::f32::consts::PI;
        let color = intersection.color() * self.color * light_power * light_reflected;
        color.clamp()
    }
}