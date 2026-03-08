use rand::RngCore;

use crate::figures::{Figure, NonPolyIcon, StandardIcon};
use crate::symmetry::Symmetry;

const LAMBDA_RANGE: (f64, f64) = (-3.0, 3.0);
const ALPHA_RANGE: (f64, f64) = (-3.0, 3.0);
const BETA_RANGE: (f64, f64) = (-1.0, 1.0);
const GAMMA_RANGE: (f64, f64) = (-1.0, 1.0);
const OMEGA_RANGE: (f64, f64) = (-1.0, 1.0);
const DELTA_RANGE: (f64, f64) = (-0.5, 0.5);
const SINGULARITIES: [u32; 3] = [0, 1, 2];

fn uniform(rng: &mut dyn RngCore, (lo, hi): (f64, f64)) -> f64 {
    lo + rand::Rng::random::<f64>(rng) * (hi - lo)
}

/// Sampled parameters for a `StandardIcon`, decoupled from scale and symmetry.
pub struct StandardParams {
    pub lambda: f64,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
    pub omega: f64,
}

/// Sampled parameters for a `NonPolyIcon`, decoupled from scale and symmetry.
pub struct NonPolyParams {
    pub lambda: f64,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
    pub delta: f64,
    pub singularity: u32,
}

pub fn sample_standard(rng: &mut dyn RngCore) -> StandardParams {
    StandardParams {
        lambda: uniform(rng, LAMBDA_RANGE),
        alpha: uniform(rng, ALPHA_RANGE),
        beta: uniform(rng, BETA_RANGE),
        gamma: uniform(rng, GAMMA_RANGE),
        omega: uniform(rng, OMEGA_RANGE),
    }
}

pub fn sample_nonpoly(rng: &mut dyn RngCore) -> NonPolyParams {
    let idx = rand::Rng::random_range(rng, 0..SINGULARITIES.len());
    NonPolyParams {
        lambda: uniform(rng, LAMBDA_RANGE),
        alpha: uniform(rng, ALPHA_RANGE),
        beta: uniform(rng, BETA_RANGE),
        gamma: uniform(rng, GAMMA_RANGE),
        delta: uniform(rng, DELTA_RANGE),
        singularity: SINGULARITIES[idx],
    }
}

pub fn build_standard(p: &StandardParams, symmetry: Symmetry, scale: usize) -> Box<dyn Figure> {
    Box::new(StandardIcon::new(
        p.lambda, p.alpha, p.beta, p.gamma, p.omega, symmetry, scale,
    ))
}

pub fn build_nonpoly(p: &NonPolyParams, symmetry: Symmetry, scale: usize) -> Box<dyn Figure> {
    Box::new(NonPolyIcon::new(
        p.lambda,
        p.alpha,
        p.beta,
        p.gamma,
        p.delta,
        p.singularity,
        symmetry,
        scale,
    ))
}
