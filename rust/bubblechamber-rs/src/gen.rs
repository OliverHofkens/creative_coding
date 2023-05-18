use crate::models::{Particle, ParticlesConfig};
use arraydeque::ArrayDeque;
use ndarray::Array1;
use rand::distributions::Slice;
use rand::prelude::*;
use rand::thread_rng;
use rand_distr::{Distribution, Exp, Normal, Uniform};

pub struct Generator {
    lifetime_dist: Exp<f32>,
    charges_dist: Uniform<i8>,
    mass_dist: Normal<f32>,
    velocity_dist: Uniform<f32>,
    avg_per_s: f32,
}

impl Generator {
    pub fn from_config(cfg: &ParticlesConfig) -> Self {
        Self {
            lifetime_dist: Exp::new(cfg.lifetime_exp_lambda).unwrap(),
            charges_dist: Uniform::from(-cfg.max_charge..=cfg.max_charge),
            mass_dist: Normal::new(cfg.mass_mean, cfg.mass_stddev).unwrap(),
            velocity_dist: Uniform::new(-2. * cfg.velocity_mean, 2. * cfg.velocity_mean),
            avg_per_s: cfg.avg_per_s,
        }
    }

    pub fn generate_particles(&self, n_particles: usize) -> Vec<Particle> {
        let mut rng = thread_rng();
        let mut results: Vec<Particle> = Vec::with_capacity(n_particles);

        for _ in 0..n_particles {
            let mass = self.mass_dist.sample(&mut rng) as usize;

            results.push(Particle {
                position: Array1::from_vec(vec![0.0, 0.0, 0.0]),
                velocity: Array1::from_iter(self.velocity_dist.sample_iter(&mut rng).take(3)),
                atomic_charges: Array1::from_iter(
                    self.charges_dist.sample_iter(&mut rng).take(mass),
                ),
                path: ArrayDeque::new(),
                decays_after: self.lifetime_dist.sample(&mut rng),
                lifetime_s: 0.0,
                is_alive: true,
            })
        }
        results
    }

    pub fn split_particle(&self, parent: &Particle) -> Vec<Particle> {
        let mut rng = thread_rng();
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
                decays_after: self.lifetime_dist.sample(&mut rng),
                lifetime_s: 0.0,
                is_alive: true,
            })
        }
        results
    }
}

pub fn maybe_add_particles(gen: &Generator, timestep: f32, particles: &mut Vec<Particle>) {
    let mut rng = thread_rng();
    let rand: f32 = rng.gen();

    let exp_new_particles = timestep * gen.avg_per_s;

    if rand < exp_new_particles {
        let mut new_parts = gen.generate_particles(1);
        particles.append(&mut new_parts);
    }
}
