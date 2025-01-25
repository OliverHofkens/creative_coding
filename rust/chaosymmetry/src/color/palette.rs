pub trait Palette {
    fn color_from_scale(&self, scale: f64) -> [u8; 4];
}

#[derive(Default)]
pub struct Grayscale {}

impl Palette for Grayscale {
    fn color_from_scale(&self, scale: f64) -> [u8; 4] {
        let val = ((1.0 - scale) * u8::MAX as f64) as u8;
        [val, val, val, u8::MAX]
    }
}

pub struct NaiveGradient {
    color_start: [u8; 4],
    color_end: [u8; 4],
}

impl NaiveGradient {
    pub fn new(color_start: [u8; 4], color_end: [u8; 4]) -> Self {
        Self {
            color_start,
            color_end,
        }
    }
}

impl Palette for NaiveGradient {
    fn color_from_scale(&self, scale: f64) -> [u8; 4] {
        let res: Vec<u8> = self
            .color_end
            .iter()
            .zip(self.color_start.iter())
            .map(|(e, s)| (*s as f64 + (scale * (e - s) as f64)) as u8)
            .collect();
        res[..].try_into().unwrap()
    }
}

pub struct Buckets {
    colors: Vec<[u8; 4]>,
}

impl Buckets {
    pub fn new(colors: Vec<[u8; 4]>) -> Self {
        Buckets { colors }
    }
}

impl Palette for Buckets {
    fn color_from_scale(&self, scale: f64) -> [u8; 4] {
        let bucket = (scale * self.colors.len() as f64) as usize;
        self.colors[bucket.clamp(0, self.colors.len() - 1)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grayscale() {
        let coloring = Grayscale {};

        assert_eq!(coloring.color_from_scale(0.0), [u8::MAX; 4]);
        assert_eq!(coloring.color_from_scale(0.5), [127, 127, 127, u8::MAX]);
        assert_eq!(
            coloring.color_from_scale(1.0),
            [u8::MIN, u8::MIN, u8::MIN, u8::MAX]
        );
    }

    #[test]
    fn linear_gradient_grayscale() {
        let coloring = NaiveGradient::new([u8::MIN; 4], [u8::MAX; 4]);

        assert_eq!(coloring.color_from_scale(0.0), [u8::MIN; 4]);
        assert_eq!(coloring.color_from_scale(0.5), [127; 4]);
        assert_eq!(coloring.color_from_scale(1.0), [u8::MAX; 4]);
    }

    #[test]
    fn two_buckets() {
        let coloring = Buckets::new(vec![[u8::MIN; 4], [u8::MAX; 4]]);

        assert_eq!(coloring.color_from_scale(0.0), [u8::MIN; 4]);
        assert_eq!(coloring.color_from_scale(0.25), [u8::MIN; 4]);
        assert_eq!(coloring.color_from_scale(0.5), [u8::MAX; 4]);
        assert_eq!(coloring.color_from_scale(0.75), [u8::MAX; 4]);
        assert_eq!(coloring.color_from_scale(1.0), [u8::MAX; 4]);
    }

    #[test]
    fn three_buckets() {
        let coloring = Buckets::new(vec![[u8::MIN; 4], [127; 4], [u8::MAX; 4]]);

        assert_eq!(coloring.color_from_scale(0.0), [u8::MIN; 4]);
        assert_eq!(coloring.color_from_scale(0.25), [u8::MIN; 4]);
        assert_eq!(coloring.color_from_scale(0.5), [127; 4]);
        assert_eq!(coloring.color_from_scale(0.75), [u8::MAX; 4]);
        assert_eq!(coloring.color_from_scale(1.0), [u8::MAX; 4]);
    }
}
