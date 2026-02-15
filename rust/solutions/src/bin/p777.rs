// Project Euler 777 - Lissajous Curves
// Sum of x^2+y^2 at self-crossings using Mobius inversion.

fn main() {
    const N: usize = 1_000_000;

    // Compute Mobius
    let mut mobius = vec![1i32; N + 1];
    let mut is_prime = vec![true; N + 1];
    is_prime[0] = false;
    if N >= 1 { is_prime[1] = false; }
    for i in 2..=N {
        if is_prime[i] {
            for j in (i..=N).step_by(i) {
                if j > i { is_prime[j] = false; }
                if (j / i) % i == 0 { mobius[j] = 0; } else { mobius[j] = -mobius[j]; }
            }
        }
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 { let t = b; b = a % b; a = t; }
        a
    }

    fn num_divisors(n: i32) -> i32 {
        let mut count = 0;
        let mut i = 1;
        while (i as i64) * (i as i64) <= n as i64 {
            if n % i == 0 {
                count += 2;
                if i * i == n { count -= 1; }
            }
            i += 1;
        }
        count
    }

    fn get_divisors(n: i32) -> Vec<i32> {
        let mut divs = Vec::new();
        let mut i = 1;
        while (i as i64) * (i as i64) <= n as i64 {
            if n % i == 0 {
                divs.push(i);
                if i != n / i { divs.push(n / i); }
            }
            i += 1;
        }
        divs
    }

    let mut nd = [0i32; 11];
    for d in 1..=10 { nd[d] = num_divisors(d as i32); }

    let mut ans_times_4: i128 = 0;

    for g in 1..=N {
        if mobius[g] == 0 { continue; }
        let n = (N / g) as i128;
        if n < 1 { continue; }

        let t = gcd(10, g as i32);
        let trn: i128 = n * (n + 1) / 2;
        let g2: i128 = (g as i128) * (g as i128);

        let mut res4: i128 = 8 * g2 * trn * trn - 12 * n * g as i128 * trn;

        let val_10_t = 10 / t;
        let divs = get_divisors(val_10_t);
        let nd_t = nd[t as usize] as i128;

        for &d in &divs {
            let e = val_10_t / d;
            let nd_val = n / d as i128;
            let ne_val = n / e as i128;
            let tnd: i128 = nd_val * (nd_val + 1) / 2;
            let tne: i128 = ne_val * (ne_val + 1) / 2;

            res4 += nd_t * (
                -6 * g2 * d as i128 * tnd * e as i128 * tne
                + 3 * nd_val * g as i128 * e as i128 * tne
                + 3 * ne_val * g as i128 * d as i128 * tnd
                + 4 * nd_val * ne_val
            );
        }

        ans_times_4 += mobius[g] as i128 * res4;
    }

    let ans = ans_times_4 as f64 / 4.0;

    let formatted = format!("{:.9e}", ans);
    let cleaned: String = formatted.chars().filter(|&c| c != '+').collect();
    println!("{}", cleaned);
}
