use crate::models::Particle;
use arraydeque::ArrayDeque;
use ndarray::Array1;
use rand::distributions::Slice;
use rand::prelude::*;
use rand::thread_rng;
use rand_distr::{Distribution, Exp, Normal, Uniform};

const PARTICLE_LIFETIME_LAMBDA: f32 = 1.0;

pub fn split_particle(parent: &Particle) -> Vec<Particle> {
    let mut rng = thread_rng();
    let lifetime_distr = Exp::new(PARTICLE_LIFETIME_LAMBDA).unwrap();
    let n_splits_distr = Uniform::from(2..=parent.mass());

    // Sample charges from the parent:
    let charge_distr = Slice::new(parent.atomic_charges.as_slice().unwrap()).unwrap();

    let n_splits = n_splits_distr.sample(&mut rng);
    let mut results: Vec<Particle> = Vec::with_capacity(n_splits as usize);

    for _ in 0..n_splits {
        let mass = n_splits_distr.sample(&mut rng) as usize - 1;

        results.push(Particle {
            position: parent.position.clone(),
            velocity: parent.velocity.clone(),
            atomic_charges: Array1::from_vec(
                charge_distr
                    .sample_iter(&mut rng)
                    .copied()
                    .take(mass)
                    .collect(),
            ),
            path: ArrayDeque::new(),
            decays_after: lifetime_distr.sample(&mut rng),
            lifetime_s: 0.0,
            is_alive: true,
        })
    }
    results
}

pub fn generate_particles(n_particles: usize, avg_mass: usize, avg_velocity: f32) -> Vec<Particle> {
    let mut rng = thread_rng();
    let lifetime_distr = Exp::new(PARTICLE_LIFETIME_LAMBDA).unwrap();
    let charges_distr = Uniform::from(-2..=2);
    let mass_distr = Normal::new(avg_mass as f32, avg_mass as f32 / 10.0).unwrap();
    let velocity_distr = Uniform::new(-2. * avg_velocity, 2. * avg_velocity);

    let mut results: Vec<Particle> = Vec::with_capacity(n_particles as usize);

    for _ in 0..n_particles {
        let mass = mass_distr.sample(&mut rng) as usize;

        results.push(Particle {
            position: Array1::from_vec(vec![0.0, 0.0, 0.0]),
            velocity: Array1::from_iter(velocity_distr.sample_iter(&mut rng).take(3)),
            atomic_charges: Array1::from_iter(charges_distr.sample_iter(&mut rng).take(mass)),
            path: ArrayDeque::new(),
            decays_after: lifetime_distr.sample(&mut rng),
            lifetime_s: 0.0,
            is_alive: true,
        })
    }
    results
}

pub fn maybe_add_particles(timestep: f32, particles: &mut Vec<Particle>) {
    let mut rng = thread_rng();
    let rand: f32 = rng.gen();
    if rand < timestep * 3. {
        let mut new_parts = generate_particles(1, 6, 300.0);
        particles.append(&mut new_parts);
    }
}
