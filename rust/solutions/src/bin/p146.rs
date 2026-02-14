// Project Euler 146 - Investigating a Prime Pattern
// n^2 + {1,3,7,9,13,27} all prime, n^2 + {5,11,15,17,19,21,23,25} all composite.

fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    (a as u128 * b as u128 % m as u128) as u64
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, modulus);
        }
        base = mod_mul(base, base, modulus);
        exp >>= 1;
    }
    result
}

fn miller_rabin_test(n: u64, a: u64) -> bool {
    if n % a == 0 {
        return n == a;
    }
    let mut d = n - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }
    let mut x = mod_pow(a, d, n);
    if x == 1 || x == n - 1 {
        return true;
    }
    for _ in 0..r - 1 {
        x = mod_mul(x, x, n);
        if x == n - 1 {
            return true;
        }
    }
    false
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let witnesses = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &a in &witnesses {
        if !miller_rabin_test(n, a) {
            return false;
        }
    }
    true
}

fn main() {
    let must_prime = [1, 3, 7, 9, 13, 27];
    let must_composite = [5, 11, 15, 17, 19, 21, 23, 25];
    let limit: i64 = 150_000_000;

    let sieve_primes = [2, 3, 5, 7, 11, 13, 17];
    let sieve_mod: i64 = 510510;

    // Build allowed residues per small prime
    let mut allowed_per_p: Vec<Vec<bool>> = Vec::new();
    for &p in &sieve_primes {
        let mut arr = vec![false; p as usize];
        for r in 0..p {
            let sq = (r * r) % p;
            let mut ok = true;
            for &k in &must_prime {
                if (sq + k) % p == 0 {
                    ok = false;
                    break;
                }
            }
            arr[r as usize] = ok;
        }
        allowed_per_p.push(arr);
    }

    // Build allowed residues mod 510510
    let mut allowed = Vec::new();
    for r in 0..sieve_mod {
        let mut ok = true;
        for (pi, &p) in sieve_primes.iter().enumerate() {
            if !allowed_per_p[pi][(r % p as i64) as usize] {
                ok = false;
                break;
            }
        }
        if ok {
            allowed.push(r);
        }
    }

    // Extra prime checks
    let extra_primes = [19, 23, 29, 31, 37, 41, 43];
    let mut extra_allowed: Vec<Vec<bool>> = Vec::new();
    for &p in &extra_primes {
        let mut arr = vec![false; p as usize];
        for r in 0..p {
            let sq = (r * r) % p;
            let mut ok = true;
            for &k in &must_prime {
                if (sq + k) % p == 0 {
                    ok = false;
                    break;
                }
            }
            arr[r as usize] = ok;
        }
        extra_allowed.push(arr);
    }

    let mut total: i64 = 0;
    for &ai in &allowed {
        let mut n = if ai == 0 { sieve_mod } else { ai };
        while n < limit {
            // Quick modular checks
            let mut skip = false;
            for (ei, &p) in extra_primes.iter().enumerate() {
                if !extra_allowed[ei][(n % p as i64) as usize] {
                    skip = true;
                    break;
                }
            }
            if skip {
                n += sieve_mod;
                continue;
            }

            let sq = n as u64 * n as u64;
            let mut all_prime = true;
            for &k in &must_prime {
                if !is_prime(sq + k as u64) {
                    all_prime = false;
                    break;
                }
            }
            if all_prime {
                let mut all_comp = true;
                for &k in &must_composite {
                    if is_prime(sq + k as u64) {
                        all_comp = false;
                        break;
                    }
                }
                if all_comp {
                    total += n;
                }
            }
            n += sieve_mod;
        }
    }

    println!("{}", total);
}
