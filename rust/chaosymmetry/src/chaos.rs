use std::sync::{Arc, RwLock};

use num::complex::Complex64;

use crate::color::palette::Palette;
use crate::color::scale::ColorScale;

type FreqMap = Vec<Vec<u64>>;
type SharedFreqMap = Arc<RwLock<FreqMap>>;

pub struct ChaosEngine {
    width: usize,
    height: usize,
    scale: f64,
    pub freq: SharedFreqMap,
    params: StandardIconParams,
    curr: Complex64,
}

impl ChaosEngine {
    pub fn new(
        width: usize,
        height: usize,
        scale: f64,
        curr: Complex64,
        params: StandardIconParams,
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
    width: usize,
    color_scale: Box<dyn ColorScale>,
    color_palette: Box<dyn Palette>,
    freq: SharedFreqMap,
}

impl Renderer {
    pub fn new(
        width: usize,
        color_scale: Box<dyn ColorScale>,
        color_palette: Box<dyn Palette>,
        freq: SharedFreqMap,
    ) -> Self {
        Renderer {
            width,
            color_scale,
            color_palette,
            freq,
        }
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        let freqs = self.freq.read().unwrap();
        self.color_scale.init_from_freq(&freqs);

        // 1 pixel is 4 u8 values: R,G,B,A
        // So we iter in chunks of 4.
        for (i, px) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % self.width;
            let y = i / self.width;

            let freq = freqs[y][x];

            let rgba = if freq == 0 {
                [u8::MAX; 4]
            } else {
                let color_scale = self.color_scale.freq_to_scale(freq);
                // println!("Color scale: {color_scale}");
                self.color_palette.color_from_scale(color_scale)
            };

            px.copy_from_slice(&rgba);
        }
    }
}

pub struct StandardIconParams {
    lambda: f64,
    alpha: f64,
    beta: f64,
    gamma: f64,
    omega: f64,
    symm_deg: u32,
}

impl StandardIconParams {
    pub fn new(lambda: f64, alpha: f64, beta: f64, gamma: f64, omega: f64, symm_deg: u32) -> Self {
        StandardIconParams {
            lambda,
            alpha,
            beta,
            gamma,
            omega,
            symm_deg,
        }
    }

    fn next(&self, curr: Complex64) -> Complex64 {
        let t1 = self.lambda;
        let t2 = self.alpha * curr * curr.conj();
        let t3 = self.beta * curr.powu(self.symm_deg).re;
        let t4 = self.omega * Complex64::I;

        let t5 = self.gamma * curr.conj().powu(self.symm_deg - 1);

        let res = (t1 + t2 + t3 + t4) * curr + t5;

        // if res.is_nan() || res.is_infinite() {
        //     panic!("{curr} -> ({t1} + {t2} + {t3} + {t4})z + {t5}");
        // }
        res
    }
}
