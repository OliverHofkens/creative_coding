use arraydeque::{ArrayDeque, Wrapping};
use ndarray::Array1;

pub struct Chamber {
    pub magnetic_field: Array1<f32>,
    pub friction: f32,
}

pub struct Particle {
    pub position: Array1<f32>,
    pub velocity: Array1<f32>,
    pub atomic_charges: Array1<i8>,
    pub path: ArrayDeque<[[f32; 3]; 256], Wrapping>,
    pub decays_after: f32,
    pub lifetime_s: f32,
    pub is_alive: bool,
}

impl Particle {
    pub fn mass(&self) -> i64 {
        self.atomic_charges.len() as i64
    }

    pub fn charge(&self) -> i64 {
        self.atomic_charges.sum() as i64
    }
}
