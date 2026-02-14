// Project Euler 554 - Centaurs on a Chessboard
//
// C(n) = 8*C(2n,n) - 3*(n-1)^2 - 8n - 4
// Find sum_{i=2}^{90} C(F_i) mod (10^8+7) using Lucas' theorem.

const M: i64 = 100_000_007; // 10^8 + 7, prime

fn power(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result: i64 = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn main() {
    // Precompute factorials mod M
    let mut fact = vec![0i64; M as usize];
    let mut inv_fact = vec![0i64; M as usize];
    fact[0] = 1;
    for i in 1..M as usize {
        fact[i] = fact[i - 1] * i as i64 % M;
    }
    inv_fact[M as usize - 1] = power(fact[M as usize - 1], M - 2, M);
    for i in (0..M as usize - 1).rev() {
        inv_fact[i] = inv_fact[i + 1] * (i as i64 + 1) % M;
    }

    let ncr_small = |a: i64, b: i64| -> i64 {
        if b < 0 || b > a || a < 0 { return 0; }
        if a >= M { return 0; }
        fact[a as usize] % M * inv_fact[b as usize] % M * inv_fact[(a - b) as usize] % M
    };

    let ncr_lucas = |mut n: u64, mut r: u64| -> i64 {
        if r > n { return 0; }
        let mut result: i64 = 1;
        while n > 0 || r > 0 {
            let ni = (n % M as u64) as i64;
            let ri = (r % M as u64) as i64;
            if ri > ni { return 0; }
            result = result * ncr_small(ni, ri) % M;
            n /= M as u64;
            r /= M as u64;
        }
        result
    };

    // Fibonacci numbers (unsigned since F_90 fits in u64)
    let mut fibs = [0u64; 91];
    fibs[0] = 0;
    fibs[1] = 1;
    for i in 2..=90 {
        fibs[i] = fibs[i - 1] + fibs[i - 2];
    }

    let mut ans: i64 = 0;
    for i in 2..=90 {
        let n = fibs[i];
        let n_mod = (n % M as u64) as i64;
        let c2n_n = ncr_lucas(2 * n, n);

        let sq = (n_mod - 1 + M) % M;
        let sq = sq * sq % M;
        let mut val = 8 * c2n_n % M;
        val = (val - 3 * sq % M + M) % M;
        val = (val - 8 * n_mod % M + M) % M;
        val = (val - 4 + M) % M;

        ans = (ans + val) % M;
    }

    println!("{ans}");
}
