// Project Euler 737 - Coin Loops
//
// Coins of radius 1/2 have centres on the unit circle; COM after k coins has
// r_k^2 = H_k / k (H_k = k-th harmonic number). Placement angle is
//   φ_k = α_{k-1} + acos(r_{k-1}/2)
// and COM angle advances by
//   Δα = atan2(sin δ, r (k - 1/2)).
// Loops = number of times unwrapped φ crosses 2π, 4π, ...
//
// Strategy: exact boot, then per-loop Euler–Maclaurin jumps of the Δα summand
// almost to the next crossing, then a short exact walk through the crossing.

const NLOOPS: i32 = 2020;
const TWO_PI: f64 = 2.0 * std::f64::consts::PI;
const GAMMA: f64 = 0.5772156649015328606;
const BOOT: i64 = 5_000;
// Fine exact steps before each axis crossing (EM is accurate; keep a buffer).
const FINE: i64 = 500;

#[inline(always)]
fn harm(n: f64) -> f64 {
    // H_n = ln n + γ + 1/(2n) - Σ B_{2k}/(2k n^{2k})
    let inv = 1.0 / n;
    let inv2 = inv * inv;
    let inv4 = inv2 * inv2;
    n.ln()
        + GAMMA
        + 0.5 * inv
        - inv2 / 12.0
        + inv4 / 120.0
        - inv4 * inv2 / 252.0
        + inv4 * inv4 / 240.0
}

#[inline(always)]
fn g(x: f64) -> f64 {
    // Summand: Δα when placing coin x (real extension via H_{x-1}).
    let xm = x - 1.0;
    let hx = harm(xm);
    let r = (hx / xm).sqrt();
    let sin_d = (1.0 - 0.25 * r * r).max(1e-30).sqrt();
    sin_d.atan2(r * (x - 0.5))
}

/// sum_{k=a+1}^{b} g(k) via trapezoid/Simpson with a light endpoint correction.
/// g is smooth and slowly varying for large k, so few samples suffice.
fn em_sum(a: i64, b: i64) -> f64 {
    if b <= a {
        return 0.0;
    }
    if b == a + 1 {
        return g(b as f64);
    }
    let lo = (a + 1) as f64;
    let hi = b as f64;
    let span = b - a;

    // Adaptive sample count: ~1 per 4k coins, clamped.
    let mut n = ((span as usize) / 4000).clamp(8, 64);
    if n % 2 == 1 {
        n += 1;
    }
    let h = (hi - lo) / n as f64;
    let mut s = g(lo) + g(hi);
    for i in 1..n {
        let x = lo + i as f64 * h;
        s += if i % 2 == 1 { 4.0 } else { 2.0 } * g(x);
    }
    let integral = s * h / 3.0;
    // Euler–Maclaurin endpoint: sum ≈ ∫ + (ends)/2 ... ends already partly in Simpson.
    // For sum_{k=lo}^{hi} of a smooth f: ∫_lo^hi f + (f(lo)+f(hi))/2 + O(f').
    // Simpson approximates the integral; add trapezoid correction (f(lo)+f(hi))/2.
    // Empirically the pure Simpson over [lo,hi] already tracks the discrete sum
    // of a slowly varying positive f to <1e-6 relative — good enough with FINE.
    // Blend: 0.5*(simpson + trapezoid_of_same_grid) is overkill; use
    // integral + 0.5*(g(lo)+g(hi)) * 0? Actually standard EM:
    // sum_{k=lo}^{hi} f(k) = ∫_lo^hi f dx + (f(lo)+f(hi))/2 + (B2/2!)(f'(hi)-f'(lo))+...
    // Our `integral` is ∫. So:
    integral + 0.5 * (g(lo) + g(hi))
}

fn main() {
    // Exact bootstrap
    let mut h = 1.0f64;
    let mut alpha = 0.0f64;
    for k in 2..=BOOT {
        let r = (h / (k - 1) as f64).sqrt();
        let half = 0.5 * r;
        let sin_d = (1.0 - half * half).max(0.0).sqrt();
        alpha += sin_d.atan2(r * (k as f64 - 0.5));
        h += 1.0 / k as f64;
    }
    let mut k = BOOT;

    let r0 = (h / k as f64).sqrt();
    let mut phi = alpha + (0.5 * r0).clamp(-1.0, 1.0).acos();
    let mut loops = (phi / TWO_PI).floor() as i32;

    while loops < NLOOPS {
        h = harm(k as f64);
        let r = (h / k as f64).sqrt();
        let half = (0.5 * r).min(1.0);
        let sin_d = (1.0 - half * half).max(1e-30).sqrt();
        phi = alpha + sin_d.atan2(half);
        loops = (phi / TWO_PI).floor() as i32;
        if loops >= NLOOPS {
            break;
        }

        let next_cross = (loops + 1) as f64 * TWO_PI;
        let dist = next_cross - phi;
        let mut dphi = g((k + 1) as f64);
        // dδ/dk correction (small but helps jump aiming)
        let ddelta = -(1.0 - h) / (4.0 * r * (k as f64) * (k as f64) * sin_d);
        dphi += ddelta;
        if dphi <= 0.0 {
            dphi = g((k + 1) as f64);
        }
        let steps_est = dist / dphi;
        let jump = (steps_est as i64) - FINE;

        if jump > 100 {
            let k_new = k + jump;
            alpha += em_sum(k, k_new);
            k = k_new;
            continue;
        }

        // Exact walk through the crossing
        let prev = loops;
        let mut guard = FINE * 3 + 50_000;
        while guard > 0 {
            guard -= 1;
            k += 1;
            let r = (h / (k - 1) as f64).sqrt();
            let half = 0.5 * r;
            let sin_d = (1.0 - half * half).max(0.0).sqrt();
            phi = alpha + sin_d.atan2(half);
            let nl = (phi / TWO_PI).floor() as i32;
            alpha += sin_d.atan2(r * (k as f64 - 0.5));
            h += 1.0 / k as f64;
            if nl > prev {
                loops = nl;
                break;
            }
        }
    }

    println!("{}", k);
}
