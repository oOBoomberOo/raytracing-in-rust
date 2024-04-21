use std::ops::Add;

use serde::Deserialize;
use ultraviolet::Vec3;

use crate::color::Color;
use crate::light::Light;
use crate::model::{Material, SurfaceType, Texture};
use crate::ray::{Intersectable, Intersection, Ray};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Sky {
    pub from: Color,
    pub to: Color,
}

#[derive(Default)]
pub struct World {
    pub lights: Vec<Box<dyn Light>>,
    pub entities: Vec<Box<dyn Entity>>,
    pub reflection_depth: usize,
    pub sky: Sky,
}

impl World {
    pub fn new() -> World {
        World::default()
    }

    fn cast_ray(&self, ray: Ray, depth: usize) -> Color {
        if depth > self.reflection_depth {
            return Color::default();
        }

        if let Some(intersection) = self.intersect(&ray) {
            self.compute_light(intersection, depth)
        } else {
            self.sky_color(&ray)
        }
    }

    pub fn sky_color(&self, ray: &Ray) -> Color {
        let t = Vec3::unit_z().dot(ray.direction).clamp(0.0, 1.0);
        Color::lerp(self.sky.from, self.sky.to, t)
    }

    pub fn compute_light(&self, intersection: Intersection, depth: usize) -> Color {
        match intersection.contact.material.surface {
            SurfaceType::Reflective { reflectivity } => {
                let color = self.diffuse_color(&intersection);
                let reflection_ray = Ray::reflect(&intersection);
                let reflected_color = self.cast_ray(reflection_ray, depth + 1) * reflectivity;
                color * (1.0 - reflectivity) + reflected_color
            }

            SurfaceType::Diffuse => self.diffuse_color(&intersection),
        }
    }

    fn diffuse_color(&self, intersection: &Intersection) -> Color {
        self.lights
            .iter()
            .map(|light| light.shading(&self, &intersection))
            .fold(Color::default(), Color::add)
    }
}

impl Intersectable for World {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.entities
            .iter()
            .flat_map(|entity| entity.intersect(ray))
            .min_by(Intersection::closest)
    }
}

pub trait Entity: Intersectable + Texture {
    fn material(&self) -> &Material;
    fn surface_normal(&self, contact: Vec3) -> Vec3;
}
