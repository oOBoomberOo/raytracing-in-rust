use std::cmp::Ordering;
use std::ops::Range;

use ultraviolet::{Vec2, Vec3};

use crate::camera::Camera;
use crate::color::Color;
use crate::model::{Material, TextureCoord};
use crate::world::Entity;

#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    const BIAS: f32 = 1e-3;

    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn from_screen(camera: &Camera, pos: Vec2) -> Ray {
        let x = pos.x * camera.aspect_ratio() * camera.fov;
        let y = pos.y * camera.fov;

        let origin = camera.origin;
        let direction = Vec3::new(x, y, -1.0).normalized();

        Ray::new(origin, direction)
    }

    pub fn shadow(intersection: &Intersection, direction: Vec3) -> Ray {
        let origin = intersection.hit + direction * Ray::BIAS;
        Ray::new(origin, direction)
    }

    pub fn reflect(intersection: &Intersection) -> Ray {
        let normal = intersection.contact.normal;
        let incident = intersection.source.direction;
        let origin = intersection.hit + normal * Ray::BIAS;
        let direction = incident - (2.0 * incident.dot(normal) * normal);
        Ray::new(origin, direction)
    }
}

pub trait Screen {
    fn size(&self) -> Vec2;

    fn at(&self, x: f32, y: f32) -> Vec2 {
        let size = self.size();
        let x = clamp(x, 0.0..size.x, -1.0..1.0);
        let y = clamp(y, 0.0..size.y, -1.0..1.0);
        Vec2::new(x, y)
    }
}

fn clamp(value: f32, from: Range<f32>, to: Range<f32>) -> f32 {
    (value - from.start) / (from.end - from.start) * (to.end - to.start) + to.start
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

pub struct Intersection<'a> {
    pub hit: Vec3,
    pub source: Ray,
    pub contact: Contact<'a>,
}

impl<'a> Intersection<'a> {
    pub fn new(hit: Vec3, source: Ray, entity: &impl Entity) -> Intersection<'_> {
        let contact = Contact::new(hit, entity);
        Intersection {
            hit,
            source,
            contact,
        }
    }

    pub fn magnitude_sq(&self) -> f32 {
        self.hit.mag_sq()
    }

    pub fn color(&self) -> Color {
        self.contact.material.color(self.contact.texture)
    }

    pub fn closest(a: &Intersection, b: &Intersection) -> Ordering {
        a.magnitude_sq()
            .partial_cmp(&b.magnitude_sq())
            .unwrap_or(Ordering::Equal)
    }
}

pub struct Contact<'a> {
    pub normal: Vec3,
    pub texture: TextureCoord,
    pub material: &'a Material,
}

impl<'a> Contact<'a> {
    pub fn new(point: Vec3, entity: &impl Entity) -> Contact<'_> {
        Contact {
            normal: entity.surface_normal(point),
            texture: entity.texture_coord(point),
            material: entity.material(),
        }
    }
}
