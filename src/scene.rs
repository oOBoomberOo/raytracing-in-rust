use anyhow::Context;
use pixels::{Error, Pixels, SurfaceTexture};
use ultraviolet::Vec2;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

use crate::camera::Camera;
use crate::ray::{Intersectable, Ray, Screen};
use crate::world::World;

pub struct Scene {
    pub camera: Camera,
    pub world: World,
}

impl Scene {
    pub fn width(&self) -> u32 {
        self.camera.viewport_width as u32
    }

    pub fn height(&self) -> u32 {
        self.camera.viewport_height as u32
    }

    pub fn window_size(&self) -> PhysicalSize<u32> {
        PhysicalSize::new(self.width(), self.height())
    }

    pub fn update(&mut self, input: &WinitInputHelper) {
        self.camera.update(input);
    }
}

impl Scene {
    pub fn new(camera: Camera, world: World) -> Self {
        Self { camera, world }
    }

    fn pos(&self, index: usize) -> Vec2 {
        let width = self.width();
        let x = index as u32 % width;
        let y = index as u32 / width;
        self.at(x as f32, y as f32)
    }
}

impl Screen for Scene {
    fn size(&self) -> Vec2 {
        self.camera.size()
    }
}

pub struct Renderer {
    pixels: Pixels,
}

impl Renderer {
    pub fn new(scene: &Scene, window: &Window) -> anyhow::Result<Self> {
        let surface_texture = SurfaceTexture::new(scene.width(), scene.height(), window);
        let pixels = Pixels::new(scene.width(), scene.height(), surface_texture)
            .context("create pixels renderer")?;
        Ok(Self { pixels })
    }

    pub fn render(&mut self, scene: &Scene) -> Result<(), Error> {
        let frame = self.pixels.frame_mut();
        Self::draw(scene, frame);
        self.pixels.render()
    }

    fn draw(scene: &Scene, frame: &mut [u8]) {
        for (index, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let ray = Ray::from_screen(&scene.camera, scene.pos(index));

            let intersected_color = scene
                .world
                .intersect(&ray)
                .map_or(scene.world.sky_color(&ray), |intersection| {
                    scene.world.compute_light(intersection, 0)
                });

            pixel.copy_from_slice(&intersected_color.as_slice());
        }
    }

    pub fn resize(&mut self, size: impl Into<PhysicalSize<u32>>) {
        let size = size.into();

        self.pixels
            .resize_surface(size.width, size.height)
            .expect("resize window");
        self.pixels
            .resize_buffer(size.width, size.height)
            .expect("resize buffer");
    }
}
