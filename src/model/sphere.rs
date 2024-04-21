use ultraviolet::Vec3;

use crate::model::{Material, Texture, TextureCoord};
use crate::ray::{Intersectable, Intersection, Ray};
use crate::world::Entity;

#[derive(Default, Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let local = self.center - ray.origin;
        let projected = local.dot(ray.direction);
        let distance = local.mag_sq() - projected * projected;
        let radius_sq = self.radius * self.radius;

        if distance > radius_sq {
            return None;
        }

        let thickness = (radius_sq - distance).sqrt();
        let edge_front = projected - thickness;
        let edge_back = projected + thickness;

        if edge_front < 0.0 && edge_back < 0.0 {
            return None;
        }

        let distance_to_edge = if edge_front < 0.0 {
            edge_back
        } else if edge_back < 0.0 {
            edge_front
        } else {
            edge_front.min(edge_back)
        };

        let hit_point = ray.origin + ray.direction * distance_to_edge;
        let source = ray.clone();

        let intersection = Intersection::new(hit_point, source, self);

        Some(intersection)
    }
}

impl Texture for Sphere {
    fn texture_coord(&self, contact: Vec3) -> TextureCoord {
        use std::f32::consts::PI;

        let normal = (contact - self.center).normalized();

        let u = (1.0 + normal.z.atan2(normal.x)) / PI * 0.5;
        let v = (contact.y / self.radius) / PI;

        TextureCoord::new(u, v)
    }
}

impl Entity for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }

    fn surface_normal(&self, contact: Vec3) -> Vec3 {
        (contact - self.center).normalized()
    }
}
