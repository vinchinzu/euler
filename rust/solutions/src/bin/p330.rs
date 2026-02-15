// Project Euler 330: Euler's Number
//
// a(n) is defined for all integers n as:
//   a(n) = 1 for n < 0
//   a(n) = sum_{i>=1} a(n-i)/i! for n >= 0
//
// Each a(n) = (A(n)*e + B(n)) / n! where A(n), B(n) are integers.
// Find (A(10^9) + B(10^9)) mod 77777777.
//
// Approach: Let h(n) = A(n) + B(n). The EGF H(x) = (1-e^x)/((1-x)*(2-e^x)).
// Define d(n) = h(n) - n*h(n-1) with h(0)=0, d(0)=0.
// d satisfies: d(n) = sum_{k=1}^n C(n,k)*d(n-k) - 1 for n >= 1.
// Key observations (mod prime p):
//   - d(n) mod p has period p-1
//   - h(n) mod p has period p*(p-1)
// Use CRT over prime factors of 77777777 = 7*11*73*101*137.

fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, x1, y1) = ext_gcd(b, a % b);
    (g, y1, x1 - (a / b) * y1)
}

fn compute_h_mod_p(p: i64, target_n: i64) -> i64 {
    let period_d = (p - 1) as usize; // d has period p-1

    // Precompute binomial coefficients mod p up to size period_d
    // We only need C(n, k) for n < period_d and k <= n
    let max_n = period_d;
    let mut binom = vec![vec![0i64; max_n + 1]; max_n + 1];
    for i in 0..=max_n {
        binom[i][0] = 1;
        for j in 1..=i {
            binom[i][j] = (binom[i - 1][j - 1] + binom[i - 1][j]) % p;
        }
    }

    // Compute d(n) mod p for n = 0..period_d-1
    let mut d_vals = vec![0i64; period_d];
    // d(0) = 0 (already set)
    for n in 1..period_d {
        let mut s = 0i64;
        for k in 1..=n {
            s = (s + binom[n][k] * d_vals[n - k]) % p;
        }
        d_vals[n] = ((s - 1) % p + p) % p;
    }

    // h(n) = d(n) + n * h(n-1), h(0) = 0
    // h has period p*(p-1)
    let period_h = (p * (p - 1)) as usize;

    let mut h_prev = 0i64; // h(0)
    // We need h(target_n % period_h), but since target_n is huge,
    // compute h for one full period
    let target_idx = (target_n % period_h as i64) as usize;

    if target_idx == 0 {
        return 0;
    }

    for n in 1..=target_idx {
        let d_n = d_vals[n % period_d];
        let h_n = (d_n + (n as i64 % p) * h_prev % p) % p;
        h_prev = h_n;
    }

    h_prev
}

fn main() {
    let n: i64 = 1_000_000_000;
    let m: i64 = 77_777_777;

    let primes = [7i64, 11, 73, 101, 137];
    let mut residues = [0i64; 5];

    for (pi, &p) in primes.iter().enumerate() {
        residues[pi] = compute_h_mod_p(p, n);
    }

    // CRT: combine residues
    let mut result = 0i64;
    for i in 0..5 {
        let mi = m / primes[i];
        let (_, x, _) = ext_gcd(mi, primes[i]);
        let x = ((x % primes[i]) + primes[i]) % primes[i];
        result = (result
            + (residues[i] as i128 * mi as i128 % m as i128 * x as i128 % m as i128)
                as i64)
            % m;
    }

    println!("{}", result);
}
