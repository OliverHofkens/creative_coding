mod chaos;
mod color;
mod figures;
mod symmetry;

use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use std::{default, fs, thread};

use chrono::Local;
use clap::Parser;
use color::ColorConfig;
use num::complex::Complex64;
use pixels::{Pixels, SurfaceTexture};
use rand::Rng;
use std::path::{Path, PathBuf};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::Key;
use winit::window::{Window, WindowId};

use chaos::{ChaosEngine, Renderer};
use figures::Figure;

const WIDTH: usize = 3456 / 2;
const HEIGHT: usize = 2234 / 2;
const SIM_WIDTH: usize = 10_000;
const SIM_HEIGHT: usize = 10_000;
const ZOOM_FACTOR: f64 = 1.25;
const PAN_STEP: isize = 100;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    figure: PathBuf,
    #[arg(short, long, value_name = "FILE")]
    style: PathBuf,
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let figure: Box<dyn Figure + Send> = {
        let content = fs::read_to_string(args.figure).unwrap();
        toml::from_str(&content).unwrap()
    };
    let style: ColorConfig = {
        let content = fs::read_to_string(args.style).unwrap();
        toml::from_str(&content).unwrap()
    };

    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut rng = rand::rng();
    let re = rng.random_range(0.001..0.005);
    let im = rng.random_range(0.001..0.005);
    let mut engine = ChaosEngine::new(SIM_WIDTH, SIM_HEIGHT, Complex64::new(re, im), figure);

    let renderer = Renderer::new(WIDTH, 0.5, style.scale, style.palette, engine.freq.clone());

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
        mult: Mult::default(),
    };
    event_loop.run_app(&mut app).unwrap();
}

struct Mult {
    value: String,
    valid_since: SystemTime,
}

impl Default for Mult {
    fn default() -> Self {
        Mult {
            value: "1".to_string(),
            valid_since: SystemTime::UNIX_EPOCH,
        }
    }
}

struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    render: Renderer,
    mult: Mult,
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
                if let Err(_err) = self
                    .pixels
                    .as_mut()
                    .unwrap()
                    .resize_buffer(size.width, size.height)
                {
                    log::error!("pixels.resize_buffer");
                    event_loop.exit();
                }
                self.render.win_width = size.width as usize;
            }
            WindowEvent::KeyboardInput {
                event,
                is_synthetic: false,
                ..
            } => {
                if event.state.is_pressed()
                    // Since "+" requires pressing "Shift", this would otherwise reset the multiplier to "1" before zooming in
                    && event.logical_key != Key::Named(winit::keyboard::NamedKey::Shift)
                {
                    if self.mult.valid_since.elapsed().unwrap() > Duration::from_secs(5) {
                        self.mult = Default::default()
                    }
                    if Key::Character("0".into()) <= event.logical_key
                        && event.logical_key <= Key::Character("9".into())
                    {
                        if self.mult.valid_since.elapsed().unwrap() <= Duration::from_secs(5) {
                            self.mult.value.push_str(&event.text.unwrap().to_string());
                        } else {
                            self.mult.value = event.text.unwrap().to_string();
                        }
                        self.mult.valid_since = SystemTime::now();
                    } else {
                        if event.logical_key == Key::Character("s".into()) {
                            let data = self.pixels.as_ref().unwrap().frame();

                            let filename = format!("{}.png", Local::now().to_rfc3339());
                            let path = Path::new(&filename);
                            save_png(data, self.render.win_width, path);
                        } else if event.logical_key == Key::Character("+".into()) {
                            self.render.scale *= ZOOM_FACTOR.powf(self.mult.value.parse().unwrap());
                        } else if event.logical_key == Key::Character("-".into()) {
                            self.render.scale /= ZOOM_FACTOR.powf(self.mult.value.parse().unwrap());
                        } else if event.logical_key
                            == Key::Named(winit::keyboard::NamedKey::ArrowUp)
                        {
                            self.render.position.vertical -=
                                self.mult.value.parse::<isize>().unwrap() * PAN_STEP;
                        } else if event.logical_key
                            == Key::Named(winit::keyboard::NamedKey::ArrowDown)
                        {
                            self.render.position.vertical +=
                                self.mult.value.parse::<isize>().unwrap() * PAN_STEP;
                        } else if event.logical_key
                            == Key::Named(winit::keyboard::NamedKey::ArrowRight)
                        {
                            self.render.position.horizontal +=
                                self.mult.value.parse::<isize>().unwrap() * PAN_STEP;
                        } else if event.logical_key
                            == Key::Named(winit::keyboard::NamedKey::ArrowLeft)
                        {
                            self.render.position.horizontal -=
                                self.mult.value.parse::<isize>().unwrap() * PAN_STEP;
                        }
                        self.mult = Default::default();
                    }
                }
            }
            _ => (),
        }
    }
}

fn save_png(img_data: &[u8], img_width: usize, output_path: &Path) {
    println!("Saving to {:?}", output_path);
    let file = File::create(output_path).unwrap();
    let w = BufWriter::new(file);

    let img_height = img_data.len() / 4 / img_width;
    let mut encoder = png::Encoder::new(w, img_width as u32, img_height as u32);
    encoder.set_color(png::ColorType::Rgba);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(img_data).unwrap();
}
