// Project Euler 574 - Verifying Primes
//
// V(p) = smallest A in triplet (A,B,q) such that A>=B>0, gcd(A,B)=1,
// AB divisible by every prime < q, p < q^2, and p=A+B or p=A-B.
// Sum V(p) for all primes p < 3800.

use euler_utils::primes_up_to;

fn gcd_ll(mut a: i64, mut b: i64) -> i64 {
    if a < 0 { a = -a; }
    if b < 0 { b = -b; }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn ext_gcd_ll(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, x1, y1) = ext_gcd_ll(b, a % b);
    (g, y1, x1 - (a / b) * y1)
}

fn v(p: i32, all_primes: &[usize]) -> i64 {
    let sq = (p as f64).sqrt() as i32;
    let q_primes: Vec<i32> = all_primes.iter()
        .filter(|&&x| x as i32 <= sq)
        .map(|&x| x as i32)
        .collect();
    let nq = q_primes.len();
    if nq == 0 {
        return 0;
    }

    // Product of all primes up to sqrt(p)
    let mut product: i64 = 1;
    for &q in &q_primes {
        product *= q as i64;
    }

    // Try A < p case: p = A + B, A >= B > 0, so A >= (p+1)/2
    for a in ((p as i64 + 1) / 2)..(p as i64) {
        let b = p as i64 - a;
        if gcd_ll(a, b) != 1 {
            continue;
        }
        if (a * b) % product == 0 {
            return a;
        }
    }

    // Try A > p case: p = A - B
    let num_subsets = 1usize << nq;
    let mut best: i64 = -1;

    for subset in 0..num_subsets {
        let mut c0: i64 = 1;
        let mut c1: i64 = 1;
        for i in 0..nq {
            if (subset >> i) & 1 != 0 {
                c0 *= q_primes[i] as i64;
            } else {
                c1 *= q_primes[i] as i64;
            }
        }

        let (g, x0, _) = ext_gcd_ll(c0, c1);
        if p as i64 % g != 0 {
            continue;
        }
        let mut x_sol = ((x0 % c1) * ((p as i64 / g) % c1)) % c1;
        if x_sol < 0 {
            x_sol += c1;
        }

        let base_a = c0 * x_sol;
        let step = c0 * c1; // = product
        if step == 0 {
            continue;
        }

        let k_start = if base_a <= p as i64 {
            (p as i64 - base_a) / step + 1
        } else {
            0i64
        };

        for k in k_start..(k_start + 2000) {
            let a = base_a + k * step;
            if a <= p as i64 {
                continue;
            }
            let b = a - p as i64;
            if gcd_ll(a, b) != 1 {
                continue;
            }
            if (a * b) % product == 0 {
                if best == -1 || a < best {
                    best = a;
                }
                break;
            }
        }
    }

    if best > 0 { best } else { 0 }
}

fn main() {
    let all_primes = primes_up_to(3800);

    let mut ans: i64 = 0;
    for &p in &all_primes {
        if p < 3800 {
            ans += v(p as i32, &all_primes);
        }
    }

    println!("{}", ans);
}
