// Project Euler 454 - Diophantine reciprocals III
// Count solutions to 1/x + 1/y = 1/n with x >= y > n, for n up to 10^12.

const N: i64 = 1_000_000_000_000;

fn main() {
    let l = (N as f64).sqrt() as i64 + 1;

    // Pre-compute Mobius function
    let mut mobius = vec![1i8; (l + 11) as usize];
    let mut is_prime_arr = vec![true; (l + 12) as usize];
    is_prime_arr[0] = false;
    if (l + 11) >= 1 { is_prime_arr[1] = false; }

    for i in 2..=(l + 10) as usize {
        if is_prime_arr[i] {
            let mut j = i + i;
            while j <= (l + 10) as usize {
                is_prime_arr[j] = false;
                j += i;
            }
            // Square-free check
            let sq = (i as i64) * (i as i64);
            let mut j = sq;
            while j <= l + 10 {
                mobius[j as usize] = 0;
                j += sq;
            }
            // Flip sign for prime factors
            let mut j = i;
            while j <= (l + 10) as usize {
                mobius[j] = -mobius[j];
                j += i;
            }
        }
    }

    let mut ans: i64 = 0;

    for g in 1..=l {
        if mobius[g as usize] == 0 { continue; }
        let n_val = N / (g * g);
        if n_val == 0 { break; }

        let mut y: i64 = 2;
        while y * y <= n_val {
            if y * y * y <= n_val {
                for x in 1..y {
                    ans += mobius[g as usize] as i64 * (n_val / y / (x + y));
                }
            } else {
                let start_q = {
                    let v = n_val / y / (2 * y - 1);
                    if v < 1 { 1 } else { v }
                };
                let mut q = start_q;
                loop {
                    let mut upper = n_val / y / q;
                    if upper > 2 * y - 1 { upper = 2 * y - 1; }
                    let mut lower = n_val / y / (q + 1);
                    if lower < y { lower = y; }
                    ans += mobius[g as usize] as i64 * (upper - lower) * q;
                    if lower == y { break; }
                    q += 1;
                }
            }
            y += 1;
        }
    }

    println!("{}", ans);
}
