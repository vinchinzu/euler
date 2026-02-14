// Project Euler 330: Euler's Number
// (A(n) + B(n)) mod 77777777 for n = 10^9.
// Uses CRT decomposition over prime factors of 77777777 = 7 * 11 * 73 * 101 * 137.

fn mod_pow_ll(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result = 1i64;
    base = ((base % modulus) + modulus) % modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % modulus as i128) as i64;
        }
        base = (base as i128 * base as i128 % modulus as i128) as i64;
        exp >>= 1;
    }
    result
}

fn mod_inv_prime(a: i64, p: i64) -> i64 {
    mod_pow_ll(a, p - 2, p)
}

fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 { return (a, 1, 0); }
    let (g, x1, y1) = ext_gcd(b, a % b);
    (g, y1, x1 - (a / b) * y1)
}

fn main() {
    let n: i64 = 1_000_000_000;
    let m: i64 = 77_777_777;

    let primes = [7i64, 11, 73, 101, 137];
    let mut residues = [0i64; 5];

    for (pi, &p) in primes.iter().enumerate() {
        // Compute inverse factorials mod p
        let mut inv_fact = vec![0i64; p as usize];
        inv_fact[0] = 1;
        let mut fact = 1i64;
        for j in 1..p as usize {
            fact = fact * j as i64 % p;
            inv_fact[j] = mod_inv_prime(fact, p);
        }

        // e_p = sum_{j=0}^{p-1} 1/j! mod p
        let mut e_p = 0i64;
        for j in 0..p as usize {
            e_p = (e_p + inv_fact[j]) % p;
        }

        // a(k) mod p for k = 0..p-2
        let mut a_vals = vec![0i64; p as usize];
        a_vals[0] = 1;
        for k in 1..p as usize - 1 {
            a_vals[k] = (a_vals[k - 1] + inv_fact[k]) % p;
        }

        // A(n) mod p
        let a_n = if n >= p {
            let mut prefix_sum = 0i64;
            for k in 0..p as usize - 1 {
                prefix_sum = (prefix_sum + a_vals[k]) % p;
            }
            (prefix_sum + ((n - p + 1) % p) * e_p % p) % p
        } else {
            let mut a_n = 0i64;
            for k in 0..n {
                if k < p - 1 {
                    a_n = (a_n + a_vals[k as usize]) % p;
                } else {
                    a_n = (a_n + e_p) % p;
                }
            }
            a_n
        };

        // B(n) = sum_{j=0}^{min(n-1,p-1)} n^j / j! mod p
        let mut b_n = 0i64;
        let mut n_pow = 1i64;
        let n_mod = n % p;
        let jmax = std::cmp::min(n - 1, p - 1) as usize;
        for j in 0..=jmax {
            b_n = (b_n + n_pow * inv_fact[j]) % p;
            n_pow = n_pow * n_mod % p;
        }

        residues[pi] = (a_n + b_n) % p;
    }

    // CRT: combine residues
    let mut result = 0i64;
    for i in 0..5 {
        let mi = m / primes[i];
        let (_, x, _) = ext_gcd(mi, primes[i]);
        let x = ((x % primes[i]) + primes[i]) % primes[i];
        result = (result + (residues[i] as i128 * mi as i128 % m as i128 * x as i128 % m as i128) as i64) % m;
    }

    println!("{}", result);
}
