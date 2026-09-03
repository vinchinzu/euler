// Project Euler 837 - Amidakuji
// Optimized: elimination of 1GB array, in-place Montgomery batch inversion
// in L1 cache stack buffers, 8-way ILP parallel range products for factorials,
// and chunked parallelization of the main summation loop using Rayon.

use rayon::prelude::*;

const A: u32 = 123456789;
const B: u32 = 987654321;
const MOD: u32 = 1234567891;

#[inline(always)]
fn mulmod(a: u32, b: u32) -> u32 {
    ((a as u64 * b as u64) % (MOD as u64)) as u32
}

fn mod_pow(mut base: u32, mut exp: u32) -> u32 {
    let mut result = 1u32;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mulmod(result, base);
        }
        base = mulmod(base, base);
        exp >>= 1;
    }
    result
}

/// Compute range product (lo..=hi) mod MOD using 8 parallel accumulators for instruction-level parallelism.
#[inline(always)]
fn range_product_8stream(lo: u32, hi: u32) -> u32 {
    if lo > hi {
        return 1;
    }
    let mut p0 = 1u64;
    let mut p1 = 1u64;
    let mut p2 = 1u64;
    let mut p3 = 1u64;
    let mut p4 = 1u64;
    let mut p5 = 1u64;
    let mut p6 = 1u64;
    let mut p7 = 1u64;
    let m = MOD as u64;
    let mut i = lo as u64;
    let hi64 = hi as u64;
    while i + 7 <= hi64 {
        p0 = (p0 * i) % m;
        p1 = (p1 * (i + 1)) % m;
        p2 = (p2 * (i + 2)) % m;
        p3 = (p3 * (i + 3)) % m;
        p4 = (p4 * (i + 4)) % m;
        p5 = (p5 * (i + 5)) % m;
        p6 = (p6 * (i + 6)) % m;
        p7 = (p7 * (i + 7)) % m;
        i += 8;
    }
    while i <= hi64 {
        p0 = (p0 * i) % m;
        i += 1;
    }
    let p01 = mulmod(p0 as u32, p1 as u32);
    let p23 = mulmod(p2 as u32, p3 as u32);
    let p45 = mulmod(p4 as u32, p5 as u32);
    let p67 = mulmod(p6 as u32, p7 as u32);
    mulmod(mulmod(p01, p23), mulmod(p45, p67))
}

struct ChunkTask {
    start: u32,
    end: u32,
    group: u8,
}

#[inline(always)]
fn get_initial_term2(k_start: u32) -> u32 {
    if k_start == 0 {
        return 0;
    }
    const TWO_THIRDS: u32 = 411522631; // 2 * inv(3) mod MOD
    let p = mod_pow(4, k_start);
    let p_minus_1 = if p == 0 { MOD - 1 } else { p - 1 };
    mulmod(TWO_THIRDS, p_minus_1)
}

fn main() {
    let half_a = (A - 1) / 2;
    let half_b = (B - 1) / 2;
    let half_sum = (A + B) / 2;

    // Build unified tasks for all range products (~1M elements per task)
    let mut tasks = Vec::with_capacity(512);
    let mut add_tasks = |lo: u32, hi: u32, n_chunks: usize, group: u8| {
        let total = (hi - lo + 1) as usize;
        let chunk_size = (total + n_chunks - 1) / n_chunks;
        for ci in 0..n_chunks {
            let start = lo + (ci * chunk_size) as u32;
            if start <= hi {
                let end = std::cmp::min(start + chunk_size as u32 - 1, hi);
                tasks.push(ChunkTask { start, end, group });
            }
        }
    };
    add_tasks(1, half_a, 64, 0);
    add_tasks(half_a + 1, half_b, 384, 1);
    add_tasks(half_b + 1, half_sum, 64, 2);

    let partial_prods: Vec<(u8, u32)> = tasks
        .into_par_iter()
        .map(|task| (task.group, range_product_8stream(task.start, task.end)))
        .collect();

    let mut fact_a = 1u32;
    let mut prod_a_to_b = 1u32;
    let mut prod_b_to_sum = 1u32;
    for (group, prod) in partial_prods {
        match group {
            0 => fact_a = mulmod(fact_a, prod),
            1 => prod_a_to_b = mulmod(prod_a_to_b, prod),
            2 => prod_b_to_sum = mulmod(prod_b_to_sum, prod),
            _ => unreachable!(),
        }
    }

    let fact_b = mulmod(fact_a, prod_a_to_b);
    let fact_total = mulmod(fact_b, prod_b_to_sum);

    let total_pairs = half_a; // 61728394 pairs
    let num_chunks = 64usize;
    let pairs_per_chunk = (total_pairs as usize + num_chunks - 1) / num_chunks;

    let chunk_results: Vec<(u32, u32)> = (0..num_chunks)
        .into_par_iter()
        .map(|c| {
            let k_start = (c * pairs_per_chunk) as u32;
            let k_end = std::cmp::min(k_start + pairs_per_chunk as u32, total_pairs);
            if k_start >= k_end {
                return (0, 1);
            }

            let mut running_term2 = get_initial_term2(k_start);
            let mut running_term1 = 1u32;
            let mut chunk_ans = 0u64;

            const SUB_BLOCK: usize = 2048;
            let mut d_arr = [0u32; SUB_BLOCK];
            let mut prefix = [0u32; SUB_BLOCK];

            let mut b_start = k_start;
            while b_start < k_end {
                let b_end = std::cmp::min(b_start + SUB_BLOCK as u32, k_end);
                let n_pairs = (b_end - b_start) as usize;

                // Pass 1: compute d_arr with unrolled loop
                let mut j = 0;
                let mut cur_s = 2 * b_start + 2;
                while j + 3 < n_pairs {
                    d_arr[j] = mulmod(cur_s, cur_s + 1);
                    d_arr[j + 1] = mulmod(cur_s + 2, cur_s + 3);
                    d_arr[j + 2] = mulmod(cur_s + 4, cur_s + 5);
                    d_arr[j + 3] = mulmod(cur_s + 6, cur_s + 7);
                    cur_s += 8;
                    j += 4;
                }
                while j < n_pairs {
                    d_arr[j] = mulmod(cur_s, cur_s + 1);
                    cur_s += 2;
                    j += 1;
                }

                // Pass 2: compute prefix products
                prefix[0] = d_arr[0];
                for j in 1..n_pairs {
                    prefix[j] = mulmod(prefix[j - 1], d_arr[j]);
                }

                // Montgomery batch invert and fold numerator into d_arr
                let base_na = (A - 1) / 2 - b_start;
                let base_nb = (B - 1) / 2 - b_start;

                let mut inv_prod = mod_pow(prefix[n_pairs - 1], MOD - 2);
                for j in (1..n_pairs).rev() {
                    let inv_j = mulmod(inv_prod, prefix[j - 1]);
                    inv_prod = mulmod(inv_prod, d_arr[j]);
                    let num = mulmod(base_na - j as u32, base_nb - j as u32);
                    d_arr[j] = mulmod(inv_j, num);
                }
                let num0 = mulmod(base_na, base_nb);
                d_arr[0] = mulmod(inv_prod, num0);

                // Pass 3: Ultra-tight accumulation loop
                for j in 0..n_pairs {
                    running_term1 = mulmod(running_term1, d_arr[j]);

                    let mut t2 = 4 * running_term2 as u64 + 2;
                    if t2 >= 2 * MOD as u64 {
                        t2 -= 2 * MOD as u64;
                    }
                    if t2 >= MOD as u64 {
                        t2 -= MOD as u64;
                    }
                    running_term2 = t2 as u32;

                    chunk_ans += mulmod(running_term1, running_term2) as u64;
                }

                b_start = b_end;
            }

            let chunk_sum = (chunk_ans % (MOD as u64)) as u32;
            let chunk_prod = running_term1;
            (chunk_sum, chunk_prod)
        })
        .collect();

    let mut cur_term1 = mod_pow(mulmod(fact_a, fact_b), MOD - 2);
    let mut total_ans = 0u64;
    for (chunk_sum, chunk_prod) in chunk_results {
        total_ans = (total_ans + mulmod(cur_term1, chunk_sum) as u64) % (MOD as u64);
        cur_term1 = mulmod(cur_term1, chunk_prod);
    }
    let ans = mulmod((total_ans % (MOD as u64)) as u32, fact_total);
    println!("{}", ans);
}
