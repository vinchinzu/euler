// Project Euler 881
// Find smallest n with g(n) > 10000 using primes up to 100 and DP.

const N_TARGET: i64 = 10000;

fn main() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47,
                  53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    let num_primes = primes.len();

    let mut best: i64 = 1_000_000_000_000_000_000;
    let mut es = vec![0i32; 30];
    let mut es_len = 0usize;

    fn compute_g(es: &[i32], es_len: usize) -> i64 {
        let mut sum_e = 0;
        for i in 0..es_len { sum_e += es[i] as usize; }
        let half = sum_e / 2;
        let mut dp = vec![0i64; half + 1];
        dp[0] = 1;
        for i in 0..es_len {
            for j in (1..=half).rev() {
                for k in 1..=es[i] as usize {
                    if k <= j {
                        dp[j] += dp[j - k];
                    }
                }
            }
        }
        dp[half]
    }

    fn helper(
        index: usize, n: i64, sum_e: i32,
        primes: &[i32], num_primes: usize,
        es: &mut Vec<i32>, es_len: &mut usize,
        best: &mut i64,
    ) {
        let g = {
            let mut se = 0;
            for i in 0..*es_len { se += es[i] as usize; }
            let half = se / 2;
            let mut dp = vec![0i64; half + 1];
            dp[0] = 1;
            for i in 0..*es_len {
                for j in (1..=half).rev() {
                    for k in 1..=es[i] as usize {
                        if k <= j { dp[j] += dp[j - k]; }
                    }
                }
            }
            dp[half]
        };

        if g > N_TARGET && n < *best {
            *best = n;
        }

        if index >= num_primes { return; }

        let p = primes[index] as i64;
        for e in 1.. {
            if *es_len > 0 && e > es[*es_len - 1] { break; }

            let mut power = 1i64;
            let mut overflow = false;
            for _ in 0..e {
                if power > 1_000_000_000_000_000_000 / p { overflow = true; break; }
                power *= p;
            }
            if overflow { break; }
            if n > 1_000_000_000_000_000_000 / power { break; }
            let new_n = n * power;
            if new_n >= *best { break; }

            es[*es_len] = e;
            *es_len += 1;
            helper(index + 1, new_n, sum_e + e, primes, num_primes, es, es_len, best);
            *es_len -= 1;
        }
    }

    helper(0, 1, 0, &primes, num_primes, &mut es, &mut es_len, &mut best);
    println!("{}", best);
}
