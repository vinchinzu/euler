use rayon::prelude::*;
use std::f64::consts::PI;

fn main() {
    let n: i32 = 1803;

    // Precompute inverses for c in 1..=n (only ~14 KB, fits in L1 cache)
    let mut inv_c = vec![0.0f64; (n + 1) as usize];
    for i in 1..=n as usize {
        inv_c[i] = 1.0 / i as f64;
    }

    // Parallelize over 'a' values (1..=n/2)
    let total_area: f64 = (1..=n / 2)
        .into_par_iter()
        .map(|a| {
            let mut local_area = 0.0f64;
            let af = a as f64;
            // SAFETY: a <= n/2 < n + 1
            let inv_af = unsafe { *inv_c.get_unchecked(a as usize) };

            let b_max = n - a;
            for b in a..=b_max {
                let bf = b as f64;
                // SAFETY: b <= n - a < n + 1
                let inv_bf = unsafe { *inv_c.get_unchecked(b as usize) };

                let ab = af + bf;
                let amb = af - bf;
                let bma = bf - af;

                let c_max = a + b - 1;
                for c in b..=c_max {
                    let cf = c as f64;
                    // SAFETY: c <= a + b - 1 <= n < n + 1
                    let inv_cf = unsafe { *inv_c.get_unchecked(c as usize) };

                    let s = (ab + cf) * 0.5;
                    let sa = (bma + cf) * 0.5;
                    let sb = (amb + cf) * 0.5;
                    let sc = (ab - cf) * 0.5;

                    let r2 = (sa * sb * sc) / s;

                    // Algebraic half-angle formulas:
                    // sin(A/2) = sqrt((s - b)(s - c) / (bc))
                    // sin(B/2) = sqrt((s - a)(s - c) / (ac))
                    let inv_bcf = inv_bf * inv_cf;
                    let inv_acf = inv_af * inv_cf;

                    let sha = (sb * sc * inv_bcf).sqrt();
                    let shb = (sa * sc * inv_acf).sqrt();

                    let ratio_a = (1.0 - sha) / (1.0 + sha);
                    let ratio_b = (1.0 - shb) / (1.0 + shb);

                    let coc_ratio = ratio_a * ratio_a;
                    let ratio_3 = ratio_b.max(coc_ratio);

                    local_area += r2 * (1.0 + ratio_a * ratio_a + ratio_3 * ratio_3);
                }
            }
            local_area
        })
        .sum();

    let count: u64 = (1..=n / 2)
        .map(|a| a as u64 * (n - 2 * a + 1) as u64)
        .sum();

    println!("{:.5}", PI * total_area / count as f64);
}

