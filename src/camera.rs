use crate::ray::Screen;
use ultraviolet::{Vec2, Vec3};
use winit::dpi::PhysicalSize;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

#[derive(Debug)]
pub struct Camera {
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub fov: f32,
    pub origin: Vec3,
    pub speed: f32,
}

impl Camera {
    const FORWARD: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    const BACKWARD: Vec3 = Vec3::new(0.0, 0.0, 1.0);
    const LEFT: Vec3 = Vec3::new(-1.0, 0.0, 0.0);
    const RIGHT: Vec3 = Vec3::new(1.0, 0.0, 0.0);

    pub fn update(&mut self, input: &WinitInputHelper) {
        if input.key_held(VirtualKeyCode::W) {
            self.origin += Camera::FORWARD * self.speed;
        }

        if input.key_held(VirtualKeyCode::S) {
            self.origin += Camera::BACKWARD * self.speed;
        }

        if input.key_held(VirtualKeyCode::A) {
            self.origin += Camera::LEFT * self.speed;
        }

        if input.key_held(VirtualKeyCode::D) {
            self.origin += Camera::RIGHT * self.speed;
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.viewport_width = size.width as f32;
        self.viewport_height = size.height as f32;
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.viewport_width / self.viewport_height
    }

    pub fn compute_fov(fov: f32) -> f32 {
        (fov.to_radians() * 0.5).tan()
    }
}

impl Screen for Camera {
    fn size(&self) -> Vec2 {
        Vec2::new(self.viewport_width, self.viewport_height)
    }
}
