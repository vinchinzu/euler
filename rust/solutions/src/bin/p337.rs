// Project Euler 337 - Totient Stairstep Sequences
// DP with AVX2 Wide Segment Tree (branching factor B=16).
//
// Optimizations:
// - Fast linear sieve for phi using phi_i == 0 as prime test (eliminating is_composite, saving 20MB)
// - Parallel rayon unstable sort on packed u64 (phi << 32 | idx)
// - 7-layer Wide Segment Tree (B=16): reduces tree height from 24 (Fenwick) to 7 layers
// - Branchless AVX2 vectorization for suffix updates inside 16-element nodes
// - Branchless query with only 7 array lookups (layers 2..6 permanently resident in L1 cache)
// - Elimination of zero updates (DP=0 when phi(j) + 1 >= j, e.g. for primes)
// - Skipping groups and prefix elements where j <= left = max(START, phi(j) + 1)
// - Reusable stack buffer instead of Vec allocations for group updates

use rayon::prelude::*;
use std::arch::x86_64::*;

const TARGET_N: usize = 20_000_000;
const MOD: i64 = 100_000_000;
const START: usize = 6;
const B: usize = 16;
const SHIFT: usize = 4;
const H_MAX: usize = 7;

struct WideSegTree {
    offsets: [usize; H_MAX],
    data: Vec<i32>,
    masks: [__m256i; 16],
}

impl WideSegTree {
    fn new(n: usize) -> Self {
        let mut offsets = [0usize; H_MAX];
        let mut cur_offset = 0;
        let mut cur_n = n;
        for h in 0..H_MAX {
            cur_offset = (cur_offset + 15) & !15;
            offsets[h] = cur_offset;
            let num_blocks = (cur_n + B - 1) / B;
            cur_offset += num_blocks * B;
            cur_n = num_blocks;
        }
        let total_size = cur_offset + 32;

        let mut masks = [unsafe { _mm256_setzero_si256() }; 16];
        for pos in 0..16 {
            let mut m = [0i32; 8];
            let p_in_chunk = pos % 8;
            for i in 0..8 {
                if i > p_in_chunk {
                    m[i] = -1;
                }
            }
            unsafe {
                masks[pos] = _mm256_loadu_si256(m.as_ptr() as *const __m256i);
            }
        }

        WideSegTree {
            offsets,
            data: vec![0i32; total_size],
            masks,
        }
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn add(&mut self, mut k: usize, x: i32) {
        unsafe {
            let d_ptr = self.data.as_mut_ptr();
            let v_x = _mm256_set1_epi32(x);
            let v_mod_minus_1 = _mm256_set1_epi32(99_999_999);
            let v_mod = _mm256_set1_epi32(100_000_000);

            for h in 0..H_MAX {
                let pos = k & (B - 1);
                if pos < 15 {
                    let block_start = *self.offsets.get_unchecked(h) + (k & !(B - 1));
                    if pos >= 8 {
                        let p1 = d_ptr.add(block_start + 8) as *mut __m256i;
                        let cur1 = _mm256_loadu_si256(p1);
                        let mask1 = *self.masks.get_unchecked(pos);
                        let delta1 = _mm256_and_si256(v_x, mask1);
                        let sum1 = _mm256_add_epi32(cur1, delta1);
                        let cmp1 = _mm256_cmpgt_epi32(sum1, v_mod_minus_1);
                        let sub1 = _mm256_and_si256(cmp1, v_mod);
                        let res1 = _mm256_sub_epi32(sum1, sub1);
                        _mm256_storeu_si256(p1, res1);
                    } else {
                        let p0 = d_ptr.add(block_start) as *mut __m256i;
                        let p1 = d_ptr.add(block_start + 8) as *mut __m256i;
                        let cur0 = _mm256_loadu_si256(p0);
                        let cur1 = _mm256_loadu_si256(p1);

                        let mask0 = *self.masks.get_unchecked(pos);
                        let delta0 = _mm256_and_si256(v_x, mask0);
                        let sum0 = _mm256_add_epi32(cur0, delta0);
                        let sum1 = _mm256_add_epi32(cur1, v_x);

                        let cmp0 = _mm256_cmpgt_epi32(sum0, v_mod_minus_1);
                        let cmp1 = _mm256_cmpgt_epi32(sum1, v_mod_minus_1);
                        let sub0 = _mm256_and_si256(cmp0, v_mod);
                        let sub1 = _mm256_and_si256(cmp1, v_mod);
                        let res0 = _mm256_sub_epi32(sum0, sub0);
                        let res1 = _mm256_sub_epi32(sum1, sub1);

                        _mm256_storeu_si256(p0, res0);
                        _mm256_storeu_si256(p1, res1);
                    }
                }
                k >>= SHIFT;
            }
        }
    }

    #[inline(always)]
    unsafe fn query(&self, mut k: usize) -> i64 {
        unsafe {
            let d_ptr = self.data.as_ptr();
            let mut s = 0i64;
            for h in 0..H_MAX {
                let idx = *self.offsets.get_unchecked(h) + k;
                s += *d_ptr.add(idx) as i64;
                k >>= SHIFT;
            }
            s % MOD
        }
    }
}

fn main() {
    let limit = TARGET_N + 1;
    let mut phi = vec![0u32; limit];
    let mut primes: Vec<u32> = Vec::with_capacity(1_300_000);

    unsafe {
        let phi_p = phi.as_mut_ptr();
        *phi_p.add(1) = 1;
        primes.push(2);
        *phi_p.add(2) = 1;
        if limit > 4 {
            *phi_p.add(4) = 2;
        }

        for i in 3..limit {
            if i & 1 == 0 {
                let phi_i = *phi_p.add(i);
                let i2 = i << 1;
                if i2 < limit {
                    *phi_p.add(i2) = phi_i << 1;
                }
                continue;
            }

            let mut phi_i = *phi_p.add(i);
            if phi_i == 0 {
                primes.push(i as u32);
                phi_i = i as u32 - 1;
                *phi_p.add(i) = phi_i;
            }

            let i2 = i << 1;
            if i2 < limit {
                *phi_p.add(i2) = phi_i;
            }

            let pr = primes.as_ptr();
            let np = primes.len();
            let mut j = 1;
            while j < np {
                let p = *pr.add(j);
                let ip = i as u64 * p as u64;
                if ip >= limit as u64 {
                    break;
                }
                if (i as u32) % p == 0 {
                    *phi_p.add(ip as usize) = phi_i * p;
                    break;
                }
                *phi_p.add(ip as usize) = phi_i * (p - 1);
                j += 1;
            }
        }
    }

    let count = TARGET_N - START + 1;
    let mut pairs: Vec<u64> = Vec::with_capacity(count);
    for i in START..=TARGET_N {
        unsafe {
            let p = *phi.get_unchecked(i) as u64;
            pairs.push((p << 32) | (i as u64));
        }
    }
    drop(phi);
    pairs.par_sort_unstable();

    let mut tree = WideSegTree::new(TARGET_N + 2);

    let mut buf_j = [0usize; 8192];
    let mut buf_val = [0i32; 8192];

    let mut total = 0i64;
    let mut pos = 0;

    unsafe {
        while pos < count {
            let cur_phi = (pairs[pos] >> 32) as u32;
            let left = START.max(cur_phi as usize + 1);

            let group_start = pos;
            while pos < count && ((pairs[pos] >> 32) as u32) == cur_phi {
                pos += 1;
            }
            let group_end = pos;

            if cur_phi == 2 {
                total = 1;
                tree.add(START, 1);
                continue;
            }

            let max_j = (pairs[group_end - 1] as u32) as usize;
            if max_j <= left {
                continue;
            }

            let mut active_start = group_start;
            let min_j = (pairs[group_start] as u32) as usize;
            if min_j <= left {
                while active_start < group_end && ((pairs[active_start] as u32) as usize) <= left {
                    active_start += 1;
                }
            }

            if active_start == group_end {
                continue;
            }

            // query(k) returns prefix sum of elements < k
            // We want sum of elements in [left, right] = query(right + 1) - query(left) = query(j) - query(left)
            let base_sum = tree.query(left);
            let mut g_len = 0;
            for k in active_start..group_end {
                let j = (pairs[k] as u32) as usize;
                let diff = tree.query(j) - base_sum;
                let value = if diff < 0 { diff + MOD } else { diff };
                let val_i32 = value as i32;
                buf_j[g_len] = j;
                buf_val[g_len] = val_i32;
                g_len += 1;

                total += value;
                if total >= MOD {
                    total -= MOD;
                }
            }

            for k in 0..g_len {
                let val = buf_val[k];
                if val != 0 {
                    tree.add(buf_j[k], val);
                }
            }
        }
    }

    println!("{}", total);
}
