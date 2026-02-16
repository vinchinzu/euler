// Project Euler 468 - Smooth divisors of binomial coefficients
// Segment tree approach for range multiplication.
//
// Key optimization: M = 1_000_000_993 < 2^30, so M*M < 10^18 < i64::MAX (9.2e18).
// Also 2*M*M < i64::MAX, so segment tree sums of two products fit in i64.
// This eliminates all i128 arithmetic.
//
// Memory optimization: mod_invs stored as u32 since all values < M < 2^30,
// saving 44MB compared to i64.

const N: usize = 11_111_111;
const M: i64 = 1_000_000_993;

fn main() {
    // Sieve
    let mut is_prime = vec![false; N + 1];
    for i in 2..=N {
        is_prime[i] = true;
    }
    {
        let mut i = 2;
        while i * i <= N {
            if is_prime[i] {
                let mut j = i * i;
                while j <= N {
                    is_prime[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
    }

    // Modular inverses (stored as u32 to save memory)
    let mut mod_invs = vec![0u32; N + 1];
    mod_invs[1] = 1;
    for i in 2..=N {
        mod_invs[i] = (M - (M / i as i64) * mod_invs[(M % i as i64) as usize] as i64 % M) as u32;
    }

    // L = ilog2(N)
    let mut l_val = 0;
    {
        let mut t = N;
        while t > 0 {
            l_val += 1;
            t >>= 1;
        }
    }

    // L2 = smallest power of 2 >= N/2 + 1, doubled
    let mut l2 = 1usize;
    while l2 < N / 2 + 1 {
        l2 *= 2;
    }
    l2 *= 2;
    let half_l2 = l2 / 2;

    // S[r] for r=0..N/2
    let half = N / 2;
    let mut s = vec![1i64; half + 1];

    let mut ans: i64 = 0;

    // Small B < L
    for b in 1..l_val {
        if is_prime[b] {
            let bb = b as i64;
            let inv_b = mod_invs[b] as i64;
            let mut prod = 1i64;
            for r in 1..=half {
                let mut nn = N + 1 - r;
                while nn % b == 0 {
                    prod = prod * bb % M;
                    nn /= b;
                }
                nn = r;
                while nn % b == 0 {
                    prod = prod * inv_b % M;
                    nn /= b;
                }
                // SAFETY: r <= half, s has length half + 1
                unsafe {
                    let sr = s.get_unchecked_mut(r);
                    *sr = *sr * prod % M;
                }
            }
        }
        for r in 0..=half {
            // SAFETY: r <= half, s has length half + 1
            ans = (ans + 2 * unsafe { *s.get_unchecked(r) }) % M;
        }
    }

    // Segment tree
    let mut mults = vec![1i64; l2];
    let mut sums = vec![0i64; l2];
    for i in 0..=half {
        // SAFETY: half_l2 + i < l2 because l2 = 2*half_l2 and half < half_l2
        unsafe {
            *sums.get_unchecked_mut(half_l2 + i) = *s.get_unchecked(i);
        }
    }
    // Free s since it's no longer needed
    drop(s);

    for i in (1..half_l2).rev() {
        // SAFETY: i < half_l2, so 2*i and 2*i+1 < l2
        unsafe {
            let val = (*mults.get_unchecked(2 * i) * *sums.get_unchecked(2 * i)
                + *mults.get_unchecked(2 * i + 1) * *sums.get_unchecked(2 * i + 1))
                % M;
            *sums.get_unchecked_mut(i) = val;
        }
    }

    // multiply_range: updates segment tree node at `start` with multiplier `mult`,
    // then propagates up the tree.
    // SAFETY: all indices accessed are within bounds of mults/sums (size l2).
    // start + half_l2 < l2, and tree traversal only goes to index 0.
    #[inline(always)]
    unsafe fn multiply_range(
        start: usize,
        mult: i64,
        half_l2: usize,
        mults: &mut [i64],
        sums: &mut [i64],
    ) {
        unsafe {
            let mut i = start + half_l2;
            while i % 2 == 0 {
                i /= 2;
            }
            loop {
                *mults.get_unchecked_mut(i) = *mults.get_unchecked(i) * mult % M;
                while i % 2 != 0 {
                    i /= 2;
                    *sums.get_unchecked_mut(i) =
                        (*mults.get_unchecked(2 * i) * *sums.get_unchecked(2 * i)
                            + *mults.get_unchecked(2 * i + 1)
                                * *sums.get_unchecked(2 * i + 1))
                            % M;
                    if i == 0 {
                        return;
                    }
                }
                i += 1;
            }
        }
    }

    // Large B >= L
    for b in l_val..=N {
        // SAFETY: b <= N, is_prime has length N + 1
        if unsafe { *is_prime.get_unchecked(b) } {
            let bb = b as i64;
            let inv_b = unsafe { *mod_invs.get_unchecked(b) } as i64;
            let mut r = N % b + 1;
            while r <= half {
                let mut nn = N + 1 - r;
                while nn % b == 0 {
                    unsafe {
                        multiply_range(r, bb, half_l2, &mut mults, &mut sums);
                    }
                    nn /= b;
                }
                r += b;
            }
            r = b;
            while r <= half {
                let mut nn = r;
                while nn % b == 0 {
                    unsafe {
                        multiply_range(r, inv_b, half_l2, &mut mults, &mut sums);
                    }
                    nn /= b;
                }
                r += b;
            }
        }
        ans = (ans + 2 * sums[1]) % M;
    }

    println!("{}", ans);
}
