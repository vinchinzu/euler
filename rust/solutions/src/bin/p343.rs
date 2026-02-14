// Project Euler 343 - Fractional Sequences
// f(k) = largest_prime_factor(k+1) - 1
// Sum f(k^3) for k=1..2000000.

const N: usize = 2_000_000;
const SZ: usize = N + 2;

fn mpow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut r = 1i64;
    base = base.rem_euclid(modulus);
    while exp > 0 {
        if exp & 1 == 1 {
            r = (r as i128 * base as i128 % modulus as i128) as i64;
        }
        base = (base as i128 * base as i128 % modulus as i128) as i64;
        exp >>= 1;
    }
    r
}

fn modsqrt(n: i64, p: i64) -> i64 {
    let n = n.rem_euclid(p);
    if n == 0 {
        return 0;
    }
    if p == 2 {
        return n & 1;
    }
    if mpow(n, (p - 1) / 2, p) != 1 {
        return -1;
    }
    let mut s = 0i64;
    let mut q = p - 1;
    while q % 2 == 0 {
        s += 1;
        q /= 2;
    }
    if s == 1 {
        return mpow(n, (p + 1) / 4, p);
    }
    let mut z = 2i64;
    while mpow(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let mut m_val = s;
    let mut c = mpow(z, q, p);
    let mut t = mpow(n, q, p);
    let mut r = mpow(n, (q + 1) / 2, p);
    loop {
        if t == 1 {
            return r;
        }
        let mut i = 1i64;
        let mut tmp = (t as i128 * t as i128 % p as i128) as i64;
        while tmp != 1 {
            tmp = (tmp as i128 * tmp as i128 % p as i128) as i64;
            i += 1;
        }
        let mut b = c;
        for _ in 0..(m_val - i - 1) {
            b = (b as i128 * b as i128 % p as i128) as i64;
        }
        m_val = i;
        c = (b as i128 * b as i128 % p as i128) as i64;
        t = (t as i128 * c as i128 % p as i128) as i64;
        r = (r as i128 * b as i128 % p as i128) as i64;
    }
}

fn main() {
    // Build sieve for largest prime factor of k+1
    let mut lpf_kp1 = vec![0i32; SZ];
    let mut primes_list: Vec<i32> = Vec::new();
    for i in 2..SZ {
        if lpf_kp1[i] == 0 {
            primes_list.push(i as i32);
            let mut j = i;
            while j < SZ {
                lpf_kp1[j] = i as i32;
                j += i;
            }
        }
    }

    // Init cofactors: cofactor[k] = k^2 - k + 1
    let mut cofactor = vec![0i64; SZ];
    let mut best_small = vec![0i32; SZ];
    for k in 1..=N {
        cofactor[k] = k as i64 * k as i64 - k as i64 + 1;
    }

    // Sieve polynomial x^2 - x + 1 by each prime
    for &p in &primes_list {
        let pp = p as i64;
        if p == 2 {
            continue;
        }
        if p == 3 {
            let mut k = 2usize;
            while k <= N {
                while cofactor[k] % 3 == 0 {
                    cofactor[k] /= 3;
                    best_small[k] = 3;
                }
                k += 3;
            }
            continue;
        }

        let sq = modsqrt(pp - 3, pp);
        if sq < 0 {
            continue;
        }

        let inv2 = (pp + 1) / 2;
        let r1 = ((1 + sq) as i128 * inv2 as i128 % pp as i128) as i64;
        let r2 = ((1 + pp - sq) as i128 * inv2 as i128 % pp as i128) as i64;
        let r1 = if r1 == 0 { pp } else { r1 };
        let r2 = if r2 == 0 { pp } else { r2 };

        let mut k = r1 as usize;
        while k <= N {
            while cofactor[k] % pp == 0 {
                cofactor[k] /= pp;
                if p > best_small[k] {
                    best_small[k] = p;
                }
            }
            k += pp as usize;
        }
        if r2 != r1 {
            let mut k = r2 as usize;
            while k <= N {
                while cofactor[k] % pp == 0 {
                    cofactor[k] /= pp;
                    if p > best_small[k] {
                        best_small[k] = p;
                    }
                }
                k += pp as usize;
            }
        }
    }

    let mut total: i64 = 0;
    for k in 1..=N {
        let lpf_q = if cofactor[k] > 1 {
            cofactor[k]
        } else if best_small[k] > 0 {
            best_small[k] as i64
        } else {
            1
        };

        let lpf1 = lpf_kp1[k + 1] as i64;
        let lpf = std::cmp::max(lpf1, lpf_q);
        total += lpf - 1;
    }
    println!("{}", total);
}
