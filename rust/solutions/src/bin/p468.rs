// Project Euler 468 - Smooth divisors of binomial coefficients
// Segment tree approach for range multiplication.
//
// Key optimization: M = 1_000_000_993 < 2^30, so M*M < 10^18 < i64::MAX (9.2e18).
// Also 2*M*M < i64::MAX, so segment tree sums of two products fit in i64.
// This eliminates all i128 arithmetic.
//
// Memory optimization: mod_invs stored as u32 since all values < M < 2^30,
// saving 44MB compared to i64.
//
// Small-B: fused in-place prime updates (const-generic `% p`) + one deferred
// sum per prime, scaled by the composite gap. Avoids 23 full arrays of `% M`.
// Large primes p^2 > N+1 have valuation 0 or 1, so the inner while is skipped.

const N: usize = 11_111_111;
const M: i64 = 1_000_000_993;

/// Multiply s[r] by p^{v_p(C(N,r))} for r=1..half via a running product.
/// `B` is a compile-time prime so `% B` folds to a reciprocal / bitmask.
#[inline(always)]
fn apply_prime<const B: usize>(s: &mut [i64], inv_b: i64) {
    let bb = B as i64;
    let half = s.len() - 1;
    let mut prod = 1i64;
    for r in 1..=half {
        let mut nn = N + 1 - r;
        while nn % B == 0 {
            prod = prod * bb % M;
            nn /= B;
        }
        nn = r;
        while nn % B == 0 {
            prod = prod * inv_b % M;
            nn /= B;
        }
        // SAFETY: r <= half, s.len() == half+1
        unsafe {
            let sr = s.get_unchecked_mut(r);
            *sr = *sr * prod % M;
        }
    }
}

fn apply_prime_dyn(b: usize, s: &mut [i64], inv_b: i64) {
    let bb = b as i64;
    let half = s.len() - 1;
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
        // SAFETY: r <= half, s.len() == half+1
        unsafe {
            let sr = s.get_unchecked_mut(r);
            *sr = *sr * prod % M;
        }
    }
}

fn main() {
    // Sieve (u8, not Vec<bool> — bit-packed indexing is slower in the large-B scan)
    let mut is_prime = vec![0u8; N + 1];
    for i in 2..=N {
        is_prime[i] = 1;
    }
    {
        let mut i = 2;
        while i * i <= N {
            if is_prime[i] != 0 {
                let mut j = i * i;
                while j <= N {
                    is_prime[j] = 0;
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

    let half = N / 2;
    let mut s = vec![1i64; half + 1];

    let small_primes: Vec<usize> = (2..l_val).filter(|&p| is_prime[p] != 0).collect();

    // B=1: s[r]=1 for all r
    let mut ans = (2 * (half as i64 + 1)) % M;

    for (pi, &b) in small_primes.iter().enumerate() {
        let inv_b = unsafe { *mod_invs.get_unchecked(b) } as i64;
        match b {
            2 => apply_prime::<2>(&mut s, inv_b),
            3 => apply_prime::<3>(&mut s, inv_b),
            5 => apply_prime::<5>(&mut s, inv_b),
            7 => apply_prime::<7>(&mut s, inv_b),
            11 => apply_prime::<11>(&mut s, inv_b),
            13 => apply_prime::<13>(&mut s, inv_b),
            17 => apply_prime::<17>(&mut s, inv_b),
            19 => apply_prime::<19>(&mut s, inv_b),
            23 => apply_prime::<23>(&mut s, inv_b),
            _ => apply_prime_dyn(b, &mut s, inv_b),
        }
        let mut sum = 0i64;
        for r in 0..=half {
            // SAFETY: r <= half, s.len() == half+1
            sum += unsafe { *s.get_unchecked(r) };
        }
        let next = if pi + 1 < small_primes.len() {
            small_primes[pi + 1]
        } else {
            l_val
        };
        // gap <= 4, sum <= (half+1)*M < 6e15, 2*gap*sum fits i64
        ans = (ans + 2 * (next - b) as i64 * (sum % M)) % M;
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

    // multiply_range: suffix-multiply the segment tree starting at `start`.
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
            // Collapse even indices: equivalent to `while i % 2 == 0 { i /= 2 }`
            i >>= i.trailing_zeros();
            loop {
                *mults.get_unchecked_mut(i) = *mults.get_unchecked(i) * mult % M;
                while i & 1 != 0 {
                    i >>= 1;
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

    // Large B >= L. Composites do not change the tree; charge them in prime gaps.
    // For p^2 > N+1 the p-valuation of N+1-r and r is 0 or 1, so skip the inner while.
    let mut last = l_val;
    for b in l_val..=N {
        // SAFETY: b <= N, is_prime has length N + 1
        if unsafe { *is_prime.get_unchecked(b) } != 0 {
            let gap = (b - last) as i64;
            if gap != 0 {
                ans = (ans + 2 * gap * sums[1]) % M;
            }
            let bb = b as i64;
            let inv_b = unsafe { *mod_invs.get_unchecked(b) } as i64;
            let b2 = b as u64 * b as u64;
            if b2 > N as u64 + 1 {
                let mut r = N % b + 1;
                while r <= half {
                    unsafe {
                        multiply_range(r, bb, half_l2, &mut mults, &mut sums);
                    }
                    r += b;
                }
                r = b;
                while r <= half {
                    unsafe {
                        multiply_range(r, inv_b, half_l2, &mut mults, &mut sums);
                    }
                    r += b;
                }
            } else {
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
            last = b + 1;
        }
    }
    if last <= N {
        let gap = (N - last + 1) as i64;
        ans = (ans + 2 * gap * sums[1]) % M;
    }

    println!("{}", ans);
}
