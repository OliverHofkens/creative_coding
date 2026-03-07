use std::sync::{Arc, RwLock};

use num::complex::Complex64;

use crate::color::palette::Palette;
use crate::color::scale::ColorScale;
use crate::figures::Figure;

type FreqMap = Vec<u64>;
type SharedFreqMap = Arc<RwLock<FreqMap>>;

pub struct ChaosEngine {
    width: usize,
    height: usize,
    pub freq: SharedFreqMap,
    params: Box<dyn Figure + Send>,
    curr: Complex64,
}

impl ChaosEngine {
    pub fn new(
        width: usize,
        height: usize,
        curr: Complex64,
        params: Box<dyn Figure + Send>,
    ) -> Self {
        ChaosEngine {
            width,
            height,
            freq: Arc::new(RwLock::new(vec![0; width * height])),
            params,
            curr,
        }
    }

    fn coord_to_screen(&self, coord: Complex64) -> (usize, usize) {
        let re = coord.re * self.params.get_scale() as f64;
        let im = coord.im * self.params.get_scale() as f64;
        let x = re + self.width as f64 / 2.0;
        let y = im + self.height as f64 / 2.0;
        (x as usize, y as usize)
    }

    pub fn step(&mut self) {
        let next = self.params.next(self.curr);
        self.curr = next;
        let (x, y) = self.coord_to_screen(next);
        let mut freqs = self.freq.write().unwrap();
        freqs[y * self.width + x] += 1;
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

#[derive(Default)]
pub struct Position {
    pub horizontal: isize,
    pub vertical: isize,
}

pub struct Renderer {
    pub sim_width: usize,
    pub win_width: usize,
    pub scale: f64,
    color_scale: Box<dyn ColorScale>,
    color_palette: Box<dyn Palette>,
    freq: SharedFreqMap,
    pub position: Position,
    frames_drawn: u64,
    update_colors_every: u64,
}

impl Renderer {
    pub fn new(
        sim_width: usize,
        win_width: usize,
        scale: f64,
        color_scale: Box<dyn ColorScale>,
        color_palette: Box<dyn Palette>,
        freq: SharedFreqMap,
        update_colors_every: u64,
    ) -> Self {
        Renderer {
            sim_width,
            win_width,
            scale,
            color_scale,
            color_palette,
            freq,
            position: Position::default(),
            frames_drawn: 0,
            update_colors_every,
        }
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        let freqs = self.freq.read().unwrap();

        // Recalculating color scale is quite expensive, so don't do it every frame.
        if self.frames_drawn.is_multiple_of(self.update_colors_every) {
            self.color_scale.init_from_freq(&freqs[..]);
        }

        // Render center of simulation in center of window
        let win_height = frame.len() / 4 / self.win_width;
        let sim_height = freqs.len() as i64 / self.sim_width as i64;

        // Window size scaled, in sim units
        let scaled_win_width = self.win_width as f64 / self.scale;
        let scaled_win_height = win_height as f64 / self.scale;

        let offset_x = (self.sim_width as f64 - scaled_win_width) / 2.0;
        let offset_y = (sim_height as f64 - scaled_win_height) / 2.0;

        let freqs_per_px = (1.0 / self.scale).clamp(1.0, f64::MAX) as i64;

        // 1 pixel is 4 u8 values: R,G,B,A
        // So we iter in chunks of 4.
        for (i, px) in frame.chunks_exact_mut(4).enumerate() {
            let win_x = i % self.win_width;
            let win_y = i / self.win_width;

            let sim_start_x =
                ((win_x as f64 / self.scale) + offset_x + self.position.horizontal as f64) as i64;
            let sim_start_y =
                ((win_y as f64 / self.scale) + offset_y + self.position.vertical as f64) as i64;

            // Fast path if zoomed in sufficiently:
            let freq = if freqs_per_px == 1 {
                freqs[(sim_start_y * self.sim_width as i64 + sim_start_x) as usize]
            } else {
                (sim_start_y.max(0)..(sim_start_y + freqs_per_px).clamp(0, sim_height - 1))
                    .map(|row| {
                        let row_offset = row as usize * self.sim_width;
                        let start = row_offset + sim_start_x.max(0) as usize;
                        let end = row_offset
                            + (sim_start_x + freqs_per_px).clamp(0, self.sim_width as i64 - 1)
                                as usize;
                        if start < end {
                            freqs[start..end].iter().sum::<u64>()
                        } else {
                            0
                        }
                    })
                    .sum::<u64>()
            };

            let rgba = if freq == 0 {
                [u8::MAX; 4]
            } else {
                let color_scale = self.color_scale.freq_to_scale(freq);
                self.color_palette.color_from_scale(color_scale)
            };

            px.copy_from_slice(&rgba);
        }

        self.frames_drawn += 1;
    }
}
