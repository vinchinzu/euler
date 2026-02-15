// Project Euler 608 - Divisor Sums
// Multiplicative function with Lucy DP + DFS over square-free divisors

const N: i64 = 1_000_000_000_000;
const K: usize = 200;
const M: i64 = 1_000_000_007;

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn sum_floor_quotients(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let sq = isqrt(n);
    let mut result = 0i64;
    for d in 1..=sq {
        result += n / d;
    }
    result = 2 * result - sq * sq;
    result
}

fn mod_inv(a: i64, m: i64) -> i64 {
    let (mut t, mut new_t) = (0i64, 1i64);
    let (mut r, mut new_r) = (m, a % m);
    while new_r != 0 {
        let q = r / new_r;
        let tmp = new_t; new_t = t - q * new_t; t = tmp;
        let tmp = new_r; new_r = r - q * new_r; r = tmp;
    }
    ((t % m) + m) % m
}

fn tr(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn main() {
    // Sieve primes up to K
    let mut is_prime = vec![true; K + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= K { if is_prime[i] { let mut j = i*i; while j <= K { is_prime[j] = false; j += i; } } i += 1; }
    let primes: Vec<usize> = (2..=K).filter(|&i| is_prime[i]).collect();

    let lim = (N as f64).powf(2.0 / 3.0) as usize;

    // Sieve divisor counts
    let mut num_divs = vec![0i32; lim + 1];
    for i in 1..=lim {
        let mut j = i;
        while j <= lim { num_divs[j] += 1; j += i; }
    }

    let mut sum_floor_small = vec![0i64; lim + 1];
    for i in 1..=lim {
        sum_floor_small[i] = (sum_floor_small[i - 1] + num_divs[i] as i64) % M;
    }
    drop(num_divs);

    // Compute product_updates
    let mut product_updates = vec![0i64; K + 1];
    let mut mult = 1i64;
    for &p in &primes {
        let mut e = 0i64;
        let mut pw = p;
        while pw <= K { e += (K / pw) as i64; pw *= p; }
        let tr_e1 = tr(e + 1) % M;
        mult = (mult as i128 * tr_e1 as i128 % M as i128) as i64;
        let tr_e = tr(e) % M;
        product_updates[p] = ((-(tr_e as i128 * mod_inv(tr_e1, M) as i128 % M as i128) % M as i128) + M as i128) as i64 % M;
    }

    // DFS
    let mut ans = 0i64;
    let lim_i64 = lim as i64;

    struct State {
        min_idx: usize,
        d: i64,
        mult: i64,
    }

    let mut stack = Vec::with_capacity(1 << 20);
    stack.push(State { min_idx: 0, d: 1, mult });

    while let Some(State { min_idx, d, mult }) = stack.pop() {
        let q = N / d;
        let sum_val = if q >= lim_i64 {
            sum_floor_quotients(q) % M
        } else {
            sum_floor_small[q as usize]
        };
        ans = ((ans as i128 + sum_val as i128 * mult as i128 % M as i128 + M as i128) % M as i128) as i64;

        for idx in min_idx..primes.len() {
            let p = primes[idx] as i64;
            if d as f64 * p as f64 > N as f64 { break; }
            let new_mult = (mult as i128 * product_updates[p as usize] as i128 % M as i128) as i64;
            let new_mult = (new_mult + M) % M;
            stack.push(State { min_idx: idx + 1, d: d * p, mult: new_mult });
        }
    }

    println!("{}", (ans % M + M) % M);
}
