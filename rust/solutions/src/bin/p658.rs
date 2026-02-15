// Project Euler 658 - Incomplete Words II
// Sum over 1<=k<=K of I(k), N=10^12, K=10^7, M=10^9+7.

const MOD: i64 = 1_000_000_007;

fn power_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut r = 1i64;
    base %= m; if base < 0 { base += m; }
    while exp > 0 {
        if exp & 1 == 1 { r = (r as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    r
}

fn main() {
    let n: i64 = 1_000_000_000_000;
    let k = 10_000_000usize;
    let mut spf = vec![0u32; k + 1];
    for i in 0..=k { spf[i] = i as u32; }
    for i in 2..=k {
        if (i as u64) * (i as u64) > k as u64 { break; }
        if spf[i] == i as u32 {
            for j in (i*i..=k).step_by(i) { if spf[j] == j as u32 { spf[j] = i as u32; } }
        }
    }
    let mut pows = vec![0i64; k + 1];
    pows[0] = 0; pows[1] = 1;
    for i in 2..=k {
        if spf[i] == i as u32 { pows[i] = power_mod(i as i64, n + 1, MOD); }
        else { pows[i] = (pows[spf[i] as usize] as i128 * pows[i / spf[i] as usize] as i128 % MOD as i128) as i64; }
    }
    let mut invs = vec![0i64; k + 1];
    invs[1] = 1;
    for i in 2..=k { invs[i] = (MOD - (MOD / i as i64) * invs[(MOD % i as i64) as usize] % MOD) % MOD; }
    let inv2 = power_mod(2, MOD - 2, MOD);
    let mut ans = 0i64;
    let mut ncr = 1i64;
    let mut inner_sum = (k % 2) as i64;
    for t in 0..k {
        let num_words = if t == 0 { 1i64 }
            else if t == 1 { (n + 1) % MOD }
            else { (pows[t] - 1 + MOD) % MOD * invs[t - 1] % MOD };
        ans = (ans + (num_words as i128 * inner_sum as i128 % MOD as i128) as i64) % MOD;
        if t < k - 1 {
            let new_ncr = (ncr as i128 * ((k - t) as i64 % MOD) as i128 % MOD as i128 * invs[t + 1] as i128 % MOD as i128) as i64;
            let parity_val = if (k - t) % 2 == 0 { 1i64 } else { MOD - 1 };
            inner_sum = (inner_sum + 1 + (parity_val as i128 * ((ncr + new_ncr) % MOD) as i128 % MOD as i128) as i64) % MOD;
            inner_sum = (inner_sum as i128 * inv2 as i128 % MOD as i128) as i64;
            ncr = new_ncr;
        }
    }
    println!("{}", ans % MOD);
}
