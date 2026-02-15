// Project Euler 621 - Sum of three triangular numbers
// Sieve-based factorization for G(n) using divisor counts mod 4

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn pow_mod_local(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut r = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { r = r * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    r
}

fn main() {
    let n_val: i64 = 17_526_000_000_000;
    let q0 = 4 * n_val + 1;
    let mut l_val = ((((8.0 * n_val as f64 + 1.0).sqrt()) - 1.0) / 2.0) as i64;
    while l_val * (l_val + 1) / 2 > n_val { l_val -= 1; }
    while (l_val + 1) * (l_val + 2) / 2 <= n_val { l_val += 1; }

    let sz = (l_val + 1) as usize;
    let mut remaining = vec![0i64; sz];
    let mut result = vec![1i32; sz];

    for k in 0..sz {
        remaining[k] = q0 - 2 * (k as i64) * (k as i64 + 1);
    }

    let sieve_limit = isqrt(q0) as usize + 2;
    let mut is_prime = vec![true; sieve_limit + 1];
    is_prime[0] = false;
    if sieve_limit >= 1 { is_prime[1] = false; }
    let mut i = 2;
    while i * i <= sieve_limit { if is_prime[i] { let mut j = i*i; while j <= sieve_limit { is_prime[j] = false; j += i; } } i += 1; }

    let m_val = 2 * q0 + 1;

    let mut p = 3;
    while p <= sieve_limit {
        if is_prime[p] {
            let pp = p as i64;
            let m_mod_p = m_val % pp;
            let euler = pow_mod_local(m_mod_p, (pp - 1) / 2, pp);
            if euler != 1 && m_mod_p != 0 { p += 2; continue; }

            let sq = if m_mod_p == 0 {
                0i64
            } else if pp % 4 == 3 {
                pow_mod_local(m_mod_p, (pp + 1) / 4, pp)
            } else {
                // Tonelli-Shanks
                let mut q_ts = pp - 1;
                let mut s_ts = 0;
                while q_ts % 2 == 0 { q_ts /= 2; s_ts += 1; }
                let mut z = 2i64;
                while pow_mod_local(z, (pp - 1) / 2, pp) == 1 { z += 1; }
                let mut mm = s_ts;
                let mut c = pow_mod_local(z, q_ts, pp);
                let mut t = pow_mod_local(m_mod_p, q_ts, pp);
                let mut r_val = pow_mod_local(m_mod_p, (q_ts + 1) / 2, pp);
                while t != 1 {
                    let mut tt = t;
                    let mut ii = 0;
                    while tt != 1 { tt = tt * tt % pp; ii += 1; }
                    let mut b2 = c;
                    for _ in 0..mm - ii - 1 { b2 = b2 * b2 % pp; }
                    mm = ii;
                    c = b2 * b2 % pp;
                    t = t * c % pp;
                    r_val = r_val * b2 % pp;
                }
                r_val
            };

            let inv2 = (pp + 1) / 2;
            let mut roots = Vec::new();
            if m_mod_p == 0 {
                roots.push((pp - 1) * inv2 % pp);
            } else {
                let r0 = (sq - 1 + pp) % pp * inv2 % pp;
                let r1 = (pp - sq - 1 + pp) % pp * inv2 % pp;
                roots.push(r0);
                if r0 != r1 { roots.push(r1); }
            }

            let p_mod4 = pp % 4;
            for &root in &roots {
                let mut k = root as usize;
                while k < sz {
                    let mut e = 0;
                    while remaining[k] % pp == 0 {
                        remaining[k] /= pp;
                        e += 1;
                    }
                    if e > 0 {
                        if p_mod4 == 1 {
                            result[k] *= e + 1;
                        } else if e % 2 == 1 {
                            result[k] = 0;
                        }
                    }
                    k += p as usize;
                }
            }
        }
        p += 2;
    }

    let mut answer: i64 = 0;
    for k in 0..sz {
        if result[k] == 0 { continue; }
        let r = remaining[k];
        if r == 1 {
            answer += result[k] as i64;
        } else if r % 4 == 1 {
            answer += result[k] as i64 * 2;
        }
    }

    println!("{}", answer);
}
