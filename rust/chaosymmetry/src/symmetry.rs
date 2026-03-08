use std::f64::consts::PI;

use num::complex::Complex64;
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum SymmetryRepr {
    Cyclic(u32),
    Dihedral(u32),
}

#[derive(Clone, Copy)]
enum SymmetryKind {
    Cyclic,
    Dihedral,
}

// Serialize round-trips through SymmetryRepr so TOML output stays compatible.
#[derive(Serialize, Deserialize, Clone)]
#[serde(from = "SymmetryRepr", into = "SymmetryRepr")]
pub struct Symmetry {
    degree: u32,
    kind: SymmetryKind,
    // Precomputed (cos(k), sin(k)), since they took a lot of compute at runtime.
    // Built once from the degree at deserialisation time; never serialised.
    rotations: Vec<(f64, f64)>,
}

impl From<SymmetryRepr> for Symmetry {
    fn from(repr: SymmetryRepr) -> Self {
        let (degree, kind) = match repr {
            SymmetryRepr::Cyclic(n) => (n, SymmetryKind::Cyclic),
            SymmetryRepr::Dihedral(n) => (n, SymmetryKind::Dihedral),
        };
        let theta = 2.0 * PI / degree as f64;
        let rotations = (0..degree)
            .map(|k| {
                let angle = k as f64 * theta;
                (angle.cos(), angle.sin())
            })
            .collect();
        Symmetry {
            degree,
            kind,
            rotations,
        }
    }
}

impl From<Symmetry> for SymmetryRepr {
    fn from(s: Symmetry) -> Self {
        match s.kind {
            SymmetryKind::Cyclic => SymmetryRepr::Cyclic(s.degree),
            SymmetryKind::Dihedral => SymmetryRepr::Dihedral(s.degree),
        }
    }
}

impl Symmetry {
    pub fn get_degree(&self) -> u32 {
        self.degree
    }

    pub fn apply_random(&self, point: Complex64, rng: &mut dyn RngCore) -> Complex64 {
        let k = rng.random_range(0..self.degree) as usize;
        // Table lookup, no cos/sin at runtime since they are expensive.
        let (cos_k, sin_k) = self.rotations[k];
        let rotated = Complex64::new(
            point.re * cos_k - point.im * sin_k,
            point.re * sin_k + point.im * cos_k,
        );
        match self.kind {
            SymmetryKind::Cyclic => rotated,
            SymmetryKind::Dihedral => {
                // Randomly reflect across the x-axis.
                if rng.random_bool(0.5) {
                    Complex64::new(rotated.re, -rotated.im)
                } else {
                    rotated
                }
            }
        }
    }
}
