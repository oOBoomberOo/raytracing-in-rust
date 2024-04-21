use std::path::PathBuf;
use std::time::Instant;

use anyhow::Context;
use clap::Parser;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use crate::ray::Screen;
use crate::scene::{Renderer, Scene};

mod camera;
mod color;
mod light;
mod model;
mod ray;
mod scene;
mod serialize;
mod texture;
mod world;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[clap(short, long, default_value = "assets/config.yaml")]
    config: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = std::fs::read_to_string(&args.config).context("read config file")?;
    let (world, camera) = serialize::parse_config(&config)?;

    let mut scene = Scene::new(camera, world);

    let mut input = WinitInputHelper::new();
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Raytracing in Rust")
        .with_inner_size(scene.window_size())
        .build(&event_loop)?;

    let mut renderer = Renderer::new(&scene, &window)?;

    let fps = 60.0;

    let frame_time = std::time::Duration::from_secs_f64(1.0 / fps);
    let mut last_draw = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Event::RedrawRequested(_) = event {
            if let Err(e) = renderer.render(&scene) {
                eprintln!("Error rendering frame: {:?}", e);
                *control_flow = ControlFlow::ExitWithCode(1);
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape)
                || input.close_requested()
                || input.destroyed()
            {
                *control_flow = ControlFlow::ExitWithCode(0);
                return;
            }

            if let Some(size) = input.window_resized() {
                renderer.resize(size);
                scene.camera.resize(size);
            }

            let now = Instant::now();
            let delta = now.duration_since(last_draw);

            if delta > frame_time {
                scene.update(&input);
                window.request_redraw();
                last_draw = now;
            }
        }
    })
}
