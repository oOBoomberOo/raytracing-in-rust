use crate::model::{Material, Texture, TextureCoord};
use crate::ray::{Intersectable, Intersection, Ray};
use crate::world::Entity;
use ultraviolet::Vec3;

#[derive(Default, Debug, Clone)]
pub struct Plane {
    pub position: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let normal = self.normal;
        let denom = normal.dot(ray.direction);

        if denom <= 1e-6 {
            return None;
        }

        let relative = self.position - ray.origin;
        let distance = relative.dot(normal) / denom;

        if distance < 0.0 {
            return None;
        }

        let hit_point = ray.origin + ray.direction * distance;
        let source = ray.clone();

        let intersection = Intersection::new(hit_point, source, self);

        Some(intersection)
    }
}

impl Texture for Plane {
    fn texture_coord(&self, contact: Vec3) -> TextureCoord {
        let mut x_axis = self.normal.cross(Vec3::unit_z());

        if x_axis.mag_sq() < 0.01 {
            x_axis = self.normal.cross(Vec3::unit_x());
        }

        let y_axis = self.normal.cross(x_axis);

        let relative = contact - self.position;
        let x = relative.dot(x_axis);
        let y = relative.dot(y_axis);

        TextureCoord::new(x, y)
    }
}

impl Entity for Plane {
    fn material(&self) -> &Material {
        &self.material
    }

    fn surface_normal(&self, _contact: Vec3) -> Vec3 {
        -self.normal
    }
}
