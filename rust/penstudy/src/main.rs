use arraydeque::ArrayDeque;
use nannou::color;
use nannou::prelude::*;

use models::{Attractor, Pen};

mod gen;
mod models;

const GRAVITATIONAL_CONSTANT: f32 = 6.674e-11;
const PEN_ANGLE_RAD: f32 = std::f32::consts::FRAC_PI_4;
const PEN_WIDTH: f32 = 15.0;
// Larger scale = less attraction.
const SCALE: f32 = 1.0;
// Prevent gravity wells.
const MIN_DIST: f32 = 170.0;
const SPEEDUP: f32 = 1.2;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1920, 1080)
        .run();
}

struct Model {
    attractors: Vec<Attractor>,
    pens: Vec<Pen>,
}

fn model(app: &App) -> Model {
    let win = app.window_rect();

    Model {
        attractors: gen::generate_attractors(10, 1e16, win.w() * 2.0 / 3.0, win.h() * 2.0 / 3.0),
        pens: vec![
            Pen {
                point: Attractor {
                    pos: Point2::new(win.left(), win.top()),
                    mass: 1e10,
                },
                path: ArrayDeque::new(),
                velocity: Point2::new(0.0, 0.0),
            },
            Pen {
                point: Attractor {
                    pos: Point2::new(win.left(), 0.0),
                    mass: 1e10,
                },
                path: ArrayDeque::new(),
                velocity: Point2::new(0.0, 0.0),
            },
            Pen {
                point: Attractor {
                    pos: Point2::new(win.left(), win.bottom()),
                    mass: 1e10,
                },
                path: ArrayDeque::new(),
                velocity: Point2::new(0.0, 0.0),
            },
            Pen {
                point: Attractor {
                    pos: Point2::new(win.right(), win.top()),
                    mass: 1e10,
                },
                path: ArrayDeque::new(),
                velocity: Point2::new(0.0, 0.0),
            },
            Pen {
                point: Attractor {
                    pos: Point2::new(win.right(), 0.0),
                    mass: 1e10,
                },
                path: ArrayDeque::new(),
                velocity: Point2::new(0.0, 0.0),
            },
            Pen {
                point: Attractor {
                    pos: Point2::new(win.right(), win.bottom()),
                    mass: 1e10,
                },
                path: ArrayDeque::new(),
                velocity: Point2::new(0.0, 0.0),
            },
        ],
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let delta = update.since_last.as_secs_f32() * SPEEDUP;

    for pen in model.pens.iter_mut() {
        pen.path.push_back([pen.point.pos[0], pen.point.pos[1]]);
        pen.point.pos += pen.velocity * delta;

        let force: Point2 = model
            .attractors
            .iter()
            .map(|att| gravitational_force(&pen.point, att, SCALE))
            .fold(Point2::ZERO, |a, b| a + b);

        pen.velocity += force / pen.point.mass * delta;
        pen.velocity = pen
            .velocity
            .clamp_length(1.0, app.window_rect().len() / 4.0);
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

    draw.background().color(BLACK);

    //for att in &model.attractors {
    //    draw.ellipse().w(5.0).h(5.0).xy(att.pos).color(GRAY);
    //}

    for pen in &model.pens {
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
