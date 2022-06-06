use arraydeque::{ArrayDeque, Wrapping};
use nannou::prelude::*;

pub struct Attractor {
    pub pos: Point2,
    pub mass: f32,
}

pub struct Pen {
    pub point: Attractor,
    pub path: ArrayDeque<[[f32; 2]; 64], Wrapping>,
    pub velocity: Point2,
}
