use std::sync::atomic::{AtomicU32, Ordering};

use serde::{Deserialize, Serialize};

fn scan_min_max(freqs: &[AtomicU32]) -> (u32, u32) {
    let mut min = u32::MAX;
    let mut max = 0u32;
    for cell in freqs {
        let v = cell.load(Ordering::Relaxed);
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
    fn init_from_freq(&mut self, freqs: &[AtomicU32]);
    fn freq_to_scale(&self, freq: u64) -> f64;
}

/// Linear interpolation. Doesn't work that well in practice at normal
/// zoom levels since the distribution of pixels is so concentrated.
#[derive(Deserialize, Serialize, Default)]
pub struct LinearColorScale {
    #[serde(default)]
    min_freq: u32,
    #[serde(default)]
    max_freq: u32,
}

#[typetag::serde]
impl ColorScale for LinearColorScale {
    fn init_from_freq(&mut self, freqs: &[AtomicU32]) {
        let (min, max) = scan_min_max(freqs);
        self.min_freq = min;
        self.max_freq = max;
    }

    fn freq_to_scale(&self, freq: u64) -> f64 {
        let val = (freq as u32).saturating_sub(self.min_freq);
        // If freq is completely uniform (or not yet initialised),
        // avoid division by zero.
        let max = (self.max_freq - self.min_freq).max(1);
        let res = val as f64 / max as f64;
        res.clamp(0.0, 1.0)
    }
}

/// Compresses dynamic range with a square-root curve, which is stronger than linear,
/// gentler than log.
#[derive(Deserialize, Serialize, Default)]
pub struct SqrtColorScale {
    #[serde(default)]
    min_freq: u32,
    #[serde(default)]
    max_freq: u32,
}

#[typetag::serde]
impl ColorScale for SqrtColorScale {
    fn init_from_freq(&mut self, freqs: &[AtomicU32]) {
        let (min, max) = scan_min_max(freqs);
        self.min_freq = min;
        self.max_freq = max;
    }

    fn freq_to_scale(&self, freq: u64) -> f64 {
        let val = (freq as u32).saturating_sub(self.min_freq) as f64;
        // Guard against uniform / uninitialised map.
        let max = (self.max_freq - self.min_freq).max(1) as f64;
        let res = (val / max).sqrt();
        res.clamp(0.0, 1.0)
    }
}

/// Compresses even more than square-root curve.
#[derive(Deserialize, Serialize, Default)]
pub struct LogColorScale {
    #[serde(default)]
    min_log: u32,
    #[serde(default)]
    max_log: u32,
}

#[typetag::serde]
impl ColorScale for LogColorScale {
    fn init_from_freq(&mut self, freqs: &[AtomicU32]) {
        let (min, max) = scan_min_max(freqs);
        self.min_log = min.ilog2();
        self.max_log = max.ilog2();
    }

    fn freq_to_scale(&self, freq: u64) -> f64 {
        let val = (freq as u32).ilog2().saturating_sub(self.min_log);
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

    fn make_freqs(vals: &[u32]) -> Vec<AtomicU32> {
        vals.iter().map(|&v| AtomicU32::new(v)).collect()
    }

    #[test]
    fn sqrt_scale_from_freq_simple() {
        let mut scale = SqrtColorScale::default();
        let freqs = make_freqs(&[0, 1, 0, 1, 100, 1, 0, 1, 0]);
        scale.init_from_freq(&freqs);

        assert_eq!(scale.min_freq, 1);
        assert_eq!(scale.max_freq, 100);
    }

    #[test]
    fn sqrt_scale_coloring() {
        let scale = SqrtColorScale {
            min_freq: 0,
            max_freq: 100,
        };

        // Boundary values
        assert_eq!(scale.freq_to_scale(0), 0.0);
        assert_eq!(scale.freq_to_scale(100), 1.0);
        // Midpoint of range maps to sqrt(0.5), not 0.5
        assert!((scale.freq_to_scale(50) - 0.5f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn sqrt_scale_uniform_no_division_by_zero() {
        let scale = SqrtColorScale {
            min_freq: 5,
            max_freq: 5,
        };
        // Should not panic, should clamp to 0.0
        assert_eq!(scale.freq_to_scale(5), 0.0);
    }

    #[test]
    fn linear_scale_uniform_no_division_by_zero() {
        let mut scale = LinearColorScale::default();
        let freqs = make_freqs(&[0, 1, 0, 1, 2, 1, 0, 1, 0]);
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
        let freqs = make_freqs(&[0, 1, 0, 1, 1024, 1, 0, 1, 0]);
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
