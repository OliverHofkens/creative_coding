use std::{u64, u8};

pub trait ColorScale {
    fn init_from_freq(&mut self, freqs: &Vec<Vec<u64>>);
    fn freq_to_scale(&self, freq: u64) -> f64;
}

#[derive(Default)]
pub struct LinearColorScale {
    min_freq: u64,
    max_freq: u64,
}

impl ColorScale for LinearColorScale {
    fn init_from_freq(&mut self, freqs: &Vec<Vec<u64>>) {
        let mut min: u64 = u64::MAX;
        let mut max: u64 = 0;

        for row in freqs {
            let row_min = row.iter().filter(|v| **v > 0).min().unwrap_or(&0);
            let row_max = row.iter().max().unwrap();

            min = min.min(*row_min);
            max = max.max(*row_max);
        }

        self.min_freq = min;
        self.max_freq = max;
    }

    fn freq_to_scale(&self, freq: u64) -> f64 {
        if freq == 0 {
            return 0.0;
        }

        let val = freq.saturating_sub(self.min_freq);
        let max = self.max_freq - self.min_freq;
        let res = 0.1 + (val as f64 / max as f64);
        res.clamp(0.0, 1.0)
    }
}

pub trait Palette {
    fn color_from_scale(&self, scale: f64) -> [u8; 4];
}

#[derive(Default)]
pub struct Grayscale {}

impl Palette for Grayscale {
    fn color_from_scale(&self, scale: f64) -> [u8; 4] {
        let val = ((1.0 - scale) * u8::MAX as f64) as u8;
        [val, val, val, u8::MAX]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_scale_from_freq_simple() {
        let mut scale = LinearColorScale::default();
        let freqs = vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 1, 0]];
        scale.init_from_freq(&freqs);

        assert_eq!(scale.min_freq, 1);
        assert_eq!(scale.max_freq, 2);
    }

    #[test]
    fn linear_scale_coloring() {
        let scale = LinearColorScale {
            min_freq: 10,
            max_freq: 20,
        };

        assert_eq!(scale.freq_to_scale(0), 0.0);
        assert_eq!(scale.freq_to_scale(10), 0.1);
        assert_eq!(scale.freq_to_scale(15), 0.6);
        assert_eq!(scale.freq_to_scale(20), 1.0);
    }
}
