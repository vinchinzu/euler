// Project Euler 496 - Incenter and circumcircle
// Uses Mobius function + number theory.

fn isqrt(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut x = (n as f64).sqrt() as i64;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

fn tr(n: i64) -> i128 {
    (n as i128) * (n as i128 + 1) / 2
}

fn main() {
    let big_n: i64 = 1_000_000_000;
    let sq = isqrt(big_n) as usize;

    // Compute Mobius function up to sq
    let mut mu = vec![1i32; sq + 1];
    let mut is_prime = vec![true; sq + 1];
    is_prime[0] = false;
    if sq >= 1 {
        is_prime[1] = false;
    }

    for i in 2..=sq {
        if is_prime[i] {
            for j in (i..=sq).step_by(i) {
                if j != i {
                    is_prime[j] = false;
                }
                if (j as u64) % ((i as u64) * (i as u64)) == 0 {
                    mu[j] = 0;
                } else {
                    mu[j] = -mu[j];
                }
            }
        }
    }

    let mut ans: i128 = 0;

    for g in 1..=sq {
        if mu[g] == 0 {
            continue;
        }

        let n = big_n / ((g as i64) * (g as i64));
        let l = ((n as f64 / 2.0).cbrt()) as i64 + 1;
        let mut res: i128 = 0;

        for x in 1..l {
            let mut max_y = 2 * x - 1;
            if max_y > n / x {
                max_y = n / x;
            }
            for y in (x + 1)..=max_y {
                if x * y <= n {
                    res += tr(n / x / y) * (x as i128) * (y as i128);
                }
            }
        }

        let sq_n = isqrt(n);
        for x in l..=sq_n {
            let mut max_y = 2 * x - 1;
            if max_y > n / x {
                max_y = n / x;
            }
            if max_y <= x {
                continue;
            }

            let mut q = n / x / max_y;
            loop {
                if (x as i128) * (x as i128) * (q as i128) > n as i128 {
                    break;
                }
                let mut upper = n / x / q;
                if upper > max_y {
                    upper = max_y;
                }
                let lower_candidate = if q + 1 > 0 { n / x / (q + 1) } else { 0 };
                let lower = if lower_candidate > x {
                    lower_candidate
                } else {
                    x
                };
                if upper > lower {
                    res += tr(q) * (x as i128) * (tr(upper) - tr(lower));
                }
                q += 1;
            }
        }

        ans += (mu[g] as i128) * (g as i128) * (g as i128) * res;
    }

    println!("{}", ans);
}
