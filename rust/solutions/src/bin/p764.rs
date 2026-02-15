// Project Euler 764 - Sum of Solutions to 16x^2+y^4=z^2
// Uses Mobius function with three cases of factorization.

fn main() {
    const MOD: i64 = 1_000_000_000;
    const NN: i128 = 10_000_000_000_000_000; // 10^16

    let l = ((2.0 * NN as f64).powf(0.25) as usize) + 10;

    // Compute Mobius
    let mut mobius = vec![1i32; l + 1];
    let mut is_prime = vec![true; l + 1];
    is_prime[0] = false;
    if l >= 1 { is_prime[1] = false; }
    for i in 2..=l {
        if is_prime[i] {
            for j in (i..=l).step_by(i) {
                if j > i { is_prime[j] = false; }
                if (j / i) % i == 0 {
                    mobius[j] = 0;
                } else {
                    mobius[j] = -mobius[j];
                }
            }
        }
    }

    // Precomputed sums of fourth powers using i128
    let mut sum_fp = vec![0i128; l + 1];
    let mut sum_ofp = vec![0i128; l + 1];
    for i in 1..=l {
        let iv = i as i128;
        sum_fp[i] = sum_fp[i - 1] + iv * iv * iv * iv;
        let odd = 2 * iv - 1;
        sum_ofp[i] = sum_ofp[i - 1] + odd * odd * odd * odd;
    }

    let sqrt2: f64 = 2.0_f64.sqrt();
    let mut s: i128 = 0;

    for g in 1..l {
        if mobius[g] == 0 { continue; }

        let g4: i128 = (g as i128) * (g as i128) * (g as i128) * (g as i128);
        let n = NN / g4;
        if n == 0 { break; }

        let mut deg4_num8: i128 = 0;
        let mut deg2: i128 = 0;

        // Case 1: only when g is odd
        if g % 2 == 1 {
            let mut b: i64 = 1;
            while (b as i128) * (b as i128) * (b as i128) * (b as i128) <= 2 * n {
                let b4 = (b as i128) * (b as i128) * (b as i128) * (b as i128);
                let limit1 = b / 2;
                let val = (2.0 * n as f64 - b4 as f64);
                let limit2 = if val > 0.0 {
                    ((val.powf(0.25)) as i64 + 1) / 2
                } else {
                    0
                };
                let num_a = limit1.min(limit2);

                if num_a > 0 {
                    deg4_num8 += 5 * num_a as i128 * b4 + 3 * sum_ofp[num_a as usize];
                    deg2 += (num_a as i128) * (num_a as i128) * b as i128;
                }
                b += 2;
            }
        }

        // Case 2
        {
            let mut b: i64 = 1;
            while 4 * (b as i128) * (b as i128) * (b as i128) * (b as i128) <= n {
                let b4 = (b as i128) * (b as i128) * (b as i128) * (b as i128);
                let limit1 = (sqrt2 * b as f64) as i64;
                let remaining = n - 4 * b4;
                let limit2 = if remaining > 0 { (remaining as f64).powf(0.25) as i64 } else { 0 };
                let mut num_a = limit1.min(limit2);

                if g % 2 == 1 {
                    num_a /= 2;
                }

                let mult: i128 = if g % 2 == 0 { 1 } else { 2 };

                if num_a > 0 {
                    let m4 = mult * mult * mult * mult;
                    deg4_num8 += 40 * num_a as i128 * b4 + 6 * m4 * sum_fp[num_a as usize];
                    let tr_num_a = num_a as i128 * (num_a as i128 + 1) / 2;
                    deg2 += 2 * mult * tr_num_a * b as i128;
                }
                b += 1;
            }
        }

        // Case 3
        {
            let mult: i64 = if g % 2 == 0 { 1 } else { 2 };
            let mut b = mult;
            while (b as i128) * (b as i128) * (b as i128) * (b as i128) <= n {
                let b4 = (b as i128) * (b as i128) * (b as i128) * (b as i128);
                let limit1 = (b as f64 / sqrt2) as i64;
                let remaining = (n as f64 - b4 as f64) / 4.0;
                let limit2 = if remaining > 0.0 { remaining.powf(0.25) as i64 } else { 0 };
                let num_a = limit1.min(limit2);

                if num_a > 0 {
                    deg4_num8 += 10 * num_a as i128 * b4 + 24 * sum_fp[num_a as usize];
                    let tr_num_a = num_a as i128 * (num_a as i128 + 1) / 2;
                    deg2 += 2 * tr_num_a * b as i128;
                }
                b += mult;
            }
        }

        let g2 = (g as i128) * (g as i128);
        s += mobius[g] as i128 * (deg4_num8 * g4 / 8 + deg2 * g2);
    }

    let mut result = (s % MOD as i128) as i64;
    if result < 0 { result += MOD; }
    println!("{}", result);
}
