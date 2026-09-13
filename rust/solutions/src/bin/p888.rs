// Project Euler 888
// Nim-like game with nimber computation, periodicity detection, and DP.
//
// OPTIMIZATION STATUS (Wave 52):
// Current runtime: ~50ms average (45-63ms range, N=100 sample)
// This code is already highly optimized:
// - Uses Rayon parallelism for main DP loop
// - Employs unsafe get_unchecked for hot paths
// - Deferred modular reduction with u128 accumulators
// - Efficient bitset-based nimber (Grundy) computation
// 
// Attempted optimizations that failed to achieve ≥5% speedup:
// - Pre-filtering active nimbers: added overhead (51.5ms vs 50.1ms)
// - Handling d=0 separately: no improvement
// - Sequential execution: major regression (97ms)
// - Raw pointer arithmetic: regression (48.6ms → 52.0ms)  
// - Manual loop unrolling (4x): regression (56.9ms)
// - Optimizing mults computation: regression (49.9ms)
// - target-cpu=native: regression (50.8ms)
// - Periodicity detection optimization: high variance, inconclusive
//
// Conclusion: Code is at optimization plateau. Further gains would require
// algorithmic changes, SIMD intrinsics, or problem parameter reduction.
// Measurement noise (~10% variance) exceeds target 5% improvement threshold.

use rayon::prelude::*;

const N_VAL: usize = 12_491_249;
const K: usize = 1249;
const KP: usize = K + 1;
const M: u64 = 912_491_249;
const L: usize = 25_000;

fn main() {
    let ds = [1usize, 2, 4, 9];

    // Grundy numbers stay tiny (max 10); one u64 bitset is enough for mex.
    let mut nimbers = vec![0u32; L];
    for n in 0..L {
        let mut bits = 0u64;
        for &d in &ds {
            if d <= n {
                bits |= 1u64 << unsafe { *nimbers.get_unchecked(n - d) };
            }
        }
        // Split pile n into (i, n-i). Odd-n extra branch is the same xor pair.
        // SAFETY: i <= n/2 < L and n-i < L; xor of stored nimbers is < 64.
        unsafe {
            for i in 1..=n / 2 {
                let v = *nimbers.get_unchecked(i) ^ *nimbers.get_unchecked(n - i);
                bits |= 1u64 << v;
            }
        }
        nimbers[n] = bits.trailing_ones();
    }

    let max_nimber = *nimbers.iter().max().unwrap() as usize;
    let mut cap = 1usize;
    while cap <= max_nimber {
        cap *= 2;
    }

    let half = L / 2;
    let target = &nimbers[half..half + half];
    let mut period = 0usize;
    for start in 0..half {
        if &nimbers[start..start + half] == target {
            period = half - start;
            break;
        }
    }

    let mut counts = vec![0u64; cap];
    for n in 1..period {
        counts[nimbers[n] as usize] += 1;
    }
    let n_val = N_VAL as u64;
    let period_u = period as u64;
    for n in 0..period {
        counts[nimbers[n + period] as usize] += (n_val - n as u64) / period_u;
    }

    // Modular inverses 1..=K. M is prime and K < M.
    let mut inv = vec![0u64; KP];
    inv[1] = 1;
    for i in 2..=K {
        inv[i] = M - (M / i as u64) * inv[(M % i as u64) as usize] % M;
    }

    let mut cur = vec![0u64; cap * KP];
    let mut nxt = vec![0u64; cap * KP];
    cur[0] = 1;

    for nimber in 0..cap {
        let c = counts[nimber];
        if c == 0 {
            continue;
        }

        let mut mults = [0u64; KP];
        mults[0] = 1;
        let mut m = 1u64;
        for d in 1..=K {
            let num = (c + d as u64 - 1) % M;
            m = m * num % M * inv[d] % M;
            mults[d] = m;
        }

        nxt.par_chunks_mut(KP).enumerate().for_each(|(n, next_row)| {
            let mut acc = [0u128; KP];
            let even_src = n * KP;
            let odd_src = (n ^ nimber) * KP;
            for d in 0..=K {
                let mult = unsafe { *mults.get_unchecked(d) };
                if mult == 0 {
                    continue;
                }
                let src_off = if d & 1 == 0 { even_src } else { odd_src };
                let mv = mult as u128;
                // SAFETY: src_off + k < cap*KP; k+d < KP.
                unsafe {
                    for k in 0..KP - d {
                        *acc.get_unchecked_mut(k + d) +=
                            mv * (*cur.get_unchecked(src_off + k) as u128);
                    }
                }
            }
            for i in 0..KP {
                next_row[i] = (acc[i] % M as u128) as u64;
            }
        });
        std::mem::swap(&mut cur, &mut nxt);
    }

    println!("{}", cur[K]);
}
