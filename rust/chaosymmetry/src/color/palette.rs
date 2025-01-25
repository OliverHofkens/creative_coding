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
    colors: Vec<[u8; 4]>,
    stops: Vec<f64>,
}

impl NaiveGradient {
    pub fn new(colors: Vec<[u8; 4]>, mut stops: Vec<f64>) -> Self {
        // TODO: Make this a zero-cost abstraction with types.
        if colors.len() != stops.len() + 2 {
            panic!("Provide N+2 colors for N stops. Stops at 0 and 100% are implicit.");
        }
        if !stops.is_sorted() {
            panic!("Stops should be sorted from low to high.");
        }
        if *stops.first().unwrap_or(&0.01) <= 0.0 || *stops.last().unwrap_or(&0.99) >= 1.0 {
            panic!("Stops should be between 0.0 and 1.0. Stops at 0 and 100% are implicit.");
        }
        stops.insert(0, 0.0);
        stops.push(1.0);

        Self { colors, stops }
    }
}

impl Palette for NaiveGradient {
    fn color_from_scale(&self, scale: f64) -> [u8; 4] {
        let end_idx = self.stops.iter().position(|stop| *stop >= scale).unwrap();
        let start_idx = end_idx.saturating_sub(1);

        let start = self.stops[start_idx];
        let stop = self.stops[end_idx];
        let color_start = self.colors[start_idx];
        let color_end = self.colors[end_idx];

        let rescale = (scale - start) / (stop - start);

        let res: Vec<u8> = color_end
            .iter()
            .zip(color_start.iter())
            .map(|(e, s)| (*s as f64 + (rescale * (*e as f64 - *s as f64))) as u8)
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
        let coloring = NaiveGradient::new(vec![[u8::MIN; 4], [u8::MAX; 4]], vec![]);

        assert_eq!(coloring.color_from_scale(0.0), [u8::MIN; 4]);
        assert_eq!(coloring.color_from_scale(0.5), [127; 4]);
        assert_eq!(coloring.color_from_scale(1.0), [u8::MAX; 4]);
    }

    #[test]
    fn linear_gradient_three_stops() {
        let coloring =
            NaiveGradient::new(vec![[u8::MIN; 4], [u8::MAX; 4], [u8::MIN; 4]], vec![0.5]);

        assert_eq!(coloring.color_from_scale(0.0), [u8::MIN; 4]);
        assert_eq!(coloring.color_from_scale(0.25), [127; 4]);
        assert_eq!(coloring.color_from_scale(0.5), [u8::MAX; 4]);
        assert_eq!(coloring.color_from_scale(0.75), [127; 4]);
        assert_eq!(coloring.color_from_scale(1.0), [u8::MIN; 4]);
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
