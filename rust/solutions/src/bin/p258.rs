const K: usize = 2000;
const MOD: i64 = 20092010;

fn poly_mul_mod(a: &[i64], b: &[i64], out: &mut [i64]) {
    let mut tmp = vec![0i64; 2 * K - 1];

    // Deferred reduction: MOD ≈ 2×10^7, products < 4×10^14,
    // sum of K=2000 products < 8×10^17 < i64::MAX. Safe to accumulate in i64.
    for i in 0..K {
        if a[i] == 0 { continue; }
        for j in 0..K {
            tmp[i + j] += a[i] * b[j];
        }
    }
    // Reduce once after accumulation
    for v in tmp.iter_mut() {
        *v %= MOD;
    }

    for i in (K..=(2 * K - 2)).rev() {
        if tmp[i] == 0 { continue; }
        let c = tmp[i];
        tmp[i] = 0;
        tmp[i - K + 1] = (tmp[i - K + 1] + c) % MOD;
        tmp[i - K] = (tmp[i - K] + c) % MOD;
    }

    out[..K].copy_from_slice(&tmp[..K]);
}

fn main() {
    let mut base = vec![0i64; K];
    let mut result = vec![0i64; K];
    let mut temp = vec![0i64; K];

    base[1] = 1;
    result[0] = 1;

    let mut n: i64 = 1_000_000_000_000_000_000;

    while n > 0 {
        if n & 1 != 0 {
            poly_mul_mod(&result, &base, &mut temp);
            result.copy_from_slice(&temp);
        }
        poly_mul_mod(&base.clone(), &base, &mut temp);
        base.copy_from_slice(&temp);
        n >>= 1;
    }

    let mut ans: i64 = 0;
    for i in 0..K {
        ans = (ans + result[i]) % MOD;
    }
    println!("{}", ans);
}
