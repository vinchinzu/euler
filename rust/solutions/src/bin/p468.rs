// Project Euler 468 - Smooth divisors of binomial coefficients
// Segment tree approach for range multiplication.
//
// Key optimization: M = 1_000_000_993 < 2^30, so M*M < 10^18 < u64::MAX.
// Segment-tree values fit in u32 and products are widened to u64, eliminating
// i128 arithmetic while halving the tree's memory footprint.
//
// Memory optimization: mod_invs stored as u32 since all values < M < 2^30,
// saving 44MB compared to i64.
//
// Small-B: fused in-place prime updates (const-generic `% p`) + one deferred
// sum per prime, scaled by the composite gap. Avoids 23 full arrays of `% M`.
// Dense large-prime updates are batched so shared tree ancestors are rebuilt once.
// Large primes p^2 > N+1 have valuation 0 or 1, so the inner while is skipped.

const N: usize = 11_111_111;
const M: i64 = 1_000_000_993;

#[derive(Clone, Copy)]
struct Node {
    mult: u32,
    sum: u32,
}

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
    // Interleaved u32 nodes halve the footprint and keep each node's fields together.
    let mut tree = vec![Node { mult: 1, sum: 0 }; l2];
    for i in 0..=half {
        unsafe {
            tree.get_unchecked_mut(half_l2 + i).sum = *s.get_unchecked(i) as u32;
        }
    }
    drop(s);

    for i in (1..half_l2).rev() {
        unsafe {
            let left = *tree.get_unchecked(2 * i);
            let right = *tree.get_unchecked(2 * i + 1);
            let v0 = left.mult as u64 * left.sum as u64;
            let v1 = right.mult as u64 * right.sum as u64;
            tree.get_unchecked_mut(i).sum = ((v0 + v1) % M as u64) as u32;
        }
    }
    unsafe fn multiply_range(start: usize, mult: u32, half_l2: usize, tree: &mut [Node]) {
        unsafe {
            let mut i = start + half_l2;
            i >>= i.trailing_zeros();
            let mult_u64 = mult as u64;
            loop {
                let node = tree.get_unchecked_mut(i);
                node.mult = (node.mult as u64 * mult_u64 % M as u64) as u32;
                while i & 1 != 0 {
                    i >>= 1;
                    let left = *tree.get_unchecked(2 * i);
                    let right = *tree.get_unchecked(2 * i + 1);
                    let v0 = left.mult as u64 * left.sum as u64;
                    let v1 = right.mult as u64 * right.sum as u64;
                    tree.get_unchecked_mut(i).sum = ((v0 + v1) % M as u64) as u32;
                    if i == 0 {
                        return;
                    }
                }
                i += 1;
            }
        }
    }

    // For dense primes, apply every multiplier change in a single tree traversal.
    // Each event stores the multiplier in effect from its position onward, so
    // adjacent suffix updates share their ancestor recomputation.
    unsafe fn apply_events(
        node: usize,
        lo: usize,
        hi: usize,
        events: &[(u32, u32)],
        before: u32,
        tree: &mut [Node],
    ) {
        unsafe {
            let uniform = if events.is_empty() {
                Some(before)
            } else if events.len() == 1 && events[0].0 as usize == lo {
                Some(events[0].1)
            } else {
                None
            };
            if let Some(mult) = uniform {
                if mult != 1 {
                    let tree_node = tree.get_unchecked_mut(node);
                    tree_node.mult = (tree_node.mult as u64 * mult as u64 % M as u64) as u32;
                }
                return;
            }

            let mid = (lo + hi) / 2;
            let split = events.partition_point(|event| event.0 < mid as u32);
            let right_before = if split == 0 {
                before
            } else {
                events.get_unchecked(split - 1).1
            };
            apply_events(
                2 * node,
                lo,
                mid,
                events.get_unchecked(..split),
                before,
                tree,
            );
            apply_events(
                2 * node + 1,
                mid,
                hi,
                events.get_unchecked(split..),
                right_before,
                tree,
            );
            let left = 2 * node;
            let left_node = *tree.get_unchecked(left);
            let right_node = *tree.get_unchecked(left + 1);
            let v0 = left_node.mult as u64 * left_node.sum as u64;
            let v1 = right_node.mult as u64 * right_node.sum as u64;
            tree.get_unchecked_mut(node).sum = ((v0 + v1) % M as u64) as u32;
        }
    }

    // Large B >= L. Composites do not change the tree; charge them in prime gaps.
    // For p^2 > N+1 the p-valuation of N+1-r and r is 0 or 1, so skip the inner while.
    const BATCH_LIMIT: usize = 2_000;
    let mut last = l_val;
    let mut events = Vec::with_capacity(N / l_val + 1);
    for b in l_val..=N {
        // SAFETY: b <= N, is_prime has length N + 1
        if unsafe { *is_prime.get_unchecked(b) } != 0 {
            let gap = (b - last) as i64;
            if gap != 0 {
                ans = (ans + 2 * gap * (tree[1].sum as i64)) % M;
            }
            let bb = b as i64;
            let inv_b = unsafe { *mod_invs.get_unchecked(b) } as i64;
            let b2 = b as u64 * b as u64;
            if b <= BATCH_LIMIT {
                let mut numerator = N % b + 1;
                let mut denominator = b;
                let mut cumulative = 1u32;
                events.clear();
                while numerator <= half || denominator <= half {
                    let pos = numerator.min(denominator);
                    let mut change = 1u64;
                    if numerator == pos {
                        let mut factor = bb as u64;
                        if b2 <= N as u64 + 1 {
                            factor = 1;
                            let mut nn = N + 1 - numerator;
                            while nn % b == 0 {
                                factor = factor * bb as u64 % M as u64;
                                nn /= b;
                            }
                        }
                        change = change * factor % M as u64;
                        numerator += b;
                    }
                    if denominator == pos {
                        let mut factor = inv_b as u64;
                        if b2 <= N as u64 + 1 {
                            factor = 1;
                            let mut nn = denominator;
                            while nn % b == 0 {
                                factor = factor * inv_b as u64 % M as u64;
                                nn /= b;
                            }
                        }
                        change = change * factor % M as u64;
                        denominator += b;
                    }
                    if change != 1 {
                        cumulative = (cumulative as u64 * change % M as u64) as u32;
                        events.push((pos as u32, cumulative));
                    }
                }
                if !events.is_empty() {
                    unsafe {
                        apply_events(1, 0, half_l2, &events, 1, &mut tree);
                    }
                }
            } else if b2 > N as u64 + 1 {
                let mut r = N % b + 1;
                while r <= half {
                    unsafe {
                        multiply_range(r, bb as u32, half_l2, &mut tree);
                    }
                    r += b;
                }
                r = b;
                while r <= half {
                    unsafe {
                        multiply_range(r, inv_b as u32, half_l2, &mut tree);
                    }
                    r += b;
                }
            } else {
                let mut r = N % b + 1;
                while r <= half {
                    let mut nn = N + 1 - r;
                    while nn % b == 0 {
                        unsafe {
                            multiply_range(r, bb as u32, half_l2, &mut tree);
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
                            multiply_range(r, inv_b as u32, half_l2, &mut tree);
                        }
                        nn /= b;
                    }
                    r += b;
                }
            }
            ans = (ans + 2 * (tree[1].sum as i64)) % M;
            last = b + 1;
        }
    }
    if last <= N {
        let gap = (N - last + 1) as i64;
        ans = (ans + 2 * gap * (tree[1].sum as i64)) % M;
    }

    println!("{}", ans);
}
