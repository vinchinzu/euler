// Project Euler 492: Exploding sequence
//
// a_1 = 1, a_{n+1} = 6a_n^2 + 10a_n + 3
// Find sum of a_N (mod p) for all primes X <= p <= X+Y.
// Uses 2x2 matrix exponentiation + segmented sieve.

use euler_utils::mod_pow;

type U = u64;

#[derive(Clone, Copy)]
struct Mat2([U; 4]);

fn mat_mul(a: Mat2, b: Mat2, p: U) -> Mat2 {
    Mat2([
        ((a.0[0] as u128 * b.0[0] as u128 + a.0[1] as u128 * b.0[2] as u128) % p as u128) as U,
        ((a.0[0] as u128 * b.0[1] as u128 + a.0[1] as u128 * b.0[3] as u128) % p as u128) as U,
        ((a.0[2] as u128 * b.0[0] as u128 + a.0[3] as u128 * b.0[2] as u128) % p as u128) as U,
        ((a.0[2] as u128 * b.0[1] as u128 + a.0[3] as u128 * b.0[3] as u128) % p as u128) as U,
    ])
}

fn mat_pow(m: Mat2, mut exp: U, p: U) -> Mat2 {
    let mut result = Mat2([1, 0, 0, 1]);
    let mut base = Mat2([
        m.0[0] % p,
        ((m.0[1] % p) + p) % p,
        ((m.0[2] % p) + p) % p,
        m.0[3] % p,
    ]);
    while exp > 0 {
        if exp & 1 == 1 {
            result = mat_mul(result, base, p);
        }
        base = mat_mul(base, base, p);
        exp >>= 1;
    }
    result
}

fn main() {
    let n_val: U = 1_000_000_000_000_000; // 10^15
    let x: U = 1_000_000_000; // 10^9
    let y: U = 10_000_000; // 10^7

    // Sieve small primes
    let sqrt_limit = ((x + y) as f64).sqrt() as usize + 1;
    let mut small_sieve = vec![true; sqrt_limit + 1];
    small_sieve[0] = false;
    small_sieve[1] = false;
    {
        let mut i = 2;
        while i * i <= sqrt_limit {
            if small_sieve[i] {
                let mut j = i * i;
                while j <= sqrt_limit {
                    small_sieve[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
    }
    let small_primes: Vec<U> = (2..=sqrt_limit).filter(|&i| small_sieve[i]).map(|i| i as U).collect();

    // Segmented sieve for [X, X+Y]
    let mut is_prime = vec![true; y as usize + 1];
    for &p in &small_primes {
        let start = if p * p > x {
            (p * p - x) as usize
        } else {
            let rem = x % p;
            if rem == 0 { 0 } else { (p - rem) as usize }
        };
        let mut j = start;
        while j <= y as usize {
            is_prime[j] = false;
            j += p as usize;
        }
    }

    let mut ans: U = 0;

    for i in 0..=y as usize {
        if !is_prime[i] { continue; }
        let p = x + i as U;

        // A = [[0, 1], [-1, 11]]
        let a = Mat2([0, 1, p - 1, 11 % p]);

        // Check if period is p-1 or p+1
        let test = mat_pow(a, p - 1, p);
        let period = if test.0[0] == 1 && test.0[1] == 0 && test.0[2] == 0 && test.0[3] == 1 {
            p - 1
        } else {
            p + 1
        };

        let exp_val = mod_pow(2, n_val - 1, period);
        let mat = mat_pow(a, exp_val, p);

        // x_n = 2 * mat[0] + 11 * mat[1]
        let x_n = ((2u128 * mat.0[0] as u128 + 11u128 * mat.0[1] as u128) % p as u128) as U;
        // a_n = (x_n - 5) / 6 mod p
        let a_n = ((x_n + p - 5 % p) % p * mod_pow(6, p - 2, p)) % p;
        ans += a_n;
    }

    println!("{}", ans);
}
