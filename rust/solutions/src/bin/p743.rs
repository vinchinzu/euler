// Project Euler 743 - Window into a Matrix
//
// Count 2xN binary matrices where every 2xK sub-matrix sums to K.

const MOD: u64 = 1_000_000_007;

#[inline]
fn mul(a: u64, b: u64) -> u64 {
    a * b % MOD
}

fn pow_mod(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul(result, base);
        }
        exp >>= 1;
        base = mul(base, base);
    }
    result
}

fn main() {
    let n: u64 = 10_000_000_000_000_000; // 10^16
    let k: u64 = 100_000_000; // 10^8
    let half_k = k / 2;

    let mut inv = vec![0u64; half_k as usize + 2];
    inv[1] = 1;
    for i in 2..=half_k as usize + 1 {
        inv[i] = (MOD - (MOD / i as u64) * inv[(MOD % i as u64) as usize] % MOD) % MOD;
    }

    // base = 2^{-2n/k} mod m
    let base = pow_mod(pow_mod(2, 2 * n / k), MOD - 2);

    let mut res = pow_mod(2, n);
    let mut ans: u64 = 0;

    for i in 0..=half_k {
        ans += res;
        if ans >= MOD {
            ans -= MOD;
        }
        if 2 * i < k {
            // SAFETY: i+1 <= half_k+1, inv sized half_k+2
            let inv_ip1 = unsafe { *inv.get_unchecked((i + 1) as usize) };
            let inv_sq = mul(inv_ip1, inv_ip1);
            res = mul(res, inv_sq);
            res = mul(res, (k - 2 * i) % MOD);
            res = mul(res, (k - 2 * i - 1) % MOD);
            res = mul(res, base);
        }
    }

    println!("{}", ans % MOD);
}
