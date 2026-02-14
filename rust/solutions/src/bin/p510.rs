// Project Euler 510 - Tangent Circles
// Sum r_A + r_B + r_C for all valid circle configurations with r_A <= r_B <= N.

fn isqrt_ll(n: i64) -> i64 {
    let mut r = (n as f64).sqrt() as i64;
    while r * r > n { r -= 1; }
    while (r + 1) * (r + 1) <= n { r += 1; }
    r
}

fn main() {
    let n: i64 = 1_000_000_000;
    let l = isqrt_ll(n) as usize;

    // Sieve primes up to l
    let sieve = euler_utils::sieve(l);
    let primes: Vec<usize> = (2..=l).filter(|&i| sieve[i]).collect();

    // Mobius function up to l
    let mut mu = vec![1i32; l + 1];
    let mut is_square_free = vec![true; l + 1];
    for &p in &primes {
        if p > l { break; }
        let mut j = p;
        while j <= l {
            mu[j] *= -1;
            if j % (p * p) == 0 {
                is_square_free[j] = false;
            }
            j += p;
        }
    }
    for i in 0..=l {
        if !is_square_free[i] { mu[i] = 0; }
    }

    let mut ans: i64 = 0;

    let mut g: i64 = 1;
    while g * g <= n {
        if mu[g as usize] == 0 { g += 1; continue; }
        let nn = n / (g * g);

        let mut b: i64 = 1;
        while b * b <= nn {
            let b2 = b * b;

            // Find all divisors of b^2
            let mut divs = Vec::new();
            {
                let mut bb = b as usize;
                let mut pf = Vec::new();
                for &p in &primes {
                    if p * p > bb { break; }
                    if bb % p == 0 {
                        let mut e = 0;
                        while bb % p == 0 { e += 1; bb /= p; }
                        pf.push((p as i64, e));
                    }
                }
                if bb > 1 { pf.push((bb as i64, 1)); }

                divs.push(1i64);
                for &(pv, pe) in &pf {
                    let old_count = divs.len();
                    let mut pp = 1i64;
                    for _ in 0..(2 * pe) {
                        pp *= pv;
                        for j in 0..old_count {
                            divs.push(divs[j] * pp);
                        }
                    }
                }
            }

            for &d in &divs {
                let a = d - b;
                if a > 0 && a <= b {
                    let c = b - b2 / d;
                    let nb2 = nn / b2;
                    let tr_nb2 = nb2 * (nb2 + 1) / 2;
                    ans += mu[g as usize] as i64 * (g * g)
                           * (a * a + b * b + c * c) * tr_nb2;
                }
            }

            b += 1;
        }
        g += 1;
    }

    println!("{}", ans);
}
