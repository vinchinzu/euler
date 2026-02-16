// Project Euler 782 - Distinct Rows and Columns
// C(n) = 3n^2 - 1 - N2 + N4 via bitarray sieve for achievability.
// Optimized with rayon parallelism, incremental inner loops, and bitset storage.

use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

fn main() {
    let n: i64 = 10000;
    let big_n = n * n;
    let big_n_u = big_n as usize;

    // Use atomic u64 bitset: 100M bits = ~1.56M words * 8 bytes = 12.5MB
    let nwords = (big_n_u + 64) / 64;
    let achievable: Vec<AtomicU64> = (0..nwords).map(|_| AtomicU64::new(0)).collect();

    #[inline(always)]
    fn bit_set(ach: &[AtomicU64], k: usize) {
        let word = k >> 6;
        let bit = 1u64 << (k & 63);
        unsafe { ach.get_unchecked(word).fetch_or(bit, Ordering::Relaxed); }
    }

    #[inline(always)]
    fn bit_get(ach: &[AtomicU64], k: usize) -> bool {
        let word = k >> 6;
        let bit = 1u64 << (k & 63);
        unsafe { (ach.get_unchecked(word).load(Ordering::Relaxed) & bit) != 0 }
    }

    bit_set(&achievable, 0);
    bit_set(&achievable, big_n_u);

    // S2: comp=2 values from 2x2 block matrices
    let mut is_s2 = vec![false; big_n_u + 1];

    for c in 1..n {
        let v = (c * c) as usize;
        if v > 0 && v < big_n_u { is_s2[v] = true; }
        let w = big_n_u - v;
        if w > 0 && w < big_n_u { is_s2[w] = true; }
    }
    for x in 1..n {
        let y = n - x;
        let v1 = (x * x + y * y) as usize;
        let v2 = (2 * x * y) as usize;
        if v1 > 0 && v1 < big_n_u { is_s2[v1] = true; }
        if v2 > 0 && v2 < big_n_u { is_s2[v2] = true; }
    }

    let mut n2: i64 = 0;
    for k in 1..big_n_u {
        if is_s2[k] {
            n2 += 1;
            bit_set(&achievable, k);
        }
    }
    drop(is_s2);

    // Construction 1: Products d*m with 1 <= d,m <= n-1
    for d in 1..n as usize {
        let mut k = d;
        let limit = d * n as usize;
        while k < limit {
            bit_set(&achievable, k);
            k += d;
        }
    }

    // Construction 2: Complement symmetry
    let mut comp_to_set = Vec::new();
    for k in 1..big_n_u {
        let comp = big_n_u - k;
        if k <= comp {
            let ak = bit_get(&achievable, k);
            let ac = bit_get(&achievable, comp);
            if ak && !ac {
                comp_to_set.push(comp);
            } else if ac && !ak {
                comp_to_set.push(k);
            }
        }
    }
    for k in comp_to_set {
        bit_set(&achievable, k);
    }

    // Construction 3: Kernel 3x3 matrices
    let rows_3: [[i64; 3]; 8] = {
        let mut r = [[0i64; 3]; 8];
        for i in 0..8 {
            r[i][0] = (i >> 2) as i64 & 1;
            r[i][1] = (i >> 1) as i64 & 1;
            r[i][2] = i as i64 & 1;
        }
        r
    };

    let mut forms: Vec<[i64; 6]> = Vec::new();

    for r0i in 0..8usize {
        for r1i in 0..8usize {
            for r2i in 0..8usize {
                let m = [rows_3[r0i], rows_3[r1i], rows_3[r2i]];

                let mut ok = true;
                for j in 0..3 {
                    let col = [m[0][j], m[1][j], m[2][j]];
                    let mut found = false;
                    for ri in 0..3 {
                        if m[ri] == col { found = true; break; }
                    }
                    if !found { ok = false; break; }
                }
                if !ok { continue; }

                let (a_coeff, b_coeff, c_coeff) = (m[0][0], m[1][1], m[2][2]);
                let d01 = m[0][1] + m[1][0];
                let d02 = m[0][2] + m[2][0];
                let d12 = m[1][2] + m[2][1];
                let aa = a_coeff + c_coeff - d02;
                let bb = b_coeff + c_coeff - d12;
                let ab = d01 + 2 * c_coeff - d02 - d12;
                let a1 = n * (d02 - 2 * c_coeff);
                let b1 = n * (d12 - 2 * c_coeff);
                let c0 = c_coeff * n * n;

                let form = [aa, bb, ab, a1, b1, c0];
                if !forms.contains(&form) {
                    forms.push(form);
                }
            }
        }
    }

    let ach_ref: &[AtomicU64] = &achievable;

    // Parallelize across forms (66 forms, rayon work-stealing handles imbalance)
    forms.par_iter().for_each(|f| {
        let aa = f[0];
        let bb = f[1];
        let ab = f[2];
        let a1 = f[3];
        let b1 = f[4];
        let c0 = f[5];
        let two_bb = 2 * bb;

        for a in 0..=n {
            let b_max = n - a;
            let base = aa * a * a + a1 * a + c0;
            let lin = ab * a + b1;
            let mut k = base;
            let mut delta = bb + lin;

            if k > 0 && k < big_n {
                bit_set(ach_ref, k as usize);
            }

            for _b in 1..=b_max {
                k += delta;
                delta += two_bb;
                if k > 0 && k < big_n {
                    bit_set(ach_ref, k as usize);
                }
            }
        }
    });

    std::sync::atomic::fence(Ordering::SeqCst);

    // Count N4: non-achievable values in [1, big_n-1]
    let mut n4: i64 = 0;
    for w in 0..nwords {
        let bits = achievable[w].load(Ordering::Relaxed);
        let base = w * 64;
        if base + 64 <= big_n_u {
            if w == 0 {
                let mask = !1u64;
                n4 += (mask & !bits).count_ones() as i64;
            } else {
                n4 += (!bits).count_ones() as i64;
            }
        } else {
            for bit in 0..64 {
                let k = base + bit;
                if k >= 1 && k < big_n_u {
                    if bits & (1u64 << bit) == 0 {
                        n4 += 1;
                    }
                }
            }
        }
    }

    println!("{}", 3 * big_n - 1 - n2 + n4);
}
