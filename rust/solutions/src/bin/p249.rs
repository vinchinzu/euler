// Project Euler 249 - Prime Subset Sums
// Subsets of primes < 5000 whose sum is prime, count mod 10^16.

const N: usize = 5000;
const M: u64 = 10_000_000_000_000_000;

#[inline(always)]
fn add_mod(a: u64, b: u64) -> u64 {
    let s = a.wrapping_add(b);
    if s >= M { s - M } else { s }
}

/// 0-1 knapsack update: dp[i] += dp[i-p] (mod M) for i = current_sum .. p.
///
/// SAFETY: `dp` has length > current_sum, current_sum >= p, so i and i-p are in-bounds.
#[inline(always)]
unsafe fn knapsack_add(dp: *mut u64, p: usize, current_sum: usize) {
    #[cfg(target_arch = "x86_64")]
    {
        unsafe { knapsack_add_avx2(dp, p, current_sum) };
        return;
    }
    #[cfg(not(target_arch = "x86_64"))]
    unsafe {
        knapsack_add_scalar(dp, p, current_sum)
    };
}

#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
unsafe fn knapsack_add_scalar(dp: *mut u64, p: usize, current_sum: usize) {
    let mut i = current_sum;
    while i >= p {
        let s = unsafe { (*dp.add(i)).wrapping_add(*dp.add(i - p)) };
        unsafe { *dp.add(i) = if s >= M { s - M } else { s } };
        i -= 1;
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn knapsack_add_avx2(dp: *mut u64, p: usize, current_sum: usize) {
    use std::arch::x86_64::*;
    // 2M = 2e16 < 2^63, so signed 64-bit compares on the unreduced sums are valid.
    let vm = _mm256_set1_epi64x(M as i64);
    let vm1 = _mm256_set1_epi64x((M - 1) as i64);

    let mut i = current_sum;
    // Load both dest/source windows before any store so a window of 8 is a
    // pure old += old[i-p] update (correct 0-1 even when p < 8).
    while i >= p + 7 {
        unsafe {
            let d0 = _mm256_loadu_si256(dp.add(i - 7) as *const __m256i);
            let s0 = _mm256_loadu_si256(dp.add(i - 7 - p) as *const __m256i);
            let d1 = _mm256_loadu_si256(dp.add(i - 3) as *const __m256i);
            let s1 = _mm256_loadu_si256(dp.add(i - 3 - p) as *const __m256i);
            let sum0 = _mm256_add_epi64(d0, s0);
            let sum1 = _mm256_add_epi64(d1, s1);
            let ge0 = _mm256_cmpgt_epi64(sum0, vm1);
            let ge1 = _mm256_cmpgt_epi64(sum1, vm1);
            let r0 = _mm256_sub_epi64(sum0, _mm256_and_si256(ge0, vm));
            let r1 = _mm256_sub_epi64(sum1, _mm256_and_si256(ge1, vm));
            _mm256_storeu_si256(dp.add(i - 7) as *mut __m256i, r0);
            _mm256_storeu_si256(dp.add(i - 3) as *mut __m256i, r1);
        }
        i -= 8;
    }
    while i >= p {
        let s = unsafe { (*dp.add(i)).wrapping_add(*dp.add(i - p)) };
        unsafe { *dp.add(i) = if s >= M { s - M } else { s } };
        i -= 1;
    }
}

/// Odd-only bit sieve: bit k is set iff 2k+1 is prime. 2 is not represented.
fn sieve_odd_bits(limit: usize) -> Vec<u64> {
    let n_odd = limit / 2 + 1;
    let nwords = (n_odd + 63) / 64;
    let mut bits = vec![u64::MAX; nwords];
    bits[0] &= !1; // 1 is not prime

    let mut p = 3usize;
    while p * p <= limit {
        let k = p >> 1;
        if bits[k >> 6] & (1u64 << (k & 63)) != 0 {
            let mut j = (p * p) >> 1;
            while j < n_odd {
                bits[j >> 6] &= !(1u64 << (j & 63));
                j += p;
            }
        }
        p += 2;
    }
    bits
}

fn main() {
    let mut is_prime_n = [true; N + 1];
    is_prime_n[0] = false;
    is_prime_n[1] = false;
    let mut i = 2usize;
    while i * i <= N {
        if is_prime_n[i] {
            let mut j = i * i;
            while j <= N {
                is_prime_n[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let mut primes = Vec::with_capacity(700);
    let mut total_sum = 0usize;
    for p in 2..=N {
        if is_prime_n[p] {
            primes.push(p);
            total_sum += p;
        }
    }

    let mut dp = vec![0u64; total_sum + 1];
    dp[0] = 1;

    let ptr = dp.as_mut_ptr();
    let mut current_sum = 0usize;
    for &p in &primes {
        current_sum += p;
        // SAFETY: current_sum <= total_sum, so 0..=current_sum is in-bounds.
        unsafe { knapsack_add(ptr, p, current_sum) };
    }

    let odd_prime = sieve_odd_bits(total_sum);

    // dp[2] is the only even prime subset-sum (the singleton {2}).
    let mut ans = unsafe { *dp.get_unchecked(2) };
    let max_k = total_sum / 2;
    for k in 1..=max_k {
        let word = unsafe { *odd_prime.get_unchecked(k >> 6) };
        if word & (1u64 << (k & 63)) != 0 {
            let n = 2 * k + 1;
            if n > total_sum {
                break;
            }
            ans = add_mod(ans, unsafe { *dp.get_unchecked(n) });
        }
    }

    println!("{}", ans);
}
