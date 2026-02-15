// Project Euler 769 - Binary Quadratic Form
// Count representations of z^2 as x^2+5xy+3y^2 with z <= N using Mobius function.

fn main() {
    const N: i64 = 100_000_000_000_000; // 10^14
    let sqrt_n = (N as f64).sqrt() as usize;

    // Compute Mobius
    let mut mobius = vec![1i32; sqrt_n + 1];
    let mut is_prime = vec![true; sqrt_n + 1];
    is_prime[0] = false;
    if sqrt_n >= 1 { is_prime[1] = false; }

    for i in 2..=sqrt_n {
        if is_prime[i] {
            for j in (i..=sqrt_n).step_by(i) {
                if j > i { is_prime[j] = false; }
                if (j / i) % i == 0 {
                    mobius[j] = 0;
                } else {
                    mobius[j] = -mobius[j];
                }
            }
        }
    }

    let sqrt3: f64 = 3.0_f64.sqrt();
    let mut ans: i64 = 0;

    for g in 1..=sqrt_n {
        let g_sq = g as i64 * g as i64;
        for &h in &[1i64, 13] {
            let mut n_idx: i64 = 1;
            while g_sq * n_idx * n_idx <= h * N {
                let term1 = n_idx as f64 / sqrt3;
                let inner = 13.0 * (n_idx as f64) * (n_idx as f64)
                    + 12.0 * (h as f64 * N as f64) / g_sq as f64;
                let term2 = (inner.sqrt() - 5.0 * n_idx as f64) / 6.0;
                let max_m = term1.min(term2) as i64;

                if (g % 13 == 0) == (h == 13) {
                    ans += mobius[g] as i64 * max_m;
                }

                if g % 13 != 0 {
                    let sign: i64 = if h == 1 { -1 } else { 1 };
                    ans += sign * mobius[g] as i64 * (max_m + (3 * n_idx) % 13) / 13;
                }

                n_idx += 1;
            }
        }
    }

    println!("{}", ans);
}
