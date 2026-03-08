use serde::{Deserialize, Serialize};

fn scan_min_max(freqs: &[u64]) -> (u64, u64) {
    let mut min = u64::MAX;
    let mut max = 0u64;
    for &v in freqs {
        if v > 0 {
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
        }
    }
    if max == 0 {
        (0, 0)
    } else {
        (min, max)
    }
}

#[typetag::serde(tag = "type")]
pub trait ColorScale: Sync {
    fn init_from_freq(&mut self, freqs: &[u64]);
    fn freq_to_scale(&self, freq: u64) -> f64;
}

#[derive(Deserialize, Serialize, Default)]
pub struct LinearColorScale {
    #[serde(default)]
    min_freq: u64,
    #[serde(default)]
    max_freq: u64,
}

#[typetag::serde]
impl ColorScale for LinearColorScale {
    fn init_from_freq(&mut self, freqs: &[u64]) {
        let (min, max) = scan_min_max(freqs);
        self.min_freq = min;
        self.max_freq = max;
    }

    fn freq_to_scale(&self, freq: u64) -> f64 {
        let val = freq.saturating_sub(self.min_freq);
        let max = self.max_freq - self.min_freq;
        let res = val as f64 / max as f64;
        res.clamp(0.0, 1.0)
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct LogColorScale {
    #[serde(default)]
    min_log: u64,
    #[serde(default)]
    max_log: u64,
}

#[typetag::serde]
impl ColorScale for LogColorScale {
    fn init_from_freq(&mut self, freqs: &[u64]) {
        let (min, max) = scan_min_max(freqs);
        self.min_log = min.ilog2() as u64;
        self.max_log = max.ilog2() as u64;
    }

    fn freq_to_scale(&self, freq: u64) -> f64 {
        let val = freq.ilog2().saturating_sub(self.min_log as u32) as u64;
        // If freq is completely uniform,
        // avoid division by zero.
        let max = (self.max_log - self.min_log).max(1);
        let res = val as f64 / max as f64;
        res.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_scale_from_freq_simple() {
        let mut scale = LinearColorScale::default();
        let freqs = vec![0, 1, 0, 1, 2, 1, 0, 1, 0];
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

        assert_eq!(scale.freq_to_scale(10), 0.0);
        assert_eq!(scale.freq_to_scale(15), 0.5);
        assert_eq!(scale.freq_to_scale(20), 1.0);
    }

    #[test]
    fn log_scale_from_freq_simple() {
        let mut scale = LogColorScale::default();
        let freqs = vec![0, 1, 0, 1, 1024, 1, 0, 1, 0];
        scale.init_from_freq(&freqs);

        assert_eq!(scale.min_log, 0);
        assert_eq!(scale.max_log, 10);
    }

    #[test]
    fn log_scale_coloring() {
        let scale = LogColorScale {
            min_log: 0,
            max_log: 10,
        };

        assert_eq!(scale.freq_to_scale(1), 0.0);
        assert_eq!(scale.freq_to_scale(8), 0.3);
        assert_eq!(scale.freq_to_scale(256), 0.8);
        assert_eq!(scale.freq_to_scale(512), 0.9);
        assert_eq!(scale.freq_to_scale(1024), 1.0);
    }
}
