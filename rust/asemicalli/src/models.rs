use arraydeque::{ArrayDeque, Wrapping};
use nannou::prelude::*;

pub struct Attractor {
    pub pos: Point2,
    pub mass: f32,
}

pub struct Hand {
    pub pos: Point2,
    pub elasticity: f32,
    pub velocity: Point2,
    pub word_idx: usize,
}

pub struct Pen {
    pub point: Attractor,
    pub path: ArrayDeque<[[f32; 2]; 8], Wrapping>,
    pub velocity: Point2,
}

pub struct Letter {
    pub attractors: Vec<Attractor>,
}

pub struct Word {
    pub bounds: Rect<f32>,
    pub attractors: Vec<Attractor>,
}
