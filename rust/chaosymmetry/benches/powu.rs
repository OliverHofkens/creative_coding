use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use num::complex::Complex64;

// ---------------------------------------------------------------------------
// Parameters — taken directly from config/book/sunflower.toml (NonPolyIcon,
// Dihedral = 57, singularity = 0), the config used for the profile run that
// showed 88% of simulation time in powu.
// ---------------------------------------------------------------------------

struct Params {
    lambda: f64,
    alpha: f64,
    beta: f64,
    gamma: f64,
    delta: f64,
    singularity: u32,
}

const SUNFLOWER: Params = Params {
    lambda: -2.225,
    alpha: 1.5,
    beta: 0.014,
    gamma: -0.002,
    delta: -0.02,
    singularity: 0,
};

// Fixed input point within the attractor basin — not zero, not on the unit
// circle, so norm() can't be trivially elided by the compiler.
const Z: Complex64 = Complex64::new(0.5, 0.3);

// ---------------------------------------------------------------------------
// Variant A — baseline
// Exact current NonPolyIcon::next body
// Two independent powu calls for t3/t5 (the dominant cost), one for t4,
// one full complex multiply for t2.
// ---------------------------------------------------------------------------

#[inline(never)]
fn next_baseline(curr: Complex64, p: u32, params: &Params) -> Complex64 {
    let t1 = params.lambda;
    let t2 = params.alpha * curr * curr.conj();
    let t3 = params.beta * curr.powu(p).re;

    let curr_norm = curr.norm();
    let t4 = params.delta * (curr / curr_norm).powu(p * params.singularity).re * curr_norm;

    let t5 = params.gamma * curr.conj().powu(p - 1);

    (t1 + t2 + t3 + t4) * curr + t5
}

// ---------------------------------------------------------------------------
// Variant B — fused binary exp + norm_sqr
// Compute z^(p-1) once via powu; derive z^p with one extra complex multiply
// and conj(z^(p-1)) for free. Halves the powu cost for t3/t5.
// t2 uses norm_sqr() (re*re + im*im) instead of a complex multiply.
// t4 is unchanged — its exponent (p * singularity) is independent, binary
// exp is the right algorithm there.
//
// Note: when p == 1, powu(0) returns Complex::one() immediately — correct,
// but none of the benchmark degrees exercise that path.
// ---------------------------------------------------------------------------

#[inline(never)]
fn next_fused(curr: Complex64, p: u32, params: &Params) -> Complex64 {
    let t1 = params.lambda;

    // t2: |z|^2 is purely real — two f64 multiplies and an add, no sqrt.
    let t2 = params.alpha * curr.norm_sqr();

    // One powu call for z^(p-1); derive z^p and conj(z^(p-1)) from it.
    let zp1 = curr.powu(p - 1); // z^(p-1)
    let t3 = params.beta * (zp1 * curr).re; // Re(z^p) = Re(z^(p-1) * z)
    let t5 = params.gamma * zp1.conj(); // conj(z^(p-1)) = z̄^(p-1)

    // t4: unchanged — independent exponent, keep binary exp.
    let curr_norm = curr.norm();
    let t4 = params.delta * (curr / curr_norm).powu(p * params.singularity).re * curr_norm;

    (t1 + t2 + t3 + t4) * curr + t5
}

// ---------------------------------------------------------------------------
// Variant C — norm_sqr only
// Only replace t2's complex multiply with norm_sqr(). Both powu calls for
// t3/t5 are unchanged. Isolates the t2 contribution from the powu fusion so
// the two effects can be read independently from the results.
// ---------------------------------------------------------------------------

#[inline(never)]
fn next_norm_sqr_only(curr: Complex64, p: u32, params: &Params) -> Complex64 {
    let t1 = params.lambda;
    let t2 = params.alpha * curr.norm_sqr(); // only change vs baseline

    let t3 = params.beta * curr.powu(p).re;

    let curr_norm = curr.norm();
    let t4 = params.delta * (curr / curr_norm).powu(p * params.singularity).re * curr_norm;

    let t5 = params.gamma * curr.conj().powu(p - 1);

    (t1 + t2 + t3 + t4) * curr + t5
}

// ---------------------------------------------------------------------------
// Variant D — De Moivre (optimised shared polar form)
// Single polar decomposition: one norm() (sqrt), one arg() (atan2), one
// powf() for r^p. All power terms share r and θ.
//
// t3: Re(z^p)      = r^p  * cos(p·θ)
// t5: z̄^(p-1)     = r^(p-1) * (cos((p-1)·θ) - i·sin((p-1)·θ))
//                  [conjugate negates the angle]
// t4: Re((z/|z|)^(p·s)) * |z| = cos(p·s·θ) * r
//                  [z/|z| has unit norm so r cancels, only angle remains]
//
// r^(p-1) = r^p / r — one f64 divide, avoids a second powf call.
// t2 written as alpha * norm * norm to reuse the already-computed norm
// without an additional method call.
//
// The formula is general (singularity stays in the exponent as-is).
// When singularity == 0, cos(0) == 1.0 at runtime — no special-casing.
// ---------------------------------------------------------------------------

#[inline(never)]
fn next_de_moivre(curr: Complex64, p: u32, params: &Params) -> Complex64 {
    let t1 = params.lambda;

    let norm = curr.norm(); // hypot — one sqrt
    let theta = curr.arg(); // atan2

    // t2: reuse norm already in hand.
    let t2 = params.alpha * norm * norm;

    // Shared: r^p computed once; r^(p-1) derived with one f64 divide.
    let rp = norm.powf(p as f64);
    let rp1 = rp / norm; // r^(p-1)

    // t3: Re(z^p) = r^p * cos(p·θ)
    let t3 = params.beta * rp * (p as f64 * theta).cos();

    // t5: z̄^(p-1) = r^(p-1) * e^(-i·(p-1)·θ)
    let pm1_theta = (p as f64 - 1.0) * theta;
    let t5 = params.gamma
        * Complex64::new(
            rp1 * pm1_theta.cos(),
            -rp1 * pm1_theta.sin(), // negative: conjugate negates the imaginary part
        );

    // t4: (z/|z|)^(p·s) has unit norm, so Re(…) = cos(p·s·θ), then * |z|.
    let ps_theta = (p * params.singularity) as f64 * theta;
    let t4 = params.delta * ps_theta.cos() * norm;

    (t1 + t2 + t3 + t4) * curr + t5
}

// ---------------------------------------------------------------------------
// Benchmark
// ---------------------------------------------------------------------------

fn bench_powu_next(c: &mut Criterion) {
    let degrees: &[u32] = &[3, 5, 6, 9, 57];

    let mut g = c.benchmark_group("powu_next");

    for &p in degrees {
        let z = black_box(Z);
        let params = &SUNFLOWER;

        g.bench_with_input(BenchmarkId::new("baseline", p), &p, |b, &p| {
            b.iter(|| black_box(next_baseline(z, p, params)))
        });

        g.bench_with_input(BenchmarkId::new("fused", p), &p, |b, &p| {
            b.iter(|| black_box(next_fused(z, p, params)))
        });

        g.bench_with_input(BenchmarkId::new("norm_sqr_only", p), &p, |b, &p| {
            b.iter(|| black_box(next_norm_sqr_only(z, p, params)))
        });

        g.bench_with_input(BenchmarkId::new("de_moivre", p), &p, |b, &p| {
            b.iter(|| black_box(next_de_moivre(z, p, params)))
        });
    }

    g.finish();
}

criterion_group!(benches, bench_powu_next);
criterion_main!(benches);
