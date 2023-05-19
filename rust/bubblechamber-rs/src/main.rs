use nannou::color;
use nannou::prelude::*;
use ndarray::Array1;
use std::fs;

use models::{Chamber, Config, Particle};
use sim::cross_product;

mod gen;
mod models;
mod sim;

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    is_running: bool,
    zoom_pct: f32,
    chamber: Chamber,
    particles: Vec<Particle>,
    generator: gen::Generator,
    config: Config,
}

fn model(_app: &App) -> Model {
    let cfg: Config = toml::from_str(&fs::read_to_string("config.toml").unwrap()).unwrap();
    let generator = gen::Generator::from_config(&cfg.particles);
    Model {
        is_running: true,
        zoom_pct: 100.0,
        chamber: Chamber {
            magnetic_field: Array1::from_vec(vec![0., 0., cfg.chamber.magnetic_field_strength]),
            friction: cfg.chamber.friction,
        },
        particles: generator.generate_particles(cfg.particles.at_start),
        generator,
        config: cfg,
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id: _, simple } => match simple {
            Some(KeyPressed(key)) => keypress(app, model, key),
            Some(ReceivedCharacter(c)) => input(app, model, c),
            _ => (),
        },
        _ => (),
    }
}

fn keypress(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => model.is_running = !model.is_running,
        _ => (),
    }
}

fn input(_app: &App, model: &mut Model, key: char) {
    match key {
        '+' => model.zoom_pct += 10.0,
        '-' => model.zoom_pct -= 10.0,
        _ => (),
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    if !model.is_running {
        return;
    }
    let tdelta = update.since_last.as_secs_f32();

    let mut to_split: Vec<usize> = Vec::with_capacity(model.particles.len());
    let mut to_remove: Vec<usize> = Vec::with_capacity(model.particles.len());

    for (i, p) in model.particles.iter_mut().enumerate() {
        // Last rites for dying particles, until their path is completely gone:
        if !p.is_alive {
            p.path.pop_front();
            if p.path.is_empty() {
                to_remove.push(i)
            }
            continue;
        }

        // Check if the particle decays:
        p.lifetime_s += tdelta;
        if p.lifetime_s >= p.decays_after {
            p.is_alive = false;
            if p.mass() > 1 {
                to_split.push(i);
            }
        }

        // Magnetic component of Lorentz force:
        let mag_force =
            p.charge() as f32 * cross_product(&p.velocity, &model.chamber.magnetic_field);
        // a = F / m
        let acceleration = mag_force / (p.mass() as f32);

        p.velocity += &(acceleration * tdelta);

        // Apply friction
        p.velocity *= 1.0 - (model.chamber.friction * tdelta);

        p.position += &(&p.velocity * tdelta);

        p.path
            .push_back([p.position[0], p.position[1], p.position[2]]);
    }

    for idx in to_split.drain(0..) {
        let mut new = model.generator.split_particle(&model.particles[idx]);
        model.particles.append(&mut new);
    }

    // Remove from highest to lowest index, so we don't go out of bounds.
    to_remove.sort_unstable_by(|a, b| b.cmp(a));
    for idx in to_remove.drain(0..) {
        model.particles.swap_remove(idx);
    }

    gen::maybe_add_particles(&model.generator, tdelta, &mut model.particles);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let cfg = &model.config.graphics;
    let scale_factor = model.zoom_pct / 100.0;
    let draw = app.draw().scale(scale_factor);

    if cfg.wipe_background {
        draw.background().color(WHITE);
    }

    for p in model.particles.iter() {
        let _charge = p.charge();
        if !cfg.draw_neutral && _charge == 0 {
            continue;
        }

        let path_len = p.path.len();
        let hue = (_charge as f32 / 20.0) + 0.5;

        draw.path()
            .stroke()
            .caps_round()
            .join_round()
            .weight(p.mass() as f32)
            .points_colored(p.path.iter().enumerate().map(|(i, pos)| {
                let pct_dist_to_head = i as f32 / path_len as f32;
                (
                    pt3(pos[0], pos[1], pos[2]),
                    color::hsva(hue, 0.66, pct_dist_to_head, pct_dist_to_head),
                )
            }));
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
