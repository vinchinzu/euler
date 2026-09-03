// Project Euler 739 - Summation of Summations
//
// Optimized parallel implementation:
// - Backward Lucas number generation (zero memory allocation)
// - Factored hockey-stick identity for binomial difference terms
// - Montgomery simultaneous batch inversion in L1/L2 cache blocks
// - Parallel chunking across threads with Rayon

use rayon::prelude::*;

const N: usize = 100_000_000;
const M: u32 = 1_000_000_007;
const BLOCK_SIZE: usize = 32768;

#[inline(always)]
fn mul_mod(a: u32, b: u32) -> u32 {
    ((a as u64 * b as u64) % (M as u64)) as u32
}

#[inline(always)]
fn add_mod(a: u32, b: u32) -> u32 {
    let s = a + b;
    if s >= M {
        s - M
    } else {
        s
    }
}

#[inline(always)]
fn sub_mod(a: u32, b: u32) -> u32 {
    if a >= b {
        a - b
    } else {
        a + M - b
    }
}

fn mod_pow(mut base: u32, mut exp: u32) -> u32 {
    let mut res = 1u32;
    while exp > 0 {
        if exp & 1 == 1 {
            res = mul_mod(res, base);
        }
        base = mul_mod(base, base);
        exp >>= 1;
    }
    res
}

// Compute (L_{k+1}, L_k) for any k >= 1
// [L_{k+1}, L_k]^T = [[1, 1], [1, 0]]^(k-1) * [3, 1]^T
fn get_lucas_pair(k: usize) -> (u32, u32) {
    if k == 1 {
        return (3, 1);
    }
    let mut a = 1u32;
    let mut b = 1u32;
    let mut c = 1u32;
    let mut d = 0u32;

    let mut ra = 1u32;
    let mut rb = 0u32;
    let mut rc = 0u32;
    let mut rd = 1u32;

    let mut p = k - 1;
    while p > 0 {
        if p & 1 == 1 {
            let nra = add_mod(mul_mod(ra, a), mul_mod(rb, c));
            let nrb = add_mod(mul_mod(ra, b), mul_mod(rb, d));
            let nrc = add_mod(mul_mod(rc, a), mul_mod(rd, c));
            let nrd = add_mod(mul_mod(rc, b), mul_mod(rd, d));
            ra = nra;
            rb = nrb;
            rc = nrc;
            rd = nrd;
        }
        let na = add_mod(mul_mod(a, a), mul_mod(b, c));
        let nb = add_mod(mul_mod(a, b), mul_mod(b, d));
        let nc = add_mod(mul_mod(c, a), mul_mod(d, c));
        let nd = add_mod(mul_mod(c, b), mul_mod(d, d));
        a = na;
        b = nb;
        c = nc;
        d = nd;
        p >>= 1;
    }

    let l_k1 = add_mod(mul_mod(ra, 3), mul_mod(rb, 1));
    let l_k = add_mod(mul_mod(rc, 3), mul_mod(rd, 1));
    (l_k1, l_k)
}

fn process_chunk(start: usize, end: usize) -> (u32, u32) {
    let k_start = N - start;
    let (mut curr_l, mut prev_l) = get_lucas_pair(k_start);

    let mut rel_ncr = 1u32;
    let mut s_c = 0u32;
    let mut inv_buf = vec![0u32; BLOCK_SIZE];

    let mut j = start;
    while j < end {
        let block_len = (end - j).min(BLOCK_SIZE);

        inv_buf[0] = j as u32;
        for i in 1..block_len {
            inv_buf[i] = mul_mod(inv_buf[i - 1], (j + i) as u32);
        }

        let mut inv = mod_pow(inv_buf[block_len - 1], M - 2);

        for i in (1..block_len).rev() {
            let val = (j + i) as u32;
            let inv_val = mul_mod(inv, inv_buf[i - 1]);
            inv = mul_mod(inv, val);
            inv_buf[i] = inv_val;
        }
        inv_buf[0] = inv;

        for i in 0..block_len {
            let cur_j = j + i;
            let k = (N - cur_j) as u32;
            let inv_j = inv_buf[i];

            let mult = mul_mod((N - 2 + cur_j) as u32, inv_j);
            rel_ncr = mul_mod(rel_ncr, mult);

            let lk_k = mul_mod(prev_l, k - 1);
            let term = mul_mod(lk_k, rel_ncr);
            s_c = add_mod(s_c, term);

            let next_l = sub_mod(curr_l, prev_l);
            curr_l = prev_l;
            prev_l = next_l;
        }

        j += block_len;
    }

    (s_c, rel_ncr)
}

fn main() {
    let chunk_size = 1_000_000;
    let max_j = N - 2;

    let mut chunks = Vec::new();
    let mut j = 1;
    while j <= max_j {
        let end = (j + chunk_size).min(max_j + 1);
        chunks.push((j, end));
        j = end;
    }

    let chunk_results: Vec<(u32, u32)> = chunks
        .into_par_iter()
        .map(|(start, end)| process_chunk(start, end))
        .collect();

    let mut total_sum = 0u32;
    let mut base_ncr = 1u32;
    for (s_c, p_c) in chunk_results {
        total_sum = add_mod(total_sum, mul_mod(base_ncr, s_c));
        base_ncr = mul_mod(base_ncr, p_c);
    }

    let (l_n, _) = get_lucas_pair(N - 1);
    let inv_n_1 = mod_pow((N - 1) as u32, M - 2);
    let ans = add_mod(l_n, mul_mod(total_sum, inv_n_1));

    println!("{}", ans);
}
