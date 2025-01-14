use std::u64;

struct Palette {}

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
        ((freq - self.min_freq) / (self.max_freq - self.min_freq)) as f64
    }
}
