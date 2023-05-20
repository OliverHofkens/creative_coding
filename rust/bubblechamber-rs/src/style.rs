use crate::models::{GraphicsConfig, Model};
use nannou::color;
use nannou::draw::Draw;
use nannou::prelude::*;
use rand::prelude::*;
use rand::thread_rng;
use rand_distr::Normal;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum Style {
    Original,
    Pastel,
    SpiderVerse,
}

pub trait StyleRenderer {
    fn draw(&self, app: &App, model: &Model, cfg: &GraphicsConfig, draw: &Draw);
}

struct OriginalRenderer;
struct PastelRenderer;
struct SpiderVerseRenderer;

pub fn get_style_renderer(style: &Style) -> Box<dyn StyleRenderer> {
    match style {
        Style::Original => Box::new(OriginalRenderer {}),
        Style::Pastel => Box::new(PastelRenderer {}),
        Style::SpiderVerse => Box::new(SpiderVerseRenderer {}),
    }
}

impl StyleRenderer for OriginalRenderer {
    fn draw(&self, _app: &App, model: &Model, cfg: &GraphicsConfig, draw: &Draw) {
        if cfg.wipe_background {
            draw.background().color(color::BLACK);
        }

        for p in model.particles.iter() {
            if !cfg.draw_neutral && p.charge() == 0 {
                continue;
            }

            draw.path()
                .stroke()
                .caps_round()
                .join_round()
                .weight(1.)
                .points_colored(
                    p.path
                        .iter()
                        .map(|pos| (pt3(pos[0], pos[1], pos[2]), color::WHITE)),
                );
        }
    }
}

impl StyleRenderer for PastelRenderer {
    fn draw(&self, _app: &App, model: &Model, cfg: &GraphicsConfig, draw: &Draw) {
        if cfg.wipe_background {
            draw.background().color(color::WHITE);
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
    }
}

impl StyleRenderer for SpiderVerseRenderer {
    fn draw(&self, app: &App, model: &Model, cfg: &GraphicsConfig, draw: &Draw) {
        if cfg.wipe_background {
            draw.background().color(color::BLACK);
        }

        let window = app.window_rect();
        let mut rng = thread_rng();

        let diam_distr = Normal::new(40.0, 10.0).unwrap();
        let y_jitter = Normal::new(0.0, 20.0).unwrap();

        let hue_min = 258. / 360.;
        let hue_max = 339. / 360.;
        let hue_bias = (hue_max - hue_min) / 2.0;
        let hue_distr = Normal::new((hue_min + hue_max) / 2., hue_bias).unwrap();

        let sat_min = 0.9;
        let sat_max = 1.0;
        let sat_bias = sat_max - sat_min;

        let lum_min = 0.5;
        let lum_max = 1.0;
        let lum_bias = lum_max - lum_min;

        let left = window.left() as i32;
        let right = window.right() as i32;
        for x in left..right {
            let diam = diam_distr.sample(&mut rng);
            let x = x as f32;
            let y = y_jitter.sample(&mut rng);
            let pct_dist_to_center = 1.0 - (x.abs() / (window.w() / 2.0));
            draw.ellipse().x_y(x, y).w_h(diam, diam).color(color::hsla(
                hue_distr.sample(&mut rng) + pct_dist_to_center * hue_bias * 1.,
                sat_min + pct_dist_to_center * sat_bias,
                lum_min + pct_dist_to_center * lum_bias / 2.0,
                0.05 + (y.abs() / window.h()),
            ));
        }

        for p in model.particles.iter() {
            let _charge = p.charge();
            if !cfg.draw_neutral && _charge == 0 {
                continue;
            }

            let path_len = p.path.len();

            draw.path()
                .stroke()
                .caps_round()
                .join_round()
                .weight(6.0)
                .points_colored(p.path.iter().enumerate().map(|(i, pos)| {
                    let pct_dist_to_head = i as f32 / path_len as f32;
                    (
                        pt3(pos[0], pos[1], pos[2]),
                        color::hsla(0.06, 0.82, 0.57, pct_dist_to_head),
                    )
                }));

            draw.path()
                .stroke()
                .caps_round()
                .join_round()
                .weight(2.0)
                .points_colored(p.path.iter().enumerate().map(|(i, pos)| {
                    let pct_dist_to_head = i as f32 / path_len as f32;
                    (
                        pt3(pos[0], pos[1], pos[2]),
                        color::hsla(0.09, 0.90, 0.77, pct_dist_to_head + 0.2),
                    )
                }));
        }
    }
}
