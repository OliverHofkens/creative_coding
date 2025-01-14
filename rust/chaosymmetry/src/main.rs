mod chaos;
mod color;

use std::sync::Arc;

use color::{Grayscale, LinearColorScale};
use num::complex::Complex64;
use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use chaos::{ChaosEngine, StandardIconParams};

const WIDTH: usize = 3456;
const HEIGHT: usize = 2234;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let config = ChaosEngine::new(
        WIDTH,
        HEIGHT,
        750.0,
        Box::new(LinearColorScale::default()),
        Box::new(Grayscale::default()),
        Complex64::new(0.001, 0.001),
        // Fish and Eye
        // StandardIconParams::new(-2.18, 10.0, -12.0, 1.0, 0.0, 2.0),
        // The Trampoline
        StandardIconParams::new(1.56, -1.0, 0.1, -0.82, 0.0, 3.0),
    );

    let mut app = App {
        window: None,
        pixels: None,
        chaos: config,
        iter_per_draw: 1000,
    };
    event_loop.run_app(&mut app).unwrap();
}

struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    chaos: ChaosEngine,
    iter_per_draw: usize,
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
                for _ in 0..self.iter_per_draw {
                    self.chaos.step();
                }
                self.chaos.draw(self.pixels.as_mut().unwrap().frame_mut());
                if let Err(_err) = self.pixels.as_ref().unwrap().render() {
                    log::error!("pixels.render");
                    event_loop.exit();
                } else {
                    // Queue a redraw for the next frame
                    self.window.as_ref().unwrap().request_redraw();
                    // thread::sleep_ms(10);
                }
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
