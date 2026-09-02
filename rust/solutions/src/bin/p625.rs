// Project Euler 625 - Gcd sum
use rayon::prelude::*;

const NN: u64 = 100_000_000_000;
const MOD: u64 = 998_244_353;
const INV2: u64 = (MOD + 1) / 2;

fn main() {
    let sieve_limit = 8_500_000usize;
    let slen = sieve_limit + 1;

    let mut primes_odd = Vec::with_capacity(300_000);
    let mut phi = vec![0u32; slen];
    phi[1] = 1;

    let limit = slen / 3;
    let mut i = 3;
    let phi_ptr = phi.as_mut_ptr();

    while i <= limit {
        let mut phi_i = unsafe { *phi_ptr.add(i) };
        if phi_i == 0 {
            phi_i = (i - 1) as u32;
            unsafe { *phi_ptr.add(i) = phi_i; }
            primes_odd.push(i as u32);
        }
        for &p in &primes_odd {
            let pi = p as usize * i;
            if pi >= slen { break; }
            if i % (p as usize) == 0 {
                unsafe { *phi_ptr.add(pi) = phi_i * p; }
                break;
            } else {
                unsafe { *phi_ptr.add(pi) = phi_i * (p - 1); }
            }
        }
        i += 2;
    }

    while i < slen {
        let phi_i = unsafe { *phi_ptr.add(i) };
        if phi_i == 0 {
            unsafe { *phi_ptr.add(i) = (i - 1) as u32; }
        }
        i += 2;
    }
    drop(primes_odd);

    let half = (slen - 1) / 2;
    for j in 1..=half {
        let v = phi[j];
        phi[2 * j] = if (j & 1) != 0 { v } else { v << 1 };
    }

    const CHUNK: usize = 500_000;
    let mod_u32 = MOD as u32;
    let chunk_totals: Vec<u32> = phi[1..slen]
        .par_chunks_mut(CHUNK)
        .map(|chunk| {
            let mut s = 0u32;
            for x in chunk.iter_mut() {
                s += *x;
                if s >= mod_u32 { s -= mod_u32; }
                *x = s;
            }
            s
        })
        .collect();

    let mut chunk_offsets = Vec::with_capacity(chunk_totals.len());
    let mut acc = 0u32;
    for &tot in &chunk_totals {
        chunk_offsets.push(acc);
        acc += tot;
        if acc >= mod_u32 { acc -= mod_u32; }
    }

    phi[1..slen]
        .par_chunks_mut(CHUNK)
        .zip(chunk_offsets.into_par_iter())
        .for_each(|(chunk, offset)| {
            if offset != 0 {
                for x in chunk.iter_mut() {
                    *x += offset;
                    if *x >= mod_u32 { *x -= mod_u32; }
                }
            }
        });

    let max_t = (NN / sieve_limit as u64) as usize;
    let mut big = vec![0u32; max_t + 2];

    let mut high = max_t;
    while high >= 1 {
        let low = (high / 2) + 1;
        let results: Vec<(usize, u32)> = (low..=high)
            .into_par_iter()
            .map(|t| {
                let n = NN / t as u64;
                let n_mod = n % MOD;
                let total = (n_mod * ((n_mod + 1) % MOD)) % MOD * INV2 % MOD;

                let m = (n as f64).sqrt() as u64;
                let limit_d = n / (m + 1);
                let mut sub_sum: u128 = 0;

                let split = (n / slen as u64).min(limit_d);
                for d in 2..=split {
                    let q = n / d;
                    sub_sum += big[(NN / q) as usize] as u128;
                }
                for d in (split + 1)..=limit_d {
                    let q = n / d;
                    sub_sum += phi[q as usize] as u128;
                }

                let mut prev_d = limit_d;
                for q in (1..=m).rev() {
                    let d2 = n / q;
                    let mut count = d2 - prev_d;
                    if count >= MOD { count %= MOD; }
                    let sq = phi[q as usize] as u64;
                    sub_sum += (count * sq) as u128;
                    prev_d = d2;
                }

                let sub = (sub_sum % (MOD as u128)) as u64;
                let res = (total + MOD - sub) % MOD;
                (t, res as u32)
            })
            .collect();

        for (t, val) in results {
            big[t] = val;
        }
        high = low - 1;
    }

    let m_nn = (NN as f64).sqrt() as u64;
    let limit_k = NN / (m_nn + 1);
    let mut ans_acc: u128 = 0;

    let split_k = (NN / slen as u64).min(limit_k);
    for k in 1..=split_k {
        let sn = big[k as usize] as u64;
        ans_acc += (k * sn) as u128;
    }
    for k in (split_k + 1)..=limit_k {
        let q = NN / k;
        let sn = phi[q as usize] as u64;
        ans_acc += (k * sn) as u128;
    }

    let mut prev_k = limit_k;
    for q in (1..=m_nn).rev() {
        let k2 = NN / q;
        let kmod = (prev_k + 1) % MOD;
        let k2mod = k2 % MOD;
        let range_sum = (kmod + k2mod) % MOD * ((k2mod + MOD - kmod + 1) % MOD) % MOD * INV2 % MOD;
        let sn = phi[q as usize] as u64;
        ans_acc += (range_sum * sn) as u128;
        prev_k = k2;
    }

    let ans = (ans_acc % (MOD as u128)) as u64;

    println!("{}", ans);
}
