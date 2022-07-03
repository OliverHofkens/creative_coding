use arraydeque::ArrayDeque;
use nannou::color;
use nannou::prelude::*;

use models::{Attractor, Hand, Pen, Word};
use std::env;
use std::fs;

mod gen;
mod models;

const GRAVITATIONAL_CONSTANT: f32 = 6.674e-11;
const PEN_ANGLE_RAD: f32 = std::f32::consts::FRAC_PI_4;
const PEN_WIDTH: f32 = 5.0;
// Larger scale = less attraction.
const SCALE: f32 = 100.0;
// Prevent gravity wells.
const MIN_DIST: f32 = 250.0;
const SPEEDUP: f32 = 1.5;

const POINTS_PER_LETTER: usize = 1;
const AVG_LETTER_MASS: f32 = 1.80e16;
const PEN_MASS: f32 = 1.2e10;

const HAND_ELASTICITY: f32 = 1.1e10;
const HAND_SPEED: f32 = 25.0;

const RESET_PADDING_LEFT_RIGHT: f32 = -1.0;
const RESET_PADDING_TOP_BOT: f32 = -0.3;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1920, 1080)
        .run();
}

struct Model {
    words: Vec<Word>,
    hand: Hand,
    pens: Vec<Pen>,
}

fn model(app: &App) -> Model {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();

    let win = app.window_rect();
    let words = gen::generate_words(
        &contents,
        POINTS_PER_LETTER,
        AVG_LETTER_MASS,
        win.w(),
        win.h(),
    );
    let hand_start = words[0].bounds.mid_left();
    let pen_start = words[0].bounds.bottom_left();

    Model {
        words: words,
        hand: Hand {
            pos: hand_start,
            elasticity: HAND_ELASTICITY,
            velocity: Point2::new(HAND_SPEED, 0.0),
            word_idx: 0,
        },
        pens: vec![Pen {
            point: Attractor {
                pos: pen_start,
                mass: PEN_MASS,
            },
            path: ArrayDeque::new(),
            velocity: Point2::new(0.0, 0.0),
        }],
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let delta = update.since_last.as_secs_f32() * SPEEDUP;

    model.hand.pos += model.hand.velocity * delta;

    let word = &model.words[model.hand.word_idx];

    for pen in model.pens.iter_mut() {
        pen.path.push_back([pen.point.pos[0], pen.point.pos[1]]);
        pen.point.pos += pen.velocity * delta;

        // If we go too far out of bands, reset the position:
        if !word
            .bounds
            .pad_left(RESET_PADDING_LEFT_RIGHT * word.bounds.h())
            .pad_right(RESET_PADDING_LEFT_RIGHT * word.bounds.h())
            .pad_top(RESET_PADDING_TOP_BOT * word.bounds.h())
            .pad_bottom(RESET_PADDING_TOP_BOT * word.bounds.h())
            .contains(pen.point.pos)
        {
            pen.point.pos = model.hand.pos;
            pen.path.clear();
            pen.velocity = Point2::new(0.0, 0.0);
        }

        let grav_force: Point2 = model.words[model.hand.word_idx]
            .attractors
            .iter()
            .map(|att| gravitational_force(&pen.point, att, SCALE))
            .fold(Point2::ZERO, |a, b| a + b);

        let elastic_force: Point2 = model.hand.elasticity * (model.hand.pos - pen.point.pos);

        let force = grav_force + elastic_force;

        pen.velocity += force / pen.point.mass * delta;
        pen.velocity = pen
            .velocity
            .clamp_length(1.0, app.window_rect().len() / 4.0);
    }

    if model.hand.pos[0] >= word.bounds.right() {
        if model.hand.word_idx >= model.words.len() - 1 {
            model.hand.word_idx = 1;
        } else {
            model.hand.word_idx += 1;
        }
        model.hand.pos = model.words[model.hand.word_idx].bounds.mid_left();

        model.pens.clear();
        model.pens.push(Pen {
            point: Attractor {
                pos: model.words[model.hand.word_idx].bounds.bottom_left(),
                mass: 1e10,
            },
            path: ArrayDeque::new(),
            velocity: Point2::new(0.0, 0.0),
        });
    }
}

fn gravitational_force(att1: &Attractor, att2: &Attractor, scale: f32) -> Point2 {
    let conn = att2.pos - att1.pos;
    let dist = f32::max(conn.length_squared() * scale, MIN_DIST);
    let force = GRAVITATIONAL_CONSTANT * (att1.mass * att2.mass / dist);
    conn.normalize() * force
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    //draw.background().color(BLACK);

    //for word in &model.words {
    //    draw.rect()
    //        .xy(word.bounds.xy())
    //        .wh(word.bounds.wh())
    //        .stroke(GRAY);

    //    //    for att in &word.attractors {
    //    //        draw.ellipse().w(5.0).h(5.0).xy(att.pos).color(GRAY);
    //    //    }
    //}

    //draw.ellipse().w(5.0).h(5.0).xy(model.hand.pos).color(GREEN);

    for pen in &model.pens {
        if pen.path.len() == 0 {
            continue;
        }
        let mut prev_v = Point2::from_slice(&pen.path[0]);
        let path_len = pen.path.len();

        for (i, next) in pen.path.iter().enumerate() {
            let pct_dist_to_head = i as f32 / path_len as f32;
            let next_v = Point2::from_slice(next);
            let conn = next_v - prev_v;
            let pen_diff_angle = PEN_ANGLE_RAD - conn.angle();
            let pen_thck = 1.0 + pen_diff_angle.sin() * PEN_WIDTH;

            draw.line()
                .start(prev_v)
                .end(next_v)
                .weight(pen_thck)
                .start_cap_round()
                .color(color::rgba(1.0, 1.0, 1.0, pct_dist_to_head));
            prev_v = next_v;
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
