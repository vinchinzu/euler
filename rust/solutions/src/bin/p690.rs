// Project Euler 690 - Tom and Jerry
// Count graphs on N=2019 vertices where every component is a lobster graph.
// Uses generating functions with partition numbers and DP.

use rayon::prelude::*;

const MAXN: usize = 2020;
const MOD: i64 = 1_000_000_007;
const MODU: u64 = MOD as u64;

fn power_mod(mut base: u64, mut exp: u64) -> u64 {
    let mut r = 1u64;
    base %= MODU;
    while exp > 0 {
        if exp & 1 == 1 { r = r * base % MODU; }
        base = base * base % MODU;
        exp >>= 1;
    }
    r
}

fn inv_mod(a: i64) -> i64 { power_mod(((a % MOD + MOD) % MOD) as u64, (MOD - 2) as u64) as i64 }

fn gf_mul(a: &[i64; MAXN], b: &[i64; MAXN]) -> [i64; MAXN] {
    // Heap partials: rayon worker stacks cannot hold [u64; MAXN] fold identities.
    (0..MAXN)
        .into_par_iter()
        .fold(
            || vec![0u64; MAXN],
            |mut out, i| {
                let ai = a[i] as u64;
                if ai != 0 {
                    for j in 0..MAXN - i {
                        out[i + j] = (out[i + j] + ai * b[j] as u64) % MODU;
                    }
                }
                out
            },
        )
        .reduce(
            || vec![0u64; MAXN],
            |mut x, y| {
                for i in 0..MAXN {
                    x[i] = (x[i] + y[i]) % MODU;
                }
                x
            },
        )
        .iter()
        .map(|&v| v as i64)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn gf_recip(f: &[i64; MAXN]) -> [i64; MAXN] {
    let mut out = [0i64; MAXN];
    let inv_f0 = inv_mod(f[0]) as u64;
    out[0] = inv_f0 as i64;
    for i in 1..MAXN {
        let mut s = 0u64;
        for j in 1..=i {
            s += f[j] as u64 * out[i - j] as u64;
            if j & 15 == 0 {
                s %= MODU;
            }
        }
        s %= MODU;
        out[i] = ((MODU - s) % MODU * inv_f0 % MODU) as i64;
    }
    out
}

fn main() {
    let n = 2019;

    // Partition numbers
    let mut p = [0i64; MAXN];
    p[0] = 1;
    for k in 1..MAXN {
        for i in k..MAXN {
            p[i] = (p[i] + p[i - k]) % MOD;
        }
    }

    // P2[2i] = P[i]
    let mut p2 = [0i64; MAXN];
    for i in 0..MAXN {
        if 2 * i < MAXN { p2[2 * i] = p[i]; }
    }

    // 1/(1-x)
    let mut recip_1mx = [0i64; MAXN];
    for i in 0..MAXN { recip_1mx[i] = 1; }

    // 1/(1-x^2)
    let mut recip_1mx2 = [0i64; MAXN];
    for i in (0..MAXN).step_by(2) { recip_1mx2[i] = 1; }

    // term1 = P - 1/(1-x)
    let mut term1 = [0i64; MAXN];
    for i in 0..MAXN {
        term1[i] = (p[i] - recip_1mx[i] + MOD) % MOD;
    }

    let term1_sq = gf_mul(&term1, &term1);

    // xP
    let mut x_p = [0i64; MAXN];
    for i in 1..MAXN { x_p[i] = p[i - 1]; }

    // one_minus_xP
    let mut one_minus_xp = [0i64; MAXN];
    one_minus_xp[0] = 1;
    for i in 0..MAXN {
        one_minus_xp[i] = (one_minus_xp[i] - x_p[i] + MOD) % MOD;
    }

    let recip_omxp = gf_recip(&one_minus_xp);
    let part_a = gf_mul(&term1_sq, &recip_omxp);

    // term2 = P2 - 1/(1-x^2)
    let mut term2 = [0i64; MAXN];
    for i in 0..MAXN {
        term2[i] = (p2[i] - recip_1mx2[i] + MOD) % MOD;
    }

    // one_plus_xP
    let mut one_plus_xp = [0i64; MAXN];
    one_plus_xp[0] = 1;
    for i in 0..MAXN {
        one_plus_xp[i] = (one_plus_xp[i] + x_p[i]) % MOD;
    }

    // x^2 * P2
    let mut x2p2 = [0i64; MAXN];
    for i in 2..MAXN { x2p2[i] = p2[i - 2]; }

    // one_minus_x2P2
    let mut one_minus_x2p2 = [0i64; MAXN];
    one_minus_x2p2[0] = 1;
    for i in 0..MAXN {
        one_minus_x2p2[i] = (one_minus_x2p2[i] - x2p2[i] + MOD) % MOD;
    }

    let recip_omx2p2 = gf_recip(&one_minus_x2p2);
    let tmp_b1 = gf_mul(&term2, &one_plus_xp);
    let part_b = gf_mul(&tmp_b1, &recip_omx2p2);

    // inner = x^2 * (part_a + part_b) / 2
    let inv2 = inv_mod(2) as u64;
    let mut inner = [0i64; MAXN];
    for i in 2..MAXN {
        inner[i] = ((part_a[i - 2] as u64 + part_b[i - 2] as u64) % MODU * inv2 % MODU) as i64;
    }

    // correction = x^3 / ((1-x)^2 * (1+x))
    let mut denom = [0i64; MAXN];
    denom[0] = 1;
    denom[1] = MOD - 1;
    denom[2] = MOD - 1;
    denom[3] = 1;
    let recip_denom = gf_recip(&denom);
    let mut correction = [0i64; MAXN];
    for i in 3..MAXN { correction[i] = recip_denom[i - 3]; }

    // num_lobsters = inner + x*P - correction
    let mut num_lobsters = [0i64; MAXN];
    for i in 0..MAXN {
        num_lobsters[i] = (inner[i] + x_p[i] + 2 * MOD - correction[i]) % MOD;
    }

    // DP for counting graphs whose components are all lobsters
    let mut mod_invs = vec![0i64; n + 2];
    mod_invs[1] = 1;
    for i in 2..=n + 1 {
        mod_invs[i] = (MOD - (MOD / i as i64) * mod_invs[(MOD % i as i64) as usize] % MOD) % MOD;
    }

    let mut dp = vec![0u64; n + 1];
    dp[0] = 1;
    let mut tmp = vec![0u64; n + 1];
    let mut ncr_buf = vec![0u64; n + 1];

    for j in 1..=n {
        let nj = num_lobsters[j] as u64;
        let max_k = n / j;
        ncr_buf[0] = 1;
        for k in 1..=max_k {
            let term = nj + k as u64 - 1;
            let term = if term >= MODU { term - MODU } else { term };
            ncr_buf[k] = ncr_buf[k - 1] * term % MODU * mod_invs[k] as u64 % MODU;
        }

        tmp[..j].copy_from_slice(&dp[..j]);
        let ncr = &ncr_buf;
        let prev = &dp;
        if n - j >= 64 {
            tmp[j..n + 1]
                .par_iter_mut()
                .with_min_len(8)
                .enumerate()
                .for_each(|(off, slot)| {
                    let i = j + off;
                    let mut val = prev[i];
                    let mut k = 1;
                    while k * j <= i {
                        val = (val + ncr[k] * prev[i - j * k] % MODU) % MODU;
                        k += 1;
                    }
                    *slot = val;
                });
        } else {
            for i in j..=n {
                let mut val = dp[i];
                let mut k = 1;
                while k * j <= i {
                    val = (val + ncr_buf[k] * dp[i - j * k] % MODU) % MODU;
                    k += 1;
                }
                tmp[i] = val;
            }
        }
        std::mem::swap(&mut dp, &mut tmp);
    }

    println!("{}", dp[n]);
}
