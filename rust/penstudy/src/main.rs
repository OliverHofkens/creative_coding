use arraydeque::{ArrayDeque, Wrapping};
use nannou::prelude::*;

use models::Attractor;

mod gen;
mod models;

const GRAVITATIONAL_CONSTANT: f32 = 6.674e-11;
const PEN_ANGLE_RAD: f32 = std::f32::consts::FRAC_PI_4;
const PEN_WIDTH: f32 = 10.0;
const SCALE: f32 = 0.8;
const SPEEDUP: f32 = 2.0;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1920, 1080)
        .run();
}

struct Model {
    attractors: Vec<Attractor>,
    pen: Attractor,
    path: ArrayDeque<[[f32; 2]; 256], Wrapping>,
    velocity: Point2,
}

fn model(app: &App) -> Model {
    let win = app.window_rect();

    Model {
        attractors: gen::generate_attractors(20, 1e16, win.w() * 2.0 / 3.0, win.h() * 2.0 / 3.0),
        pen: Attractor {
            pos: Point2::new(win.left(), win.top()),
            mass: 1e10,
        },
        path: ArrayDeque::new(),
        velocity: Point2::new(win.w() / 30.0, win.h() / -30.0),
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let delta = update.since_last.as_secs_f32() * SPEEDUP;

    model.path.push_back([model.pen.pos[0], model.pen.pos[1]]);
    model.pen.pos += model.velocity * delta;

    let force: Point2 = model
        .attractors
        .iter()
        .map(|att| gravitational_force(&model.pen, att, SCALE))
        .fold(Point2::ZERO, |a, b| a + b);

    model.velocity += force / model.pen.mass * delta;
    model.velocity = model
        .velocity
        .clamp_length(1.0, app.window_rect().len() / 4.0);
}

fn gravitational_force(att1: &Attractor, att2: &Attractor, scale: f32) -> Point2 {
    let conn = att2.pos - att1.pos;
    let force = GRAVITATIONAL_CONSTANT * (att1.mass * att2.mass / (conn.length_squared() * scale));
    conn.normalize() * force
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    //for att in &model.attractors {
    //    draw.ellipse().w(10.0).h(10.0).xy(att.pos);
    //}

    let mut prev_v = Point2::from_slice(&model.path[0]);
    for next in model.path.iter() {
        let next_v = Point2::from_slice(next);
        let conn = next_v - prev_v;
        let pen_diff_angle = PEN_ANGLE_RAD - conn.angle();
        let pen_thck = 1.0 + pen_diff_angle.sin() * PEN_WIDTH;

        let line_angle = draw
            .line()
            .start(prev_v)
            .end(next_v)
            .weight(pen_thck)
            .color(WHITE);
        prev_v = next_v;
    }

    draw.to_frame(app, &frame).unwrap();
}
