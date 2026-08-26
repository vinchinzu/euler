// Project Euler 374 - Maximum Integer Partition Product

use rayon::prelude::*;

const MOD: u64 = 982_451_653;
const INV2: u64 = (MOD + 1) / 2;

fn mod_pow(mut base: u64, mut exp: u64) -> u64 {
    let mut acc = 1u64;
    while exp > 0 {
        if exp & 1 != 0 {
            acc = acc * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    acc
}

fn main() {
    let n_big: i64 = 100_000_000_000_000; // 10^14

    // Find K such that T_K <= N < T_{K+1}
    let mut lo: i64 = 1;
    let mut hi: i64 = 20_000_000;
    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        if (mid as i128) * (mid as i128 + 1) / 2 <= n_big as i128 {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    let k_val = lo as usize;
    let sz = k_val + 3;

    let mut fact = vec![0u32; sz];
    fact[0] = 1;
    {
        let f = fact.as_mut_ptr();
        let mut prev = 1u64;
        for i in 1..sz {
            prev = prev * (i as u64) % MOD;
            unsafe {
                *f.add(i) = prev as u32;
            }
        }
    }

    // 1/i! via one Fermat inverse, then prefix products — avoids variable-divisor divs.
    let mut harmonic = vec![0u32; sz];
    {
        let hptr = harmonic.as_mut_ptr();
        unsafe {
            *hptr.add(sz - 1) = mod_pow(fact[sz - 1] as u64, MOD - 2) as u32;
        }
        let mut prev = unsafe { *hptr.add(sz - 1) } as u64;
        for i in (1..sz).rev() {
            prev = prev * (i as u64) % MOD;
            unsafe {
                *hptr.add(i - 1) = prev as u32;
            }
        }
    }
    let inv_kp1 = fact[k_val] as u64 * harmonic[k_val + 1] as u64 % MOD;

    // Overwrite 1/i! with H_i = sum_{j=2}^{i} 1/j
    {
        let f = fact.as_ptr();
        let hptr = harmonic.as_mut_ptr();
        let mut h = 0u64;
        unsafe {
            *hptr = 0;
            *hptr.add(1) = 0;
        }
        for i in 2..sz {
            let inv_i = unsafe { *f.add(i - 1) as u64 * *hptr.add(i) as u64 % MOD };
            h += inv_i;
            if h >= MOD {
                h -= MOD;
            }
            unsafe {
                *hptr.add(i) = h as u32;
            }
        }
    }

    // k=1,2: n=1..5 contributions
    let mut total: u64 = 22;

    // k = 3..K-1: r_max = k, so Case 1+2+3 collapse to two mulmods.
    const CHUNK: usize = 65_536;
    let n_chunks = (k_val - 3 + CHUNK - 1) / CHUNK;
    let par: u64 = (0..n_chunks)
        .into_par_iter()
        .map(|ci| {
            let start = 3 + ci * CHUNK;
            let end = (start + CHUNK).min(k_val);
            let fp = fact.as_ptr();
            let hp = harmonic.as_ptr();
            let mut local = 0u64;
            for k in start..end {
                let ku = k as u64;
                unsafe {
                    let fk = *fp.add(k) as u64;
                    let fk1 = *fp.add(k + 1) as u64;
                    let hk = *hp.add(k) as u64;
                    let c13 = fk1 * (((ku - 1) * hk + ku) % MOD) % MOD;
                    let c2 = fk * (((ku + 2) * (ku - 1) / 2) % MOD) % MOD;
                    local += c13 + c2;
                }
            }
            local
        })
        .sum();
    total += par;

    // k = K may have a truncated r_max
    {
        let k = k_val as i64;
        let t_k = k * (k + 1) / 2;
        let mut r_max = n_big - t_k;
        if r_max > k {
            r_max = k;
        }

        let r1 = std::cmp::min(k - 2, r_max);
        if r1 >= 0 {
            let j_min = k - r1;
            let hk = harmonic[k as usize] as u64;
            let sum_inv = if j_min <= 1 {
                (1 + hk) % MOD
            } else {
                (hk + MOD - harmonic[(j_min - 1) as usize] as u64) % MOD
            };
            total += fact[(k + 1) as usize] as u64 * ((k - 1) as u64) % MOD * sum_inv % MOD;
        }
        if r_max >= k - 1 {
            total += fact[(k + 2) as usize] as u64 * ((k - 1) as u64) % MOD * INV2 % MOD * inv_kp1
                % MOD;
        }
        if r_max >= k {
            total += fact[(k + 1) as usize] as u64 * (k as u64) % MOD;
        }
    }

    println!("{}", total % MOD);
}
