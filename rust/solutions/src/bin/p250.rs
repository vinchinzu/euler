// Project Euler 250: 250250
// Non-empty subsets of {n^n : n=1..250250} with sum ≡ 0 (mod 250), mod 10^16.

const N: u32 = 250_250;
const K: usize = 250;
const MOD: u64 = 10_000_000_000_000_000;

#[inline(always)]
fn add_mod(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= MOD { s - MOD } else { s }
}

#[inline(always)]
fn mul_mod(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % (MOD as u128)) as u64
}

fn pow_u32(mut base: u32, mut exp: u32, m: u32) -> u32 {
    let mut r = 1u32;
    base %= m;
    while exp > 0 {
        if exp & 1 != 0 {
            r = r * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    r
}

fn pow2_mod(mut exp: u64) -> u64 {
    let mut r = 1u64;
    let mut base = 2u64;
    while exp > 0 {
        if exp & 1 != 0 {
            r = mul_mod(r, base);
        }
        base = mul_mod(base, base);
        exp >>= 1;
    }
    r
}

/// n^n mod 250. 250 = 2*125; φ(125)=100. Multiples of 5: 0 if even, 125 if odd.
fn nn_mod_250(n: u32) -> usize {
    if n % 5 == 0 {
        return if n % 2 == 0 { 0 } else { 125 };
    }
    if n & 1 == 1 {
        return pow_u32(n, n % 100, 250) as usize;
    }
    let n125 = pow_u32(n, n % 100, 125);
    if n125 & 1 == 0 {
        n125 as usize
    } else {
        (n125 + 125) as usize
    }
}

/// ndp[i] = dp[i] + dp[(i - val) mod K]  (one copy of residue `val`)
#[inline(always)]
fn apply_val(dp: &[u64; K], ndp: &mut [u64; K], val: usize) {
    // SAFETY: val ∈ 1..K, so i and i+K-val / i-val are in 0..K.
    unsafe {
        for i in 0..val {
            let s = *dp.get_unchecked(i) + *dp.get_unchecked(i + K - val);
            *ndp.get_unchecked_mut(i) = if s >= MOD { s - MOD } else { s };
        }
        for i in val..K {
            let s = *dp.get_unchecked(i) + *dp.get_unchecked(i - val);
            *ndp.get_unchecked_mut(i) = if s >= MOD { s - MOD } else { s };
        }
    }
}

fn main() {
    let mut freq = [0u32; K];
    for n in 1..=N {
        freq[nn_mod_250(n)] += 1;
    }

    let mut a = [0u64; K];
    let mut b = [0u64; K];
    a[0] = 1;

    let mut dp: &mut [u64; K] = &mut a;
    let mut ndp: &mut [u64; K] = &mut b;

    let f0 = freq[0];
    let f125 = freq[125];

    for val in 1..K {
        if val == 125 {
            continue;
        }
        for _ in 0..freq[val] {
            apply_val(dp, ndp, val);
            core::mem::swap(&mut dp, &mut ndp);
        }
    }

    // (1 + x^125)^f = 2^{f-1} (1 + x^125) for f >= 1
    if f125 > 0 {
        let factor = pow2_mod((f125 - 1) as u64);
        for i in 0..125 {
            let v = mul_mod(add_mod(dp[i], dp[i + 125]), factor);
            dp[i] = v;
            dp[i + 125] = v;
        }
    }

    // (1 + x^0)^f0 = 2^{f0}
    if f0 > 0 {
        let factor = pow2_mod(f0 as u64);
        for x in dp.iter_mut() {
            *x = mul_mod(*x, factor);
        }
    }

    let ans = (dp[0] + MOD - 1) % MOD;
    println!("{}", ans);
}
