// Project Euler 643 - 2-Friendly
// Count pairs 1 <= p < q <= N with gcd(p,q) a power of 2.
// Sub-linear sum of Euler's totient with sieve up to N^{2/3}.

const MOD: i64 = 1_000_000_007;

fn power_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut r = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { r = (r as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    r
}

fn main() {
    let n_val: i64 = 100_000_000_000; // 10^11

    // Sieve phi up to V ~ N^{2/3}
    let mut v_size = (n_val as f64).powf(2.0 / 3.0) as usize + 1;
    if v_size < 100 { v_size = 100; }

    let mut phi = vec![0i32; v_size + 1];
    for i in 0..=v_size { phi[i] = i as i32; }
    for i in 2..=v_size {
        if phi[i] == i as i32 { // prime
            let mut j = i;
            while j <= v_size {
                phi[j] -= phi[j] / i as i32;
                j += i;
            }
        }
    }

    // Prefix sums mod MOD
    let mut small = vec![0i64; v_size + 1];
    for i in 1..=v_size {
        small[i] = (small[i - 1] + phi[i] as i64) % MOD;
    }
    drop(phi);

    let inv2 = power_mod(2, MOD - 2, MOD);

    // Hash table for large[i] = S(N/i) for i <= limit
    const HT_SIZE: usize = 1 << 18;
    const HT_MASK: usize = HT_SIZE - 1;

    let mut ht_keys = vec![0i64; HT_SIZE];
    let mut ht_vals = vec![0i64; HT_SIZE];
    let mut ht_used = vec![false; HT_SIZE];

    let ht_hash = |key: i64| -> usize {
        ((key as u64).wrapping_mul(2654435761) >> 14) as usize & HT_MASK
    };

    let limit = (n_val / (v_size as i64 + 1)) as usize + 1;

    for i in (1..=limit).rev() {
        let m = n_val / i as i64;
        if m <= v_size as i64 { continue; }

        let mut result = (m % MOD) * ((m + 1) % MOD) % MOD * inv2 % MOD;

        let mut d: i64 = 2;
        while d <= m {
            let q = m / d;
            let d_max = m / q;
            let s_q = if q <= v_size as i64 {
                small[q as usize]
            } else {
                let idx = n_val / q;
                let mut h = ht_hash(idx);
                loop {
                    if ht_used[h] && ht_keys[h] == idx { break ht_vals[h]; }
                    h = (h + 1) & HT_MASK;
                }
            };
            result = (result - ((d_max - d + 1) % MOD + MOD) % MOD * s_q % MOD + MOD) % MOD;
            d = d_max + 1;
        }

        let mut h = ht_hash(i as i64);
        while ht_used[h] && ht_keys[h] != i as i64 {
            h = (h + 1) & HT_MASK;
        }
        ht_keys[h] = i as i64;
        ht_vals[h] = result % MOD;
        ht_used[h] = true;
    }

    // S(m) lookup
    let get_s = |m: i64| -> i64 {
        if m <= v_size as i64 {
            small[m as usize]
        } else {
            let idx = n_val / m;
            let mut h = ht_hash(idx);
            loop {
                if ht_used[h] && ht_keys[h] == idx { return ht_vals[h]; }
                h = (h + 1) & HT_MASK;
            }
        }
    };

    // Compute the answer
    let mut ans = 0i64;
    let mut t = 1u32;
    while (1i64 << t) <= n_val {
        let lim = n_val >> t;
        let s_lim = get_s(lim);
        ans = (ans + s_lim - 1 + MOD) % MOD;
        t += 1;
    }

    println!("{}", ans % MOD);
}
