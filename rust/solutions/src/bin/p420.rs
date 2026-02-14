// Project Euler 420 - 2x2 positive integer matrix M with trace < N
// where M = M^(-1) (involutory matrices)
// Sieve divisor counts, then iterate over (T1, T2, r) triples.

fn main() {
    let n: i64 = 10_000_000;
    let limit = 2 * n;

    // Sieve divisor counts up to limit
    let mut num_divs = vec![0u32; (limit + 1) as usize];
    for i in 1..=limit {
        let mut j = i;
        while j <= limit {
            num_divs[j as usize] += 1;
            j += i;
        }
    }

    let mut ans: i64 = 0;

    // isqrt(N)
    let mut sq_n: i64 = 1;
    while (sq_n + 1) * (sq_n + 1) <= n { sq_n += 1; }

    for t1 in 1..=sq_n {
        let t2_max_sq = 2 * n - 1 - t1 * t1;
        if t2_max_sq < 0 { continue; }
        let mut t2_max: i64 = 1;
        while (t2_max + 1) * (t2_max + 1) <= t2_max_sq { t2_max += 1; }

        let mut t2 = t1 + 2;
        while t2 <= t2_max {
            let g = gcd(t1, t2);
            let r_start = g % 2;
            let r_max = (t1 - 2) * g / t2;

            let mut r = r_start;
            while r <= r_max {
                let val = (g * g - r * r) / 4;
                if val > 0 && val <= limit {
                    if r == 0 {
                        ans += num_divs[val as usize] as i64;
                    } else {
                        ans += 2 * num_divs[val as usize] as i64;
                    }
                }
                r += 2;
            }
            t2 += 2;
        }
    }

    println!("{}", ans);
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
