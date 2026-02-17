// Problem 977: Iterated Functions
// Ported from python/977.py.

const MOD: u64 = 1_000_000_007;

#[inline]
fn mod_mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % MOD as u128) as u64
}

fn mod_pow(mut base: u64, mut exp: usize) -> u64 {
    let mut res = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            res = mod_mul(res, base);
        }
        base = mod_mul(base, base);
        exp >>= 1;
    }
    res
}

fn count_mod(n: usize) -> u64 {
    if n == 1 {
        return 1;
    }

    let mut total: u64;

    // L = 1 closed form.
    let m = (n - 2) as u128;
    let sum_q = m * (m + 1) * (2 * m + 1) / 6 + m * (m + 1) / 2;
    total = ((sum_q + n as u128) % MOD as u128) as u64;

    for l in 2..=n {
        let r = n - l;

        if r >= 1 {
            let q_full = (r - 1) / l;
            let max_a = q_full + 2;

            let mut pow_a = vec![0u64; max_a + 1];
            if l == 2 {
                for (a, slot) in pow_a.iter_mut().enumerate().take(max_a + 1).skip(1) {
                    *slot = mod_mul(a as u64, a as u64);
                }
            } else if l == 3 {
                for (a, slot) in pow_a.iter_mut().enumerate().take(max_a + 1).skip(1) {
                    let aa = mod_mul(a as u64, a as u64);
                    *slot = mod_mul(aa, a as u64);
                }
            } else {
                for (a, slot) in pow_a.iter_mut().enumerate().take(max_a + 1).skip(1) {
                    *slot = mod_pow(a as u64, l);
                }
            }

            for q in 0..q_full {
                let a = q + 1;
                let b = q + 2;
                let a_l = pow_a[a];
                let b_l = pow_a[b];
                let a_l1 = mod_mul(a_l, a as u64);
                let t0 = mod_mul(q as u64 % MOD, a_l);
                let t1 = mod_mul(mod_mul(a as u64 % MOD, a as u64 % MOD), b_l);
                let t2 = mod_mul(b as u64 % MOD, a_l1);
                let term = (t0 + t1 + MOD - t2) % MOD;
                total += term;
                if total >= MOD {
                    total -= MOD;
                }
            }

            // Last partial block.
            let q = q_full;
            let m_last = (r - 1) - q_full * l;
            let a = q + 1;
            let b = q + 2;
            let a_l = pow_a[a];
            let mut term = mod_mul(q as u64 % MOD, a_l);

            if m_last >= 1 {
                let a_l1 = mod_mul(a_l, a as u64 % MOD);
                let exp = l + 1 - m_last;
                let a_l1_m = match exp {
                    1 => a as u64 % MOD,
                    2 => mod_mul(a as u64 % MOD, a as u64 % MOD),
                    3 => {
                        let aa = mod_mul(a as u64 % MOD, a as u64 % MOD);
                        mod_mul(aa, a as u64 % MOD)
                    }
                    _ => mod_pow(a as u64, exp),
                };
                let b_m = match m_last {
                    1 => b as u64 % MOD,
                    2 => mod_mul(b as u64 % MOD, b as u64 % MOD),
                    3 => {
                        let bb = mod_mul(b as u64 % MOD, b as u64 % MOD);
                        mod_mul(bb, b as u64 % MOD)
                    }
                    _ => mod_pow(b as u64, m_last),
                };
                let inner = (mod_mul(a_l1_m, b_m) + MOD - a_l1) % MOD;
                term = (term + mod_mul(b as u64 % MOD, inner)) % MOD;
            }
            total += term;
            if total >= MOD {
                total -= MOD;
            }
        }

        // mu = 0 contribution.
        let q = r / l;
        let rem = r - q * l;
        let a = q + 1;
        let b = q + 2;
        let base = if rem == 0 {
            mod_pow(a as u64, l)
        } else {
            mod_mul(mod_pow(a as u64, l - rem), mod_pow(b as u64, rem))
        };
        total += base;
        if total >= MOD {
            total -= MOD;
        }
    }

    total % MOD
}

fn main() {
    println!("{}", count_mod(1_000_000));
}
