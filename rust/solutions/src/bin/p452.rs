// Project Euler 452: Long products
// Number of N-tuples of positive integers whose product <= N.

use rayon::prelude::*;

const N: u32 = 1_000_000_000;
const M: u64 = 1_234_567_891;
/// Sequential subtree once remaining product N/n is at most this.
const SPLIT_Q: u32 = 2_000_000;

#[inline(always)]
fn mul(a: u64, b: u64) -> u64 {
    // M^2 = 1.52e18 < 2^64
    a.wrapping_mul(b) % M
}

fn mod_pow(mut base: u64, mut exp: u64) -> u64 {
    let mut res = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            res = mul(res, base);
        }
        base = mul(base, base);
        exp >>= 1;
    }
    res
}

struct Ctx {
    prods: [u64; 64],
    inv_fact: [u64; 64],
}

#[derive(Clone, Copy)]
struct Job {
    min_val: u32,
    q: u32,
    prev: u32,
    num_elements: u32,
    num_perm: u64,
}

#[inline(always)]
fn node_contrib(
    min_val: u32,
    q: u32,
    prev: u32,
    num_elements: u32,
    num_perm: u64,
    prods: &[u64; 64],
) -> u64 {
    let mut ans = 0u64;
    if prev != 1 {
        // SAFETY: num_elements <= 31
        ans += mul(num_perm, unsafe {
            *prods.get_unchecked(num_elements as usize)
        });
    }
    if min_val <= q {
        ans += mul(
            mul(num_perm, unsafe {
                *prods.get_unchecked(num_elements as usize + 1)
            }),
            (q - min_val + 1) as u64,
        );
    }
    ans
}

#[inline(always)]
fn child(
    min_val: u32,
    q: u32,
    prev: u32,
    num_elements: u32,
    num_perm: u64,
    ctx: &Ctx,
    prods: &[u64; 64],
) -> u64 {
    if (min_val as u64) * (min_val as u64) > q as u64 {
        node_contrib(min_val, q, prev, num_elements, num_perm, prods)
    } else {
        helper(min_val, q, prev, num_elements, num_perm, ctx)
    }
}

/// Sequential DFS. `q` is N/n.
fn helper(min_val: u32, q: u32, prev: u32, num_elements: u32, num_perm: u64, ctx: &Ctx) -> u64 {
    let prods = &ctx.prods;
    let inv_fact = &ctx.inv_fact;
    let mut ans = node_contrib(min_val, q, prev, num_elements, num_perm, prods);
    let mut i = min_val;
    loop {
        let mut new_q = q / i;
        if i > new_q {
            break;
        }
        // count=1: inv_fact[1] == 1 so num_perm is unchanged
        ans += child(i + 1, new_q, 1, num_elements + 1, num_perm, ctx, prods);
        let mut count = 2u32;
        while i <= new_q {
            new_q /= i;
            let new_perm = mul(num_perm, unsafe { *inv_fact.get_unchecked(count as usize) });
            ans += child(
                i + 1,
                new_q,
                count,
                num_elements + count,
                new_perm,
                ctx,
                prods,
            );
            count += 1;
        }
        i += 1;
    }
    ans
}

/// Expand large remaining-product nodes into independent child jobs (single par_iter).
fn collect(
    min_val: u32,
    q: u32,
    prev: u32,
    num_elements: u32,
    num_perm: u64,
    ctx: &Ctx,
    jobs: &mut Vec<Job>,
    base: &mut u64,
) {
    if (min_val as u64) * (min_val as u64) > q as u64 {
        *base += node_contrib(min_val, q, prev, num_elements, num_perm, &ctx.prods);
        return;
    }
    // Keep splitting while the smallest remaining factor is tiny (Zipf-heavy).
    if q <= SPLIT_Q && min_val > 4 {
        jobs.push(Job {
            min_val,
            q,
            prev,
            num_elements,
            num_perm,
        });
        return;
    }
    *base += node_contrib(min_val, q, prev, num_elements, num_perm, &ctx.prods);
    let mut i = min_val;
    loop {
        let mut new_q = q / i;
        if i > new_q {
            break;
        }
        let mut count = 1u32;
        loop {
            let new_perm = mul(num_perm, unsafe {
                *ctx.inv_fact.get_unchecked(count as usize)
            });
            collect(
                i + 1,
                new_q,
                count,
                num_elements + count,
                new_perm,
                ctx,
                jobs,
                base,
            );
            count += 1;
            if i > new_q {
                break;
            }
            new_q /= i;
        }
        i += 1;
    }
}

fn main() {
    let mut l = 0usize;
    let mut tmp = N;
    while tmp > 0 {
        l += 1;
        tmp >>= 1;
    }
    l += 1;

    let mut prods = [0u64; 64];
    prods[0] = 1;
    for i in 1..=l {
        prods[i] = mul(prods[i - 1], (N as u64 + 1) - i as u64);
    }

    let mut fact = 1u64;
    for i in 1..=l {
        fact = mul(fact, i as u64);
    }
    let mut inv_fact = [0u64; 64];
    inv_fact[l] = mod_pow(fact, M - 2);
    for i in (1..=l).rev() {
        inv_fact[i - 1] = mul(inv_fact[i], i as u64);
    }

    let ctx = Ctx { prods, inv_fact };
    let mut jobs = Vec::with_capacity(1 << 16);
    let mut ans = 0u64;
    collect(2, N, 0, 0, 1, &ctx, &mut jobs, &mut ans);

    ans += jobs
        .par_iter()
        .with_max_len(1)
        .map(|j| helper(j.min_val, j.q, j.prev, j.num_elements, j.num_perm, &ctx))
        .sum::<u64>();

    println!("{}", ans % M);
}
