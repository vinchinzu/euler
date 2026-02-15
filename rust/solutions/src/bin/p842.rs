// Project Euler 842 - n-Star Polygons

const MOD: i64 = 1000000007;

fn main() {
    // Exact factorials for small n using i128
    let mut fact = [0i128; 62];
    fact[0] = 1;
    for i in 1..=61 { fact[i] = fact[i - 1] * i as i128; }

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

    let compute_t = |n: usize| -> i64 {
        if n < 4 { return 0; }

        if n % 2 == 1 {
            let bn4 = fact[n] / (fact[4] * fact[n - 4]);
            let result = bn4 * 2 * fact[n - 3];
            return (result % MOD as i128) as i64;
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

        let mut total_val: i64 = 0;
        for (_, &pairs) in &point_counts {
            let delta = 1 + 8 * pairs;
            let sq = (delta as f64).sqrt().round() as i32;
            let m = (1 + sq) / 2;

            let mut c: i128 = 0;
            for k in 2..=m as usize {
                let term_sign: i128 = if k % 2 == 0 { 1 } else { -1 };
                let bin_mk = fact[m as usize] / (fact[k] * fact[m as usize - k]);
                let pow2: i128 = 1i128 << (k - 1);
                let fact_rem = fact[n - k - 1];
                c += term_sign * (k as i128 - 1) * bin_mk * pow2 * fact_rem;
            }

            let term_total = ((c % MOD as i128) + MOD as i128) % MOD as i128;
            total_val = (total_val + term_total as i64) % MOD;
        }

        total_val
    };

    let mut total_sum: i64 = 0;
    for n in 3..=60 {
        let tn = compute_t(n);
        total_sum = (total_sum + tn) % MOD;
    }

    println!("{}", total_sum);
}
