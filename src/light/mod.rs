use crate::color::Color;
use crate::ray::Intersection;
use crate::world::World;

pub mod directional;
pub mod point;

pub trait Light {
    fn shading(&self, world: &World, intersection: &Intersection) -> Color;
}
