use num::complex::Complex64;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

use crate::symmetry::Symmetry;

#[typetag::serde(tag = "type")]
pub trait Figure: Send {
    fn next(&self, curr: Complex64) -> Complex64;
    fn get_scale(&self) -> usize;
}

#[derive(Serialize, Deserialize)]
pub struct StandardIcon {
    lambda: f64,
    alpha: f64,
    beta: f64,
    gamma: f64,
    omega: f64,
    symmetry: Symmetry,
    scale: usize,
}

#[typetag::serde]
impl Figure for StandardIcon {
    fn next(&self, curr: Complex64) -> Complex64 {
        let symm_deg = self.symmetry.get_degree();
        let t1 = self.lambda;
        let t2 = self.alpha * curr * curr.conj();
        let t3 = self.beta * curr.powu(symm_deg).re;
        let t4 = self.omega * Complex64::I;

        let t5 = self.gamma * curr.conj().powu(symm_deg - 1);

        (t1 + t2 + t3 + t4) * curr + t5

        // if res.is_nan() || res.is_infinite() {
        //     panic!("{curr} -> ({t1} + {t2} + {t3} + {t4})z + {t5}");
        // }
    }
    fn get_scale(&self) -> usize {
        self.scale
    }
}

#[derive(Serialize, Deserialize)]
pub struct NonPolyIcon {
    lambda: f64,
    alpha: f64,
    beta: f64,
    gamma: f64,
    delta: f64,
    singularity: u32,
    symmetry: Symmetry,
    scale: usize,
}

#[typetag::serde]
impl Figure for NonPolyIcon {
    fn next(&self, curr: Complex64) -> Complex64 {
        let symm_deg = self.symmetry.get_degree();
        let t1 = self.lambda;
        let t2 = self.alpha * curr * curr.conj();
        let t3 = self.beta * curr.powu(symm_deg).re;

        let t4 =
            self.delta * (curr / curr.norm()).powu(symm_deg * self.singularity).re * curr.norm();

        let t5 = self.gamma * curr.conj().powu(symm_deg - 1);

        (t1 + t2 + t3 + t4) * curr + t5

        // if res.is_nan() || res.is_infinite() {
        //     panic!("{curr} -> ({t1} + {t2} + {t3} + {t4})z + {t5}");
        // }
    }
    fn get_scale(&self) -> usize {
        self.scale
    }
}

#[derive(Serialize, Deserialize)]
pub struct SymmetricFractal {
    a11: f64,
    a12: f64,
    a21: f64,
    a22: f64,
    b1: f64,
    b2: f64,
    symmetry: Symmetry,
    scale: usize,
    // symm_deg: usize,
    // vertices: Vec<Complex64>,
}

#[typetag::serde]
impl Figure for SymmetricFractal {
    fn next(&self, curr: Complex64) -> Complex64 {
        // let mut rng = rand::rng();
        // let vertex = self.vertices.choose(&mut rng).unwrap();

        // let x = self.a11 * vertex.re + self.a12 * vertex.im + self.b1 * curr.re;
        // let y = self.a21 * vertex.re + self.a22 * vertex.im + self.b2 * curr.im;
        let x = self.a11 * curr.re + self.a12 * curr.im + self.b1;
        let y = self.a21 * curr.re + self.a22 * curr.im + self.b2;
        let res = Complex64::new(x, y);
        self.symmetry.apply_random(res)
    }
    fn get_scale(&self) -> usize {
        self.scale
    }
}

fn generate_equilateral_polygon_vertices(sides: usize, radius: usize) -> Vec<Complex64> {
    let theta = (2.0 * PI) / sides as f64;
    (0..sides)
        .map(|n| {
            Complex64::new(
                radius as f64 * (n as f64 * theta).cos(),
                radius as f64 * (n as f64 * theta).sin(),
            )
        })
        .collect()
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_square_vertices() {
        let result = generate_equilateral_polygon_vertices(4, 1);

        assert_eq!(result[0], Complex64::new(1.0, 0.0));
        // Account for floating point error
        assert!((result[1] - Complex64::new(0.0, 1.0)).norm() < 0.00001);
        assert!((result[2] - Complex64::new(-1.0, 0.0)).norm() < 0.00001);
        assert!((result[3] - Complex64::new(0.0, -1.0)).norm() < 0.00001);
    }

    #[test]
    fn test_triangle_vertices() {
        let result = generate_equilateral_polygon_vertices(3, 1);

        assert_eq!(result[0], Complex64::new(1.0, 0.0));
        // Account for floating point error
        assert!((result[1] - Complex64::new(-0.5, 3.0f64.sqrt() / 2.0)).norm() < 0.00001);
        assert!((result[2] - Complex64::new(-0.5, -1.0 * 3.0f64.sqrt() / 2.0)).norm() < 0.00001);
    }
}
