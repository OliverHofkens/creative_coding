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
