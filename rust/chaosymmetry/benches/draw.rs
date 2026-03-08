use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

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
        let mut h = DefaultHasher::new();
        i.hash(&mut h);
        let r = h.finish();
        if r % 10 < 5 {
            v.push(0);
        } else {
            v.push((r % 1000) + 1);
        }
    }
    v
}

// ---------------------------------------------------------------------------
// Variant A — original iterator-chain approach (from before the refactor)
// ---------------------------------------------------------------------------

fn sum_block_iter(
    freqs: &[u64],
    sim_width: usize,
    sim_start_x: i64,
    sim_start_y: i64,
    freqs_per_px: i64,
    sim_height: i64,
) -> u64 {
    let sim_x1 = (sim_start_x + freqs_per_px).clamp(0, sim_width as i64 - 1) as usize;
    let sim_y1 = (sim_start_y + freqs_per_px).clamp(0, sim_height - 1) as usize;

    (sim_start_y.max(0)..sim_y1 as i64)
        .map(|row| {
            let row_offset = row as usize * sim_width;
            let start = row_offset + sim_start_x.max(0) as usize;
            let end = row_offset + sim_x1;
            if start < end {
                freqs[start..end].iter().sum::<u64>()
            } else {
                0
            }
        })
        .sum::<u64>()
}

// ---------------------------------------------------------------------------
// Variant B — current incremental-slice approach
// ---------------------------------------------------------------------------

fn sum_block_incremental(
    freqs: &[u64],
    sim_width: usize,
    x0: usize,
    x1: usize,
    y0: usize,
    y1: usize,
) -> u64 {
    let col_len = x1.saturating_sub(x0);
    if col_len == 0 || y0 >= y1 {
        return 0;
    }
    let mut res = 0u64;
    let mut row_start = y0 * sim_width + x0;
    for _ in y0..y1 {
        res += freqs[row_start..row_start + col_len].iter().sum::<u64>();
        row_start += sim_width;
    }
    res
}

// ---------------------------------------------------------------------------
// Variant C — summed-area table (from perf/summed-area-table branch)
// ---------------------------------------------------------------------------

struct SummedAreaTable {
    table: Vec<u64>,
    width: usize,
    height: usize,
}

impl SummedAreaTable {
    fn new(width: usize, height: usize) -> Self {
        Self {
            table: vec![0; (width + 1) * (height + 1)],
            width,
            height,
        }
    }

    fn init_from_map(&mut self, map: &[u64]) {
        let stride = self.width + 1;
        for row in 1..self.height + 1 {
            let map_row_offset = (row - 1) * self.width;
            let tbl_row_offset = row * stride;
            for col in 1..self.width + 1 {
                let map_idx = map_row_offset + col - 1;
                let tbl_idx = tbl_row_offset + col;
                self.table[tbl_idx] = map[map_idx]
                    .wrapping_add(self.table[tbl_idx - 1])
                    .wrapping_add(self.table[tbl_idx - stride])
                    .wrapping_sub(self.table[tbl_idx - stride - 1]);
            }
        }
    }

    /// Inclusive rectangle query: sums cells in [x0, x1] × [y0, y1].
    fn query(&self, x0: usize, y0: usize, x1: usize, y1: usize) -> u64 {
        let stride = self.width + 1;
        let x1 = x1 + 1;
        let y1 = y1 + 1;
        let idx_d = stride * y1 + x1;
        let idx_b = stride * y0 + x1;
        let idx_c = stride * y1 + x0;
        let idx_a = stride * y0 + x0;
        self.table[idx_d]
            .wrapping_sub(self.table[idx_b])
            .wrapping_sub(self.table[idx_c])
            .wrapping_add(self.table[idx_a])
    }
}

// ---------------------------------------------------------------------------
// Benchmarks
// ---------------------------------------------------------------------------

/// Per-query cost of each approach — what draw() pays per output pixel.
/// The SAT is pre-built outside the timed loop (its best case / amortised cost).
fn bench_block_sum_query(c: &mut Criterion) {
    const W: usize = 10_000;
    const H: usize = 10_000;

    let freqs = make_freq_map(W, H);

    // Query a tile in the middle of the sim — representative of the common case.
    let cx = W / 2;
    let cy = H / 2;

    // Pre-build SAT once; only query cost is timed below.
    let mut sat = SummedAreaTable::new(W, H);
    sat.init_from_map(&freqs);

    let mut g = c.benchmark_group("block_sum_query");

    for block_size in [2usize, 8, 32] {
        let x0 = cx;
        let x1 = cx + block_size;
        let y0 = cy;
        let y1 = cy + block_size;
        let fpp = block_size as i64;

        g.bench_with_input(BenchmarkId::new("iter", block_size), &block_size, |b, _| {
            b.iter(|| {
                sum_block_iter(
                    black_box(&freqs),
                    W,
                    black_box(cx as i64),
                    black_box(cy as i64),
                    black_box(fpp),
                    H as i64,
                )
            })
        });

        g.bench_with_input(
            BenchmarkId::new("incremental_slice", block_size),
            &block_size,
            |b, _| {
                b.iter(|| {
                    sum_block_incremental(
                        black_box(&freqs),
                        W,
                        black_box(x0),
                        black_box(x1),
                        black_box(y0),
                        black_box(y1),
                    )
                })
            },
        );

        // SAT query only — table already built, reflects amortised per-pixel cost.
        g.bench_with_input(
            BenchmarkId::new("sat_query", block_size),
            &block_size,
            |b, _| {
                // x1/y1 are inclusive in the SAT API.
                b.iter(|| black_box(sat.query(black_box(x0), black_box(y0), x1 - 1, y1 - 1)))
            },
        );
    }

    g.finish();
}

/// SAT build cost in isolation — the one-off overhead paid per frame.
/// Uses iter_batched so the table is reset between samples without touching
/// the timed section.
fn bench_sat_build(c: &mut Criterion) {
    const W: usize = 10_000;
    const H: usize = 10_000;

    let freqs = make_freq_map(W, H);

    c.bench_function("sat_build/10000x10000", |b| {
        b.iter_batched(
            || SummedAreaTable::new(W, H),
            |mut sat| sat.init_from_map(black_box(&freqs)),
            BatchSize::LargeInput,
        )
    });
}

criterion_group!(benches, bench_block_sum_query, bench_sat_build);
criterion_main!(benches);
