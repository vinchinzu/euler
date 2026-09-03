// Project Euler 468 - Smooth divisors of binomial coefficients
//
// Key optimizations:
// 1. Parallel independent prime event ranges: Partition [0, N/2] into K chunks of r.
//    By Kummer's theorem and multiplicativity, each chunk k only tracks local suffix
//    events within its leaf range and maintains a single global_scale scalar for all
//    multipliers applying before lo_k. The chunks are completely decoupled and run
//    in parallel via Rayon across CPU cores.
// 2. Local Segment Trees: Each chunk maintains its own compact power-of-2 segment
//    tree fitting entirely in L2/L3 cache (4 MB vs 134 MB monolithic tree), reducing
//    tree height and eliminating memory bus contention.
// 3. Modulo & Valuation optimizations:
//    - For primes b > sqrt(N), v_b(C(N, lo_k)) reduces to a single comparison:
//      lo_k % b > N % b, avoiding expensive prime factorizations.
//    - For primes b > N/2, denominator multiples are absent, N % b = N - b, and
//      lo_k % b > N % b simplifies to b > N - lo_k (no division/modulo required).
//    - Modular inverses mod_invs are only needed up to N/2.
//    - Intermediate products widened to u64 with MOD < 2^30.

use rayon::prelude::*;

const N: usize = 11_111_111;
const M: u64 = 1_000_000_993;
const HALF: usize = N / 2;

fn vp_binom(n: usize, r: usize, p: usize) -> u32 {
    let mut sum_r = 0;
    let mut t = r;
    while t > 0 {
        sum_r += (t % p) as u32;
        t /= p;
    }
    let mut sum_nr = 0;
    t = n - r;
    while t > 0 {
        sum_nr += (t % p) as u32;
        t /= p;
    }
    let mut sum_n = 0;
    t = n;
    while t > 0 {
        sum_n += (t % p) as u32;
        t /= p;
    }
    (sum_r + sum_nr - sum_n) / (p as u32 - 1)
}

fn pow_mod(mut base: u64, mut exp: u32, m: u64) -> u64 {
    let mut res = 1;
    base %= m;
    while exp > 0 {
        if exp % 2 == 1 {
            res = res * base % m;
        }
        base = base * base % m;
        exp /= 2;
    }
    res
}

#[derive(Clone, Copy)]
struct Node {
    mult: u32,
    sum: u32,
}

#[inline(always)]
unsafe fn multiply_range(start: usize, mult: u32, half_l2: usize, tree: &mut [Node]) {
    unsafe {
        let mut i = start + half_l2;
        i >>= i.trailing_zeros();
        let mult_u64 = mult as u64;
        loop {
            let node = tree.get_unchecked_mut(i);
            node.mult = (node.mult as u64 * mult_u64 % M) as u32;
            while i & 1 != 0 {
                i >>= 1;
                let left = *tree.get_unchecked(2 * i);
                let right = *tree.get_unchecked(2 * i + 1);
                let v0 = left.mult as u64 * left.sum as u64;
                let v1 = right.mult as u64 * right.sum as u64;
                tree.get_unchecked_mut(i).sum = ((v0 + v1) % M) as u32;
                if i == 0 {
                    return;
                }
            }
            i += 1;
        }
    }
}

fn solve_chunk(k: usize, num_chunks: usize, is_prime: &[u8], mod_invs: &[u32], sqrt_n: usize) -> u64 {
    let chunk_size = (HALF + 1 + num_chunks - 1) / num_chunks;
    let lo = k * chunk_size;
    let hi = ((k + 1) * chunk_size).min(HALF + 1);
    if lo >= hi {
        return 0;
    }
    let len = hi - lo;

    let mut l2 = 1usize;
    while l2 < len {
        l2 *= 2;
    }
    l2 *= 2;
    let half_l2 = l2 / 2;

    let mut tree = vec![Node { mult: 1, sum: 0 }; l2];
    for i in 0..len {
        tree[half_l2 + i].sum = 1;
    }
    for i in (1..half_l2).rev() {
        let left = tree[2 * i];
        let right = tree[2 * i + 1];
        let v0 = left.mult as u64 * left.sum as u64;
        let v1 = right.mult as u64 * right.sum as u64;
        tree[i].sum = ((v0 + v1) % M) as u32;
    }

    let mut global_scale = 1u64;
    let mut ans_k = 0u64;
    let mut last = 1usize;

    // Primes b <= HALF
    for b in 2..=HALF {
        // SAFETY: b <= HALF < is_prime.len()
        if unsafe { *is_prime.get_unchecked(b) } != 0 {
            let gap = (b - last) as u64;
            if gap > 0 {
                let cur_sum = global_scale * tree[1].sum as u64 % M;
                ans_k = (ans_k + 2 * gap * cur_sum) % M;
            }
            let bb = b as u64;
            let inv_b = unsafe { *mod_invs.get_unchecked(b) } as u64;
            let n_mod_b = N % b;

            if lo > 0 {
                if b <= sqrt_n {
                    let v_base = vp_binom(N, lo, b);
                    if v_base > 0 {
                        let f_base = pow_mod(bb, v_base, M);
                        global_scale = global_scale * f_base % M;
                    }
                } else {
                    let lo_mod_b = lo % b;
                    if lo_mod_b > n_mod_b {
                        global_scale = global_scale * bb % M;
                    }
                }
            }

            let rem = if n_mod_b + 1 == b { b } else { n_mod_b + 1 };
            let first_r = if lo < rem {
                rem
            } else {
                rem + ((lo - rem) / b + 1) * b
            };

            if b <= sqrt_n {
                let mut r = first_r;
                while r < hi {
                    let mut nn = N + 1 - r;
                    let mut factor = 1u64;
                    while nn % b == 0 {
                        factor = factor * bb % M;
                        nn /= b;
                    }
                    unsafe {
                        multiply_range(r - lo, factor as u32, half_l2, &mut tree);
                    }
                    r += b;
                }

                let first_r_den = ((lo / b) + 1) * b;
                let mut r = first_r_den;
                while r < hi {
                    let mut nn = r;
                    let mut factor = 1u64;
                    while nn % b == 0 {
                        factor = factor * inv_b % M;
                        nn /= b;
                    }
                    unsafe {
                        multiply_range(r - lo, factor as u32, half_l2, &mut tree);
                    }
                    r += b;
                }
            } else {
                let mut r = first_r;
                while r < hi {
                    unsafe {
                        multiply_range(r - lo, bb as u32, half_l2, &mut tree);
                    }
                    r += b;
                }

                let first_r_den = ((lo / b) + 1) * b;
                let mut r = first_r_den;
                while r < hi {
                    unsafe {
                        multiply_range(r - lo, inv_b as u32, half_l2, &mut tree);
                    }
                    r += b;
                }
            }

            let cur_sum = global_scale * tree[1].sum as u64 % M;
            ans_k = (ans_k + 2 * cur_sum) % M;
            last = b + 1;
        }
    }

    // Primes b > HALF: specialized fast loop
    let threshold = N - lo;
    for b in (HALF + 1)..=N {
        // SAFETY: b <= N < is_prime.len()
        if unsafe { *is_prime.get_unchecked(b) } != 0 {
            let gap = (b - last) as u64;
            if gap > 0 {
                let cur_sum = global_scale * tree[1].sum as u64 % M;
                ans_k = (ans_k + 2 * gap * cur_sum) % M;
            }
            let bb = b as u64;

            if lo > 0 && b > threshold {
                global_scale = global_scale * bb % M;
            }

            let rem = N - b + 1;
            if rem > lo && rem < hi {
                unsafe {
                    multiply_range(rem - lo, bb as u32, half_l2, &mut tree);
                }
            }

            let cur_sum = global_scale * tree[1].sum as u64 % M;
            ans_k = (ans_k + 2 * cur_sum) % M;
            last = b + 1;
        }
    }

    if last <= N {
        let gap = (N - last + 1) as u64;
        let cur_sum = global_scale * tree[1].sum as u64 % M;
        ans_k = (ans_k + 2 * gap * cur_sum) % M;
    }

    ans_k
}

fn main() {
    let mut is_prime = vec![1u8; N + 1];
    is_prime[0] = 0;
    is_prime[1] = 0;
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

    let mut mod_invs = vec![0u32; HALF + 1];
    mod_invs[1] = 1;
    for i in 2..=HALF {
        mod_invs[i] = (M - (M / i as u64) * mod_invs[(M % i as u64) as usize] as u64 % M) as u32;
    }
    let sqrt_n = (N as f64).sqrt() as usize + 1;

    let num_chunks = 32;
    let total: u64 = (0..num_chunks)
        .into_par_iter()
        .map(|k| solve_chunk(k, num_chunks, &is_prime, &mod_invs, sqrt_n))
        .sum::<u64>() % M;

    println!("{}", total);
}
