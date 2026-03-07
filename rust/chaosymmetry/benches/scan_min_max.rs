use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use itertools::MinMaxResult::{MinMax, NoElements, OneElement};

// ---------------------------------------------------------------------------
// Shared input generation
// ---------------------------------------------------------------------------

/// Builds a realistic freq map: ~50% of cells are zero (unvisited), the rest
/// are random u64s in a plausible hit-count range.
fn make_freq_map(width: usize, height: usize) -> Vec<u64> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let n = width * height;
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        // Cheap deterministic "random" without pulling in rand as a dev-dep.
        let mut h = DefaultHasher::new();
        i.hash(&mut h);
        let r = h.finish();
        // 50% is zero
        if r % 10 < 5 {
            v.push(0);
        } else {
            v.push((r % 1000) + 1);
        }
    }
    v
}

// ---------------------------------------------------------------------------
// Candidate implementations
// ---------------------------------------------------------------------------

fn minmax_itertools(freqs: &[u64]) -> (u64, u64) {
    match freqs.iter().filter(|v| **v > 0).minmax() {
        NoElements => (0, 0),
        OneElement(x) => (*x, *x),
        MinMax(x, y) => (*x, *y),
    }
}

fn minmax_manual_loop(freqs: &[u64]) -> (u64, u64) {
    let mut min = u64::MAX;
    let mut max = 0u64;
    for &v in freqs {
        if v > 0 {
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
        }
    }
    if max == 0 {
        (0, 0)
    } else {
        (min, max)
    }
}

fn minmax_two_pass(freqs: &[u64]) -> (u64, u64) {
    let min = freqs.iter().filter(|&&v| v > 0).min().copied().unwrap_or(0);
    let max = freqs.iter().copied().max().unwrap_or(0);
    (min, max)
}

fn minmax_fold(freqs: &[u64]) -> (u64, u64) {
    let (min, max) = freqs
        .iter()
        .filter(|&&v| v > 0)
        .fold((u64::MAX, 0u64), |(mn, mx), &v| (mn.min(v), mx.max(v)));
    if max == 0 {
        (0, 0)
    } else {
        (min, max)
    }
}

fn minmax_chunked(freqs: &[u64]) -> (u64, u64) {
    const CHUNK: usize = 1024;
    let (min, max) = freqs
        .chunks(CHUNK)
        .fold((u64::MAX, 0u64), |(mn, mx), chunk| {
            let (cmn, cmx) = chunk
                .iter()
                .filter(|&&v| v > 0)
                .fold((u64::MAX, 0u64), |(a, b), &v| (a.min(v), b.max(v)));
            (mn.min(cmn), mx.max(cmx))
        });
    if max == 0 {
        (0, 0)
    } else {
        (min, max)
    }
}

// ---------------------------------------------------------------------------
// Benchmarks
// ---------------------------------------------------------------------------

fn bench_scan_min_max(c: &mut Criterion) {
    // Use real simulation dimensions.
    const W: usize = 10_000;
    const H: usize = 10_000;

    let freqs = make_freq_map(W, H);

    let mut g = c.benchmark_group("scan_min_max");

    g.bench_function("itertools_minmax", |b| {
        b.iter(|| minmax_itertools(black_box(&freqs)))
    });

    g.bench_function("manual_loop", |b| {
        b.iter(|| minmax_manual_loop(black_box(&freqs)))
    });

    g.bench_function("two_pass", |b| {
        b.iter(|| minmax_two_pass(black_box(&freqs)))
    });

    g.bench_function("fold", |b| b.iter(|| minmax_fold(black_box(&freqs))));

    g.bench_function("chunked_fold", |b| {
        b.iter(|| minmax_chunked(black_box(&freqs)))
    });

    g.finish();
}

criterion_group!(benches, bench_scan_min_max);
criterion_main!(benches);
