mod chaos;
mod color;
mod figures;
mod symmetry;

use std::sync::Arc;
use std::time::Instant;
use std::{fs, thread};

use clap::Parser;
use color::palette::{Buckets, Grayscale, NaiveGradient};
use color::scale::{LinearColorScale, LogColorScale};
use num::complex::Complex64;
use pixels::{Pixels, SurfaceTexture};
use std::path::PathBuf;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use chaos::{ChaosEngine, Renderer};
use figures::Figure;

const WIDTH: usize = 3456 / 2;
const HEIGHT: usize = 2234 / 2;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    figure: PathBuf,
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let figure: Box<dyn Figure + Send> = {
        let content = fs::read_to_string(args.figure).unwrap();
        toml::from_str(&content).unwrap()
    };

    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut engine = ChaosEngine::new(
        WIDTH,
        HEIGHT,
        750.0 / 2.0,
        Complex64::new(0.001, 0.001),
        figure,
    );

    let renderer = Renderer::new(
        WIDTH,
        Box::new(LogColorScale::default()),
        // Box::new(Grayscale::default()),
        Box::new(NaiveGradient::new(
            vec![[131, 58, 180, u8::MAX], [252, 176, 69, u8::MAX]],
            vec![],
        )),
        // Box::new(Buckets::new(vec![
        //     [252, 177, 3, u8::MAX],
        //     [252, 3, 177, u8::MAX],
        //     [3, 40, 252, u8::MAX],
        // ])),
        engine.freq.clone(),
    );

    // Simulate in background thread
    thread::spawn(move || {
        engine.step_transient();
        loop {
            const STEPS: usize = 10_000;
            let start = Instant::now();
            engine.batch_step(STEPS);
            let duration = Instant::now() - start;
            println!(
                "Simulating {} steps per second.",
                (STEPS as f64 / duration.as_secs_f64()) as usize
            );
        }
    });

    let mut app = App {
        window: None,
        pixels: None,
        render: renderer,
    };
    event_loop.run_app(&mut app).unwrap();
}

struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    render: Renderer,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = PhysicalSize::new(WIDTH as f64, HEIGHT as f64);
        let win = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Symmetry in Chaos")
                    .with_inner_size(size)
                    .with_min_inner_size(size),
            )
            .unwrap();
        let window = Arc::new(win);

        self.window = Some(window.clone());
        self.pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, window.clone());
            match Pixels::new(
                WIDTH.try_into().unwrap(),
                HEIGHT.try_into().unwrap(),
                surface_texture,
            ) {
                Ok(pixels) => {
                    window.request_redraw();
                    Some(pixels)
                }
                Err(_err) => {
                    log::error!("pixels::new");
                    event_loop.exit();
                    None
                }
            }
        };
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                log::info!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let start = Instant::now();
                self.render.draw(self.pixels.as_mut().unwrap().frame_mut());
                if let Err(_err) = self.pixels.as_ref().unwrap().render() {
                    log::error!("pixels.render");
                    event_loop.exit();
                } else {
                    // Queue a redraw for the next frame
                    self.window.as_ref().unwrap().request_redraw();
                    // thread::sleep_ms(10);
                }
                let draw_duration = Instant::now() - start;

                println!("Draw duration: {:?}", draw_duration);
            }
            WindowEvent::Resized(size) => {
                if let Err(_err) = self
                    .pixels
                    .as_mut()
                    .unwrap()
                    .resize_surface(size.width, size.height)
                {
                    log::error!("pixels.resize_surface");
                    event_loop.exit();
                }
            }
            _ => (),
        }
    }
}
