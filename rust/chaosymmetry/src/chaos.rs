use num::complex::Complex64;

use crate::color::{ColorScale, Palette};

pub struct ChaosEngine {
    width: usize,
    height: usize,
    scale: f64,
    color_scale: Box<dyn ColorScale>,
    color_palette: Box<dyn Palette>,
    freq: Vec<Vec<u64>>,
    params: StandardIconParams,
    curr: Complex64,
}

impl ChaosEngine {
    pub fn new(
        width: usize,
        height: usize,
        scale: f64,
        color_scale: Box<dyn ColorScale>,
        color_palette: Box<dyn Palette>,
        curr: Complex64,
        params: StandardIconParams,
    ) -> Self {
        ChaosEngine {
            width,
            height,
            scale,
            color_scale,
            color_palette,
            freq: vec![vec![0; width]; height],
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
        self.freq[y][x] += 1;
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        self.color_scale.init_from_freq(&self.freq);

        // 1 pixel is 4 u8 values: R,G,B,A
        // So we iter in chunks of 4.
        for (i, px) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % self.width;
            let y = i / self.width;

            let freq = self.freq[y][x];

            let color_scale = self.color_scale.freq_to_scale(freq);
            let rgba = self.color_palette.color_from_scale(color_scale);

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
    symm_deg: f64,
}

impl StandardIconParams {
    pub fn new(lambda: f64, alpha: f64, beta: f64, gamma: f64, omega: f64, symm_deg: f64) -> Self {
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
        let t3 = self.beta * curr.powf(self.symm_deg).re;
        let t4 = self.omega * Complex64::I;

        let t5 = self.gamma * curr.conj().powf(self.symm_deg - 1.0);

        let res = (t1 + t2 + t3 + t4) * curr + t5;

        if res.is_nan() || res.is_infinite() {
            panic!("{curr} -> ({t1} + {t2} + {t3} + {t4})z + {t5}");
        }
        res
    }
}
