use std::f64::consts::PI;

use num::complex::Complex64;

pub struct StandardIcon {
    lambda: f64,
    alpha: f64,
    beta: f64,
    gamma: f64,
    omega: f64,
    symm_deg: u32,
}

impl StandardIcon {
    pub fn new(lambda: f64, alpha: f64, beta: f64, gamma: f64, omega: f64, symm_deg: u32) -> Self {
        StandardIcon {
            lambda,
            alpha,
            beta,
            gamma,
            omega,
            symm_deg,
        }
    }

    pub fn next(&self, curr: Complex64) -> Complex64 {
        let t1 = self.lambda;
        let t2 = self.alpha * curr * curr.conj();
        let t3 = self.beta * curr.powu(self.symm_deg).re;
        let t4 = self.omega * Complex64::I;

        let t5 = self.gamma * curr.conj().powu(self.symm_deg - 1);

        let res = (t1 + t2 + t3 + t4) * curr + t5;

        // if res.is_nan() || res.is_infinite() {
        //     panic!("{curr} -> ({t1} + {t2} + {t3} + {t4})z + {t5}");
        // }
        res
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
    fn test_triangle_vertices() {
        let result = generate_equilateral_polygon_vertices(3, 1);

        assert_eq!(result[0], Complex64::new(1.0, 0.0));
        // Account for floating point error
        assert!((result[1] - Complex64::new(-0.5, 3.0f64.sqrt() / 2.0)).norm() < 0.00001);
        assert!((result[2] - Complex64::new(-0.5, -1.0 * 3.0f64.sqrt() / 2.0)).norm() < 0.00001);
    }
}
