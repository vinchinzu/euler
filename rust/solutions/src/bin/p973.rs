// Problem 973: Random Dealings
// Ported from python/973.py.

const MOD: i64 = 1_000_000_007;
const INV2: i64 = (MOD + 1) / 2;

fn mod_pow(mut base: i64, mut exp: usize, modu: i64) -> i64 {
    let mut res = 1_i64;
    base %= modu;
    while exp > 0 {
        if exp & 1 == 1 {
            res = ((res as i128 * base as i128) % modu as i128) as i64;
        }
        base = ((base as i128 * base as i128) % modu as i128) as i64;
        exp >>= 1;
    }
    res
}

fn s_k_value(n: usize, k: usize) -> i64 {
    if k == 0 {
        if n == 0 {
            return 1;
        }
        let t = mod_pow(MOD - 2, n - 1, MOD);
        return (MOD - t) % MOD;
    }

    let b = 1usize << k;
    let period = b << 1;
    let per_mask = period - 1;

    let mut s_vals = vec![0_i64; n + 1];
    let mut p_vals = vec![0_i64; n + 1];
    s_vals[0] = 1;
    p_vals[0] = 1;

    for idx in 1..=n {
        let mut val = 0_i128;
        let mut s = 1usize;
        while s <= idx {
            let p = s & per_mask;
            let (sign, mut e) = if p < b {
                (1_i128, s + (b - p) - 1)
            } else {
                (-1_i128, s + (period - p) - 1)
            };
            if e > idx {
                e = idx;
            }

            let t_lo = idx - e;
            let t_hi = idx - s;
            let sum_range = if t_lo == 0 {
                p_vals[t_hi]
            } else {
                p_vals[t_hi] - p_vals[t_lo - 1]
            };
            val += sign * sum_range as i128;
            s = e + 1;
        }

        let cur = ((val % MOD as i128 + MOD as i128) % MOD as i128) as i64;
        s_vals[idx] = cur;
        p_vals[idx] = (p_vals[idx - 1] + cur) % MOD;
    }

    s_vals[n]
}

fn x_value(n: usize) -> i64 {
    if n == 0 {
        return 0;
    }
    let p = mod_pow(2, n - 1, MOD);
    let mut total = 0_i64;
    for k in 0..14 {
        let sk = s_k_value(n, k);
        let ak = ((((p - sk) % MOD) + MOD) % MOD * INV2) % MOD;
        let term = mod_pow(2, k, MOD);
        total = (total + (term as i128 * ak as i128 % MOD as i128) as i64) % MOD;
    }
    (total - (n as i64 & 1) + MOD) % MOD
}

fn main() {
    println!("{}", x_value(10_000));
}
