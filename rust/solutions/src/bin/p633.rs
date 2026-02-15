// Project Euler 633 - Square prime factors II
// DP over primes for limit of C_K(N)/N, K=7

const K_VAL: usize = 7;

fn main() {
    let mut ans = 0.0f64;
    let mut num_primes_limit = K_VAL;

    loop {
        let mut c = [0.0f64; K_VAL + 2];
        c[0] = 1.0;

        // Sieve primes
        let mut is_prime = vec![true; num_primes_limit + 1];
        is_prime[0] = false;
        if num_primes_limit >= 1 { is_prime[1] = false; }
        let mut i = 2;
        while i * i <= num_primes_limit { if is_prime[i] { let mut j = i*i; while j <= num_primes_limit { is_prime[j] = false; j += i; } } i += 1; }

        for p in 2..=num_primes_limit {
            if !is_prime[p] { continue; }
            let p_sq = p as f64 * p as f64;
            for k in (0..=K_VAL).rev() {
                c[k + 1] += c[k] / p_sq;
                c[k] *= 1.0 - 1.0 / p_sq;
            }
        }

        if c[K_VAL] != 0.0 && (ans / c[K_VAL] - 1.0).abs() < 1e-5 {
            ans = c[K_VAL];
            break;
        }
        ans = c[K_VAL];
        num_primes_limit *= 2;
    }

    println!("{:.4e}", ans);
}
