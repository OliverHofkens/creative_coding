use std::fs;
use std::path::PathBuf;

use clap::Parser;
use num::complex::Complex64;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use chaosymmetry::figures::Figure;
use chaosymmetry::sweep::{build_nonpoly, build_standard, sample_nonpoly, sample_standard};
use chaosymmetry::symmetry::Symmetry;

/// Iterations discarded as transient before testing begins.
const WARMUP_STEPS: usize = 5_000;

/// Iterations used to accumulate the hit-count grid for scoring.
const TEST_STEPS: usize = 100_000;

/// Escape radius — orbits that exceed this are rejected.
const ESCAPE_RADIUS: f64 = 1e6;

/// Fixed-point detection: reject if variance over the last N steps is below this.
const FIXED_POINT_VARIANCE_THRESHOLD: f64 = 1e-10;
const FIXED_POINT_WINDOW: usize = 100;

/// Coarse grid side length for entropy scoring (grid is N×N).
const GRID_SIZE: usize = 64;

/// Shannon entropy lower bound (nats). Below this the attractor is too degenerate.
const H_LOW: f64 = 2.0;

/// Shannon entropy upper bound (nats). Above this the attractor fills the grid
/// too uniformly. ln(64×64) ≈ 8.32, so this is effectively a safety valve only.
const H_HIGH: f64 = 9.5;

/// Minimum fraction of the GRID_SIZE×GRID_SIZE cells that must be occupied.
/// Filters thin 1D attractors (outlines, rings, spokes) that score well on
/// entropy but have negligible 2D support. Every figure in the book collection
/// scores ≥ 9.7%; hexagons and similar outlines score ~6%.
const FILL_MIN: f64 = 0.12;

/// Target fraction of the sim grid dimension the attractor should fill.
const TARGET_FILL: f64 = 0.8;

/// Simulation grid dimension in pixels (must match SIM_WIDTH / SIM_HEIGHT in main.rs).
const SIM_SIZE: f64 = 10_000.0;

/// Computed scale is clamped to this range to guard against degenerate orbits
/// that somehow pass the fixed-point check.
const SCALE_MIN: usize = 500;
const SCALE_MAX: usize = 500_000;

#[derive(Parser)]
#[command(
    name = "sweep",
    about = "Randomly sweep for interesting chaos attractors"
)]
struct Cli {
    /// Number of candidates to test.
    #[arg(short = 'n', long, default_value_t = 1000)]
    count: usize,

    /// Symmetry degrees to include (repeat for multiple, e.g. -d 3 -d 5).
    #[arg(short = 'd', long = "degree", default_values_t = vec![3u32, 4, 5, 6, 7, 8])]
    degrees: Vec<u32>,

    /// Only generate Dihedral symmetry.
    #[arg(long, conflicts_with = "cyclic_only")]
    dihedral_only: bool,

    /// Only generate Cyclic symmetry.
    #[arg(long, conflicts_with = "dihedral_only")]
    cyclic_only: bool,

    /// Figure type(s) to sweep: standard, nonpoly, or both.
    #[arg(short = 't', long = "type", default_value = "both")]
    figure_type: FigureType,

    /// Directory to write accepted TOML files into.
    #[arg(short = 'o', long, default_value = "config/custom")]
    out_dir: PathBuf,

    /// RNG seed for reproducibility (omit for a random seed).
    #[arg(short = 's', long)]
    seed: Option<u64>,

    /// Stop early once this many attractors have been accepted.
    #[arg(short = 'm', long)]
    min_accepted: Option<usize>,

    /// Search algorithm: random | de.
    #[arg(long, default_value = "random")]
    algorithm: Algorithm,
}

#[derive(Clone, clap::ValueEnum)]
enum FigureType {
    Standard,
    Nonpoly,
    Both,
}

#[derive(Clone, clap::ValueEnum)]
enum Algorithm {
    Random,
    De,
}

#[derive(Clone, Copy, PartialEq)]
enum SymmetryKind {
    Cyclic,
    Dihedral,
}

fn build_symmetry_variants(cli: &Cli) -> Vec<(SymmetryKind, u32)> {
    let mut variants = Vec::new();
    for &deg in &cli.degrees {
        if !cli.cyclic_only {
            variants.push((SymmetryKind::Dihedral, deg));
        }
        if !cli.dihedral_only {
            variants.push((SymmetryKind::Cyclic, deg));
        }
    }
    variants
}

fn make_symmetry(kind: SymmetryKind, degree: u32) -> Symmetry {
    match kind {
        SymmetryKind::Dihedral => Symmetry::dihedral(degree),
        SymmetryKind::Cyclic => Symmetry::cyclic(degree),
    }
}

fn main() {
    let cli = Cli::parse();

    if matches!(cli.algorithm, Algorithm::De) {
        eprintln!("error: --algorithm de is not yet implemented.");
        std::process::exit(1);
    }

    let mut rng = match cli.seed {
        Some(s) => SmallRng::seed_from_u64(s),
        None => SmallRng::from_rng(&mut rand::rng()),
    };

    fs::create_dir_all(&cli.out_dir).expect("failed to create output directory");

    let symmetry_variants = build_symmetry_variants(&cli);
    if symmetry_variants.is_empty() {
        eprintln!("error: no symmetry variants (check --degree / --dihedral-only / --cyclic-only)");
        std::process::exit(1);
    }

    let use_standard = matches!(cli.figure_type, FigureType::Standard | FigureType::Both);
    let use_nonpoly = matches!(cli.figure_type, FigureType::Nonpoly | FigureType::Both);

    println!(
        "Sweeping {} candidates: {}  degrees {:?}  symmetry {}",
        cli.count,
        match cli.figure_type {
            FigureType::Standard => "StandardIcon",
            FigureType::Nonpoly => "NonPolyIcon",
            FigureType::Both => "StandardIcon + NonPolyIcon",
        },
        cli.degrees,
        if cli.dihedral_only {
            "Dihedral only"
        } else if cli.cyclic_only {
            "Cyclic only"
        } else {
            "Dihedral + Cyclic"
        },
    );

    let mut accepted = 0usize;
    let mut tested = 0usize;

    for i in 0..cli.count {
        let (kind, degree) = symmetry_variants[rng.random_range(0..symmetry_variants.len())];

        let use_standard_this = if use_standard && use_nonpoly {
            rng.random_bool(0.5)
        } else {
            use_standard
        };

        let type_label = if use_standard_this {
            "standard"
        } else {
            "nonpoly"
        };
        let symm_label = if kind == SymmetryKind::Dihedral {
            format!("dihedral{degree}")
        } else {
            format!("cyclic{degree}")
        };

        tested += 1;

        // Sample parameters. Build a probe with scale=1 so orbit coordinates are
        // in natural attractor units, letting evaluate() measure the true extent
        // and compute the correct scale. Then re-build with that scale.
        if use_standard_this {
            let params = sample_standard(&mut rng);
            let probe = build_standard(&params, make_symmetry(kind, degree), 1);
            match evaluate(&*probe, &mut rng) {
                EvalResult::Reject(reason) => print_reject(i, reason),
                EvalResult::Accept {
                    entropy,
                    fill,
                    scale,
                } => {
                    accepted += 1;
                    let figure = build_standard(&params, make_symmetry(kind, degree), scale);
                    write_accepted(
                        i,
                        &cli,
                        type_label,
                        &symm_label,
                        entropy,
                        fill,
                        scale,
                        accepted,
                        &figure,
                    );
                }
            }
        } else {
            let params = sample_nonpoly(&mut rng);
            let probe = build_nonpoly(&params, make_symmetry(kind, degree), 1);
            match evaluate(&*probe, &mut rng) {
                EvalResult::Reject(reason) => print_reject(i, reason),
                EvalResult::Accept {
                    entropy,
                    fill,
                    scale,
                } => {
                    accepted += 1;
                    let figure = build_nonpoly(&params, make_symmetry(kind, degree), scale);
                    write_accepted(
                        i,
                        &cli,
                        type_label,
                        &symm_label,
                        entropy,
                        fill,
                        scale,
                        accepted,
                        &figure,
                    );
                }
            }
        }

        if let Some(min) = cli.min_accepted {
            if accepted >= min {
                println!("Reached --min-accepted {min}, stopping early.");
                break;
            }
        }
    }

    println!(
        "\nDone. Tested: {tested}  Accepted: {accepted}  Written to: {}",
        cli.out_dir.display(),
    );
}

fn print_reject(i: usize, reason: RejectReason) {
    match reason {
        RejectReason::Escape => println!("[{i:5}] REJECT  escape"),
        RejectReason::FixedPoint => println!("[{i:5}] REJECT  fixed_point"),
        RejectReason::Degenerate(h) => println!("[{i:5}] REJECT  degenerate   H={h:.2}"),
        RejectReason::Uniform(h) => println!("[{i:5}] REJECT  too_uniform  H={h:.2}"),
        RejectReason::ThinSupport(f) => {
            println!("[{i:5}] REJECT  thin_support fill={:.1}%", f * 100.0)
        }
    }
}

fn write_accepted(
    i: usize,
    cli: &Cli,
    type_label: &str,
    symm_label: &str,
    entropy: f64,
    fill: f64,
    scale: usize,
    accepted: usize,
    figure: &Box<dyn Figure>,
) {
    let filename = cli.out_dir.join(format!(
        "sweep_{type_label}_{symm_label}_{accepted:04}.toml"
    ));
    let toml_str = toml::to_string_pretty(figure).expect("failed to serialise figure");
    fs::write(&filename, &toml_str).expect("failed to write TOML");
    println!(
        "[{i:5}] ACCEPT  {type_label:<10} {symm_label:<12}  H={entropy:.2}  fill={:.1}%  scale={scale}  → {}",
        fill * 100.0,
        filename.display()
    );
}

enum RejectReason {
    Escape,
    FixedPoint,
    Degenerate(f64),
    Uniform(f64),
    ThinSupport(f64),
}

enum EvalResult {
    Reject(RejectReason),
    Accept {
        entropy: f64,
        fill: f64,
        scale: usize,
    },
}

/// Evaluate a probe figure (built with scale=1 so coordinates are in natural
/// attractor units). Returns either a reject reason, or the entropy score and
/// auto-computed scale that fills TARGET_FILL of SIM_SIZE.
fn evaluate(figure: &dyn Figure, rng: &mut SmallRng) -> EvalResult {
    // Use an asymmetric starting point. (0.1, 0.1) lies exactly on the re==im
    // diagonal, which is an invariant subspace for some symmetric figures. That
    // causes the orbit to stay 1D, passing the entropy check but producing a
    // degenerate figure the renderer (which starts from a random asymmetric
    // point) will never reproduce.
    let mut z = Complex64::new(0.1, 0.17);

    for _ in 0..WARMUP_STEPS {
        z = figure.next(z, rng);
        if !z.is_finite() || z.norm() > ESCAPE_RADIUS {
            return EvalResult::Reject(RejectReason::Escape);
        }
    }

    let mut history: Vec<Complex64> = Vec::with_capacity(TEST_STEPS);
    let mut x_min = f64::MAX;
    let mut x_max = f64::MIN;
    let mut y_min = f64::MAX;
    let mut y_max = f64::MIN;

    for _ in 0..TEST_STEPS {
        z = figure.next(z, rng);
        if !z.is_finite() || z.norm() > ESCAPE_RADIUS {
            return EvalResult::Reject(RejectReason::Escape);
        }
        history.push(z);
        if z.re < x_min {
            x_min = z.re;
        }
        if z.re > x_max {
            x_max = z.re;
        }
        if z.im < y_min {
            y_min = z.im;
        }
        if z.im > y_max {
            y_max = z.im;
        }
    }

    // Fixed-point check: variance of the tail window.
    let tail = &history[history.len().saturating_sub(FIXED_POINT_WINDOW)..];
    let mean_re = tail.iter().map(|p| p.re).sum::<f64>() / tail.len() as f64;
    let mean_im = tail.iter().map(|p| p.im).sum::<f64>() / tail.len() as f64;
    let variance = tail
        .iter()
        .map(|p| (p.re - mean_re).powi(2) + (p.im - mean_im).powi(2))
        .sum::<f64>()
        / tail.len() as f64;
    if variance < FIXED_POINT_VARIANCE_THRESHOLD {
        return EvalResult::Reject(RejectReason::FixedPoint);
    }

    // Auto-compute scale: make the attractor fill TARGET_FILL of the sim grid.
    // The probe was built with scale=1, so coordinates are in natural units.
    let extent = (x_max - x_min).max(y_max - y_min).max(1e-12);
    let computed_scale = ((SIM_SIZE * TARGET_FILL) / extent).round() as usize;
    let computed_scale = computed_scale.clamp(SCALE_MIN, SCALE_MAX);

    // Accumulate hits into a GRID_SIZE×GRID_SIZE grid, normalised to bounding box.
    let x_range = (x_max - x_min).max(1e-12);
    let y_range = (y_max - y_min).max(1e-12);
    let mut grid = vec![0u32; GRID_SIZE * GRID_SIZE];
    for p in &history {
        let gx = ((p.re - x_min) / x_range * (GRID_SIZE - 1) as f64) as usize;
        let gy = ((p.im - y_min) / y_range * (GRID_SIZE - 1) as f64) as usize;
        grid[gy * GRID_SIZE + gx] += 1;
    }

    // Shannon entropy over occupied cells (nats).
    let total = TEST_STEPS as f64;
    let occupied = grid.iter().filter(|&&c| c > 0).count();
    let entropy: f64 = grid
        .iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / total;
            -p * p.ln()
        })
        .sum();

    // Fill fraction: occupied cells / total grid cells.
    // Filters 1D attractors (outlines, rings) that score well on entropy but
    // have thin 2D support — every book figure scores ≥ 9.7%, outlines ~6%.
    let fill = occupied as f64 / (GRID_SIZE * GRID_SIZE) as f64;

    if entropy < H_LOW {
        EvalResult::Reject(RejectReason::Degenerate(entropy))
    } else if entropy > H_HIGH {
        EvalResult::Reject(RejectReason::Uniform(entropy))
    } else if fill < FILL_MIN {
        EvalResult::Reject(RejectReason::ThinSupport(fill))
    } else {
        EvalResult::Accept {
            entropy,
            fill,
            scale: computed_scale,
        }
    }
}
