use num::complex::Complex64;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Symmetry {
    Dihedral(u32),
    Cyclic(u32),
}

impl Symmetry {
    pub fn get_degree(&self) -> u32 {
        match self {
            Symmetry::Cyclic(n) | Symmetry::Dihedral(n) => *n,
        }
    }
    pub fn apply_random(&self, point: Complex64) -> Complex64 {
        let mut rng = rand::thread_rng();
        match self {
            Symmetry::Cyclic(n) => {
                // Randomly choose one of n rotations
                let k = rng.gen_range(0..*n);
                self.rotate(point, k as f64 * 2.0 * std::f64::consts::PI / *n as f64)
            }
            Symmetry::Dihedral(n) => {
                // First randomly choose rotation
                let k = rng.gen_range(0..*n);
                let rotated = self.rotate(point, k as f64 * 2.0 * std::f64::consts::PI / *n as f64);

                // Then randomly decide whether to reflect
                if rng.gen_bool(0.5) {
                    self.reflect(rotated)
                } else {
                    rotated
                }
            }
        }
    }

    fn rotate(&self, point: Complex64, angle: f64) -> Complex64 {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();
        Complex64::new(
            point.re * cos_theta - point.im * sin_theta,
            point.re * sin_theta + point.im * cos_theta,
        )
    }

    fn reflect(&self, point: Complex64) -> Complex64 {
        // Reflect across x-axis
        Complex64::new(point.re, -point.im)
    }
}
