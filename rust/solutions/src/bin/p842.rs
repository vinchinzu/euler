// Project Euler 842 - Irregular Star Polygons
// T(n) = sum of I(S) over all n-star polygons S.
// For odd n: all intersections have multiplicity 2.
// For even n: compute multiplicities geometrically.
// Sum T(n) for n=3..60 mod 10^9+7.

const MOD: u64 = 1_000_000_007;

fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { r = (r as u128 * base as u128 % m as u128) as u64; }
        base = (base as u128 * base as u128 % m as u128) as u64;
        exp >>= 1;
    }
    r
}

fn modinv(a: u64) -> u64 {
    pow_mod(a, MOD - 2, MOD)
}

fn main() {
    // Precompute modular factorials up to 61
    let mut fact_mod = [0u64; 62];
    fact_mod[0] = 1;
    for i in 1..=61usize {
        fact_mod[i] = fact_mod[i - 1] * i as u64 % MOD;
    }

    // Precompute inverse factorials
    let mut inv_fact = [0u64; 62];
    inv_fact[61] = modinv(fact_mod[61]);
    for i in (0..61).rev() {
        inv_fact[i] = inv_fact[i + 1] * (i + 1) as u64 % MOD;
    }

    let mod_binom = |n: usize, k: usize| -> u64 {
        if k > n { return 0; }
        fact_mod[n] % MOD * inv_fact[k] % MOD * inv_fact[n - k] % MOD
    };

    let get_xy = |k: usize, n: usize| -> (f64, f64) {
        let angle = 2.0 * std::f64::consts::PI * k as f64 / n as f64;
        (angle.cos(), angle.sin())
    };

    let get_intersection = |x1: f64, y1: f64, x2: f64, y2: f64,
                            x3: f64, y3: f64, x4: f64, y4: f64| -> Option<(f64, f64)> {
        let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if denom.abs() < 1e-15 { return None; }
        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
        Some((x1 + t * (x2 - x1), y1 + t * (y2 - y1)))
    };

    let compute_t = |n: usize| -> u64 {
        if n < 4 { return 0; }

        if n % 2 == 1 {
            // All intersections have multiplicity 2
            // T(n) = C(n,4) * 2 * (n-3)!
            let bn4 = mod_binom(n, 4);
            return bn4 * 2 % MOD * fact_mod[n - 3] % MOD;
        }

        // Even n: find multiplicities via hash map
        use std::collections::HashMap;
        let mut point_counts: HashMap<(i64, i64), i32> = HashMap::new();

        for a in 0..n {
            let (ax, ay) = get_xy(a, n);
            for b in (a + 1)..n {
                let (bx, by) = get_xy(b, n);
                for c in (b + 1)..n {
                    let (cx, cy) = get_xy(c, n);
                    for d in (c + 1)..n {
                        let (dx, dy) = get_xy(d, n);
                        if let Some((px, py)) = get_intersection(ax, ay, cx, cy, bx, by, dx, dy) {
                            let kx = (px * 1e9).round() as i64;
                            let ky = (py * 1e9).round() as i64;
                            *point_counts.entry((kx, ky)).or_insert(0) += 1;
                        }
                    }
                }
            }
        }

        let mut total_val: u64 = 0;
        for (_, &pairs) in &point_counts {
            let delta = 1 + 8 * pairs;
            let sq = (delta as f64).sqrt().round() as i32;
            let m = ((1 + sq) / 2) as usize;

            // Contribution for one point of multiplicity m:
            // c = sum_{k=2}^m (-1)^k * (k-1) * C(m,k) * 2^(k-1) * (n-k-1)!
            let mut c: u64 = 0;
            for k in 2..=m {
                let bin_mk = mod_binom(m, k);
                let pow2 = pow_mod(2, (k - 1) as u64, MOD);
                let fact_rem = fact_mod[n - k - 1];
                let term = (k as u64 - 1) % MOD * bin_mk % MOD * pow2 % MOD * fact_rem % MOD;
                if k % 2 == 0 {
                    c = (c + term) % MOD;
                } else {
                    c = (c + MOD - term) % MOD;
                }
            }

            total_val = (total_val + c) % MOD;
        }

        total_val
    };

    let mut total_sum: u64 = 0;
    for n in 3..=60 {
        let tn = compute_t(n);
        total_sum = (total_sum + tn) % MOD;
    }

    println!("{}", total_sum);
}
