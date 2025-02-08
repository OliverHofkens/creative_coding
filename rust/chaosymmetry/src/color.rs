use serde::Deserialize;

pub mod palette;
pub mod scale;

#[derive(Deserialize)]
pub struct ColorConfig {
    pub palette: Box<dyn palette::Palette>,
    pub scale: Box<dyn scale::ColorScale>,
}
