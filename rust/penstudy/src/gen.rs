use crate::models::Attractor;
use nannou::prelude::*;
use rand::thread_rng;
use rand_distr::{Distribution, Normal, Uniform};

pub fn generate_attractors(num: usize, avg_mass: f32, width: f32, height: f32) -> Vec<Attractor> {
    let mut rng = thread_rng();
    let mass_distr = Normal::new(avg_mass, avg_mass / 10.0).unwrap();
    let x_distr = Uniform::new(-1.0 * width / 2.0, width / 2.0);
    let y_distr = Uniform::new(-1.0 * height / 2.0, height / 2.0);

    let mut results: Vec<Attractor> = Vec::with_capacity(num);

    for _ in 0..num {
        results.push(Attractor {
            pos: Point2::new(x_distr.sample(&mut rng), y_distr.sample(&mut rng)),
            mass: mass_distr.sample(&mut rng),
        })
    }
    results
}
