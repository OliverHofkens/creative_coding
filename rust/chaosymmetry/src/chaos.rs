use std::sync::{Arc, RwLock};

use num::complex::Complex64;

use crate::color::palette::Palette;
use crate::color::scale::ColorScale;
use crate::figures::Figure;

type FreqMap = Vec<Vec<u64>>;
type SharedFreqMap = Arc<RwLock<FreqMap>>;

pub struct ChaosEngine {
    width: usize,
    height: usize,
    scale: f64,
    pub freq: SharedFreqMap,
    params: Box<dyn Figure + Send>,
    curr: Complex64,
}

impl ChaosEngine {
    pub fn new(
        width: usize,
        height: usize,
        scale: f64,
        curr: Complex64,
        params: Box<dyn Figure + Send>,
    ) -> Self {
        ChaosEngine {
            width,
            height,
            scale,
            freq: Arc::new(RwLock::new(vec![vec![0; width]; height])),
            params,
            curr,
        }
    }

    fn coord_to_screen(&self, coord: Complex64) -> (usize, usize) {
        let re = coord.re * self.scale;
        let im = coord.im * self.scale;
        let x = re + self.width as f64 / 2.0;
        let y = im + self.height as f64 / 2.0;
        (x as usize, y as usize)
    }

    pub fn step(&mut self) {
        let next = self.params.next(self.curr);
        self.curr = next;
        let (x, y) = self.coord_to_screen(next);
        let mut freqs = self.freq.write().unwrap();
        freqs[y][x] += 1;
    }

    pub fn batch_step(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }

    pub fn step_transient(&mut self) {
        for _ in 0..1000 {
            let next = self.params.next(self.curr);
            self.curr = next;
        }
    }
}

pub struct Renderer {
    pub win_width: usize,
    pub scale: f64,
    color_scale: Box<dyn ColorScale>,
    color_palette: Box<dyn Palette>,
    freq: SharedFreqMap,
}

impl Renderer {
    pub fn new(
        win_width: usize,
        scale: f64,
        color_scale: Box<dyn ColorScale>,
        color_palette: Box<dyn Palette>,
        freq: SharedFreqMap,
    ) -> Self {
        Renderer {
            win_width,
            scale,
            color_scale,
            color_palette,
            freq,
        }
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        let freqs = self.freq.read().unwrap();
        self.color_scale.init_from_freq(&freqs);

        // Render center of simulation in center of window
        let win_height = frame.len() / 4 / self.win_width;
        let sim_width = freqs[0].len() as f64;
        let sim_height = freqs.len() as f64;

        // Window size scaled, in sim units
        let scaled_win_width = self.win_width as f64 / self.scale;
        let scaled_win_height = win_height as f64 / self.scale;

        let offset_x = (sim_width - scaled_win_width) / 2.0;
        let offset_y = (sim_height - scaled_win_height) / 2.0;

        let freqs_per_px = (1.0 / self.scale).clamp(1.0, f64::MAX) as i64;

        // 1 pixel is 4 u8 values: R,G,B,A
        // So we iter in chunks of 4.
        for (i, px) in frame.chunks_exact_mut(4).enumerate() {
            let win_x = i % self.win_width;
            let win_y = i / self.win_width;

            let sim_start_x = ((win_x as f64 / self.scale) + offset_x) as i64;
            let sim_start_y = ((win_y as f64 / self.scale) + offset_y) as i64;

            let mut freq = 0;
            for row in sim_start_y..sim_start_y + freqs_per_px {
                for col in sim_start_x..sim_start_x + freqs_per_px {
                    if (row >= 0 && row < sim_height as i64) && (col >= 0 && col < sim_width as i64)
                    {
                        freq += freqs[row as usize][col as usize];
                    }
                }
            }

            let rgba = if freq == 0 {
                [u8::MAX; 4]
            } else {
                let color_scale = self.color_scale.freq_to_scale(freq);
                self.color_palette.color_from_scale(color_scale)
            };

            px.copy_from_slice(&rgba);
        }
    }
}
