use nannou::prelude::*;
use rand::prelude::*;

use models::Attractor;

mod models;

const GRAVITATIONAL_CONSTANT: f32 = 6.674e-11;
const PEN_ANGLE_RAD: f32 = std::f32::consts::FRAC_PI_4;
const PEN_WIDTH: f32 = 10.0;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    attractors: Vec<Attractor>,
    pen: Attractor,
    prev_pos: Point2,
    velocity: Point2,
}

fn model(app: &App) -> Model {
    let win = app.window_rect();

    Model {
        // TODO: randomize and add more
        attractors: vec![
            Attractor {
                pos: Point2::new(100.0, 100.0),
                mass: 1e15,
            },
            Attractor {
                pos: Point2::new(-100.0, 100.0),
                mass: 1e15,
            },
            Attractor {
                pos: Point2::new(10.0, -10.0),
                mass: 1e15,
            },
        ],
        pen: Attractor {
            pos: Point2::new(win.left(), win.top()),
            mass: 1e8,
        },
        prev_pos: Point2::new(win.left(), win.top()),
        velocity: Point2::new(win.w() / 30.0, win.h() / -30.0),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.prev_pos = model.pen.pos;
    model.pen.pos += model.velocity;

    let force: Point2 = model
        .attractors
        .iter()
        .map(|att| gravitational_force(&model.pen, att))
        .fold(Point2::ZERO, |a, b| a + b);
    println!("{}", force / model.pen.mass);
    model.velocity += force / model.pen.mass;
    model.velocity = model
        .velocity
        .clamp_length(1.0, app.window_rect().len() / 16.0);
}

fn gravitational_force(att1: &Attractor, att2: &Attractor) -> Point2 {
    let conn = att2.pos - att1.pos;
    let force = GRAVITATIONAL_CONSTANT * (att1.mass * att2.mass / conn.length_squared());
    conn.normalize() * force
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    let conn = model.pen.pos - model.prev_pos;
    let pen_diff_angle = PEN_ANGLE_RAD - conn.angle();
    let pen_thck = 1.0 + pen_diff_angle.sin() * PEN_WIDTH;

    let line_angle = draw
        .line()
        .start(model.prev_pos)
        .end(model.pen.pos)
        .weight(pen_thck)
        .color(RED);

    draw.to_frame(app, &frame).unwrap();
}
