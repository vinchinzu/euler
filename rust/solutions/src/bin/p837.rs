// Project Euler 837 - Amidakuji
// Optimized: u64 arithmetic (MOD < 2^31, so a*b fits in u64)
// + parallel factorial and inverse computation using rayon

use rayon::prelude::*;

const A: u64 = 123456789;
const B: u64 = 987654321;
const MOD: u64 = 1234567891;

#[inline(always)]
fn mulmod(a: u64, b: u64) -> u64 {
    a * b % MOD
}

fn mod_pow(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 { result = mulmod(result, base); }
        base = mulmod(base, base);
        exp >>= 1;
    }
    result
}

/// Compute product of (lo..=hi) mod MOD using parallel chunking
fn parallel_product_range(lo: u64, hi: u64) -> u64 {
    if lo > hi { return 1; }
    let chunk_size = 1_000_000u64;
    let n_chunks = ((hi - lo) / chunk_size + 1) as usize;
    let products: Vec<u64> = (0..n_chunks).into_par_iter().map(|ci| {
        let start = lo + ci as u64 * chunk_size;
        let end = std::cmp::min(start + chunk_size - 1, hi);
        let mut prod = 1u64;
        for i in start..=end {
            prod = mulmod(prod, i);
        }
        prod
    }).collect();
    let mut result = 1u64;
    for p in products {
        result = mulmod(result, p);
    }
    result
}

fn main() {
    let half_a = (A - 1) / 2; // 61728394
    let half_b = (B - 1) / 2; // 493827160
    let half_sum = (A + B) / 2; // 555555555
    let a_usize = A as usize;

    // Step 1: Forward pass - compute fact[0..A]
    let mut arr = vec![0u64; a_usize + 1];
    arr[0] = 1;
    arr[1] = 1;
    for i in 2..=a_usize {
        unsafe {
            *arr.get_unchecked_mut(i) = mulmod(*arr.get_unchecked(i - 1), i as u64);
        }
    }
    let fact_a = arr[half_a as usize];

    // Step 2: Compute extended factorials in parallel
    let (prod_a_to_b, prod_b_to_sum) = rayon::join(
        || parallel_product_range(half_a + 1, half_b),
        || parallel_product_range(half_b + 1, half_sum),
    );
    let fact_b = mulmod(fact_a, prod_a_to_b);
    let fact_total = mulmod(fact_b, prod_b_to_sum);

    // Step 3: Parallel backward pass to compute inverses
    // Split [2..A] into chunks. For each chunk endpoint, compute inv_fact via mod_pow.
    // Then fill each chunk independently.
    let n_chunks = 16usize; // use 16 chunks for good parallelism on 8 cores
    let chunk_len = (a_usize - 1) / n_chunks; // elements per chunk

    // Create chunk boundaries
    let mut boundaries = Vec::with_capacity(n_chunks + 1);
    for c in 0..n_chunks {
        boundaries.push(2 + c * chunk_len);
    }
    boundaries.push(a_usize + 1); // exclusive end

    // Compute inv_fact at each boundary start using mod_pow in parallel
    // inv_fact(i) = mod_pow(fact(i), MOD-2)
    // But we only need inv_fact at the END of each chunk (= start of next chunk - 1)
    // Actually we need inv_fact at the START of each chunk's backward walk,
    // which is the last element of the chunk.
    let chunk_inv_facts: Vec<u64> = (0..n_chunks).into_par_iter().map(|c| {
        let end = boundaries[c + 1] - 1; // last index in this chunk (inclusive)
        mod_pow(arr[end], MOD - 2)
    }).collect();

    // Save boundary factorial values (arr[start-1]) before overwriting arr.
    // Each chunk C reads arr[boundaries[C]-1] which may be in chunk C-1's range.
    let boundary_facts: Vec<u64> = (0..n_chunks).map(|c| {
        if boundaries[c] >= 2 { arr[boundaries[c] - 1] } else { 1 }
    }).collect();

    // Fill each chunk in parallel: walk backward, compute inv(i) = inv_fact(i) * fact(i-1)
    let arr_addr = arr.as_mut_ptr() as usize;
    // SAFETY: Each chunk writes to non-overlapping ranges of arr[boundaries[c]..boundaries[c+1]-1].
    // Each chunk reads arr[i-1] for i in its range. The only cross-boundary read is arr[start-1]
    // which we've saved in boundary_facts. All other reads arr[i-1] where i > start are
    // within the chunk and haven't been overwritten yet (we walk backwards: write i, then read i-1).
    (0..n_chunks).into_par_iter().for_each(|c| {
        let ptr = arr_addr as *mut u64;
        let start = boundaries[c];
        let end = boundaries[c + 1] - 1;
        let mut inv_fact = chunk_inv_facts[c];
        // Process i from end down to start+1 (these read arr[i-1] which is within chunk)
        for i in ((start + 1)..=end).rev() {
            unsafe {
                let fact_im1 = *ptr.add(i - 1);
                let inv_i = mulmod(inv_fact, fact_im1);
                inv_fact = mulmod(inv_fact, i as u64);
                *ptr.add(i) = inv_i;
            }
        }
        // Handle i = start separately using saved boundary factorial
        unsafe {
            let fact_im1 = boundary_facts[c];
            let inv_i = mulmod(inv_fact, fact_im1);
            *ptr.add(start) = inv_i;
        }
    });
    arr[1] = 1;

    // Main loop
    let mut term1 = mod_pow(mulmod(fact_a, fact_b), MOD - 2);
    let mut term2 = 0u64;
    let mut ans = 0u64;

    let mut t = 3u64;
    while t <= A {
        unsafe {
            term1 = mulmod(term1, *arr.get_unchecked((t - 1) as usize));
            term1 = mulmod(term1, *arr.get_unchecked(t as usize));
        }
        term1 = mulmod(term1, (A - t + 2) / 2);
        term1 = mulmod(term1, (B - t + 2) / 2);

        term2 = (4 * term2 + 2) % MOD;
        ans = (ans + mulmod(term1, term2)) % MOD;

        t += 2;
    }

    ans = mulmod(ans, fact_total);
    println!("{}", ans);
}
