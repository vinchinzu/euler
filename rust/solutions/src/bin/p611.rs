// Project Euler 611 - Hallway of square steps
// Lucy DP for primes by residue mod 4 + DFS over prime powers

use rayon::prelude::*;

#[derive(Clone, Copy)]
struct Frame {
    min_idx: usize,
    n: i64,
    p_val: i64,
    skip: bool,
}

struct Ctx<'a> {
    primes: &'a [i64],
    p2s: &'a [i64],
    p_k: &'a [u8],
    adj: &'a [i32],
    small0: &'a [i32],
    big0: &'a [i64],
    n_val: i64,
    l: i64,
    last: usize,
}

#[inline(always)]
fn node_contrib(f: &Frame, nlim: i64, ctx: &Ctx) -> i64 {
    let mut ans = 0i64;
    // ((P+1)/2 - P) % 2 != 0  <=>  P ≡ 2 or 3 (mod 4)
    if !f.skip && (f.p_val & 2) != 0 {
        ans += 1;
    }
    let p0 = unsafe { *ctx.primes.get_unchecked(f.min_idx) };
    if nlim >= p0 && (f.p_val & 1) != 0 {
        // n <= l ⇒ π₁(N/n) = big0[n]; else small0[N/n]
        let pi1 = if f.n <= ctx.l {
            unsafe { *ctx.big0.get_unchecked(f.n as usize) }
        } else {
            unsafe { *ctx.small0.get_unchecked(nlim as usize) as i64 }
        };
        ans += pi1 + unsafe { *ctx.adj.get_unchecked(f.min_idx) as i64 };
    }
    ans
}

#[inline(always)]
fn push_powers(idx: usize, n: i64, nlim: i64, p_val: i64, ctx: &Ctx, stack: &mut Vec<Frame>) {
    let p = unsafe { *ctx.primes.get_unchecked(idx) };
    let k = unsafe { *ctx.p_k.get_unchecked(idx) };
    let next = idx + 1;
    if k == 3 {
        let pp = unsafe { *ctx.p2s.get_unchecked(idx) };
        let mut pe = pp;
        loop {
            stack.push(Frame { min_idx: next, n: n * pe, p_val, skip: false });
            if pe > nlim / pp {
                break;
            }
            pe *= pp;
        }
    } else if k == 1 {
        let mut pe = p;
        let mut e = 1i64;
        loop {
            stack.push(Frame {
                min_idx: next,
                n: n * pe,
                p_val: p_val * (e + 1),
                skip: e == 1,
            });
            if pe > nlim / p {
                break;
            }
            pe *= p;
            e += 1;
        }
    } else {
        // p = 2
        let mut pe = 2i64;
        loop {
            stack.push(Frame { min_idx: next, n: n * pe, p_val, skip: false });
            if pe > nlim / 2 {
                break;
            }
            pe *= 2;
        }
    }
}

#[inline(always)]
unsafe fn sub_range_i64(a: &mut [i64], b: &mut [i64], mut i: i64, i_last: i64, va: i64, vb: i64) -> i64 {
    unsafe {
        while i + 4 <= i_last {
            let iu = i as usize;
            *a.get_unchecked_mut(iu) -= va;
            *b.get_unchecked_mut(iu) -= vb;
            *a.get_unchecked_mut(iu + 1) -= va;
            *b.get_unchecked_mut(iu + 1) -= vb;
            *a.get_unchecked_mut(iu + 2) -= va;
            *b.get_unchecked_mut(iu + 2) -= vb;
            *a.get_unchecked_mut(iu + 3) -= va;
            *b.get_unchecked_mut(iu + 3) -= vb;
            i += 4;
        }
        while i <= i_last {
            *a.get_unchecked_mut(i as usize) -= va;
            *b.get_unchecked_mut(i as usize) -= vb;
            i += 1;
        }
    }
    i
}

#[inline(always)]
unsafe fn sub_range_i32(a: &mut [i32], b: &mut [i32], mut j: i64, i: i64, va: i32, vb: i32) {
    unsafe {
        while j + 4 <= i {
            let ju = j as usize;
            *a.get_unchecked_mut(ju) -= va;
            *b.get_unchecked_mut(ju) -= vb;
            *a.get_unchecked_mut(ju + 1) -= va;
            *b.get_unchecked_mut(ju + 1) -= vb;
            *a.get_unchecked_mut(ju + 2) -= va;
            *b.get_unchecked_mut(ju + 2) -= vb;
            *a.get_unchecked_mut(ju + 3) -= va;
            *b.get_unchecked_mut(ju + 3) -= vb;
            j += 4;
        }
        while j <= i {
            *a.get_unchecked_mut(j as usize) -= va;
            *b.get_unchecked_mut(j as usize) -= vb;
            j += 1;
        }
    }
}

fn dfs_seq(start: Frame, ctx: &Ctx) -> i64 {
    let mut stack = Vec::with_capacity(4096);
    stack.push(start);
    let mut ans = 0i64;
    while let Some(f) = stack.pop() {
        let nlim = ctx.n_val / f.n;
        ans += node_contrib(&f, nlim, ctx);
        let mut idx = f.min_idx;
        while idx < ctx.last {
            let pp = unsafe { *ctx.p2s.get_unchecked(idx) };
            if pp > nlim {
                break;
            }
            push_powers(idx, f.n, nlim, f.p_val, ctx, &mut stack);
            idx += 1;
        }
    }
    ans
}


fn main() {
    let n_val: i64 = 1_000_000_000_000;
    let l = isqrt(n_val);
    let lu = l as usize;

    // Odd sieve up to l
    let mut sieve = vec![0u8; lu + 1];
    sieve[0] = 1;
    sieve[1] = 1;
    let mut i = 3usize;
    while i * i <= lu {
        if sieve[i] == 0 {
            let mut j = i * i;
            while j <= lu {
                sieve[j] = 1;
                j += i + i;
            }
        }
        i += 2;
    }

    let mut primes: Vec<i64> = Vec::with_capacity(80_000);
    primes.push(2);
    let mut p = 3i64;
    while p <= l {
        if sieve[p as usize] == 0 {
            primes.push(p);
        }
        p += 2;
    }
    drop(sieve);

    let last = primes.len() - 1;
    let p2s: Vec<i64> = primes.iter().map(|&p| p * p).collect();
    let p_k: Vec<u8> = primes.iter().map(|&p| (p & 3) as u8).collect();

    let big_size = (n_val / l + 1) as usize;
    let mut big0 = vec![0i64; big_size];
    let mut big1 = vec![0i64; big_size];
    let mut small0 = vec![0i32; lu + 1];
    let mut small1 = vec![0i32; lu + 1];

    for i in 1..big_size {
        let v = n_val / i as i64;
        big0[i] = (v + 3) / 4;
        big1[i] = (v + 1) / 4;
    }
    for i in 1..=lu {
        small0[i] = ((i as i64 + 3) / 4) as i32;
        small1[i] = ((i as i64 + 1) / 4) as i32;
    }

    // Lucy DP: loop-carried over primes; inner i-updates use old larger indices.
    let bs_m1 = (big_size - 1) as i64;
    for pi in 1..primes.len() {
        let p = primes[pi];
        let p2 = p2s[pi];
        let sp0 = small0[(p - 1) as usize];
        let sp1 = small1[(p - 1) as usize];
        let sp0_64 = sp0 as i64;
        let sp1_64 = sp1 as i64;
        let mod1 = p_k[pi] == 1;
        let pu = p as usize;

        let m = n_val / p;
        let i_max = (m / p).min(bs_m1);
        if i_max >= 1 {
            let i_mid = (bs_m1 / p).min(i_max);
            unsafe {
                let mut ip = pu;
                if mod1 {
                    for i in 1..=i_mid as usize {
                        let v0 = *big0.get_unchecked(ip) - sp0_64;
                        let v1 = *big1.get_unchecked(ip) - sp1_64;
                        *big0.get_unchecked_mut(i) -= v0;
                        *big1.get_unchecked_mut(i) -= v1;
                        ip += pu;
                    }
                    let mut i = i_mid + 1;
                    while i <= i_max {
                        let q = m / i;
                        let mut i_last = m / q;
                        if i_last > i_max {
                            i_last = i_max;
                        }
                        let v0 = *small0.get_unchecked(q as usize) as i64 - sp0_64;
                        let v1 = *small1.get_unchecked(q as usize) as i64 - sp1_64;
                        i = sub_range_i64(&mut big0, &mut big1, i, i_last, v0, v1);
                    }
                } else {
                    for i in 1..=i_mid as usize {
                        let v0 = *big0.get_unchecked(ip) - sp0_64;
                        let v1 = *big1.get_unchecked(ip) - sp1_64;
                        *big0.get_unchecked_mut(i) -= v1;
                        *big1.get_unchecked_mut(i) -= v0;
                        ip += pu;
                    }
                    let mut i = i_mid + 1;
                    while i <= i_max {
                        let q = m / i;
                        let mut i_last = m / q;
                        if i_last > i_max {
                            i_last = i_max;
                        }
                        let v0 = *small0.get_unchecked(q as usize) as i64 - sp0_64;
                        let v1 = *small1.get_unchecked(q as usize) as i64 - sp1_64;
                        i = sub_range_i64(&mut big0, &mut big1, i, i_last, v1, v0);
                    }
                }
            }
        }

        if p2 <= l {
            let mut i = l;
            unsafe {
                while i >= p2 {
                    let q = i / p;
                    let lo = (q * p).max(p2);
                    let v0 = *small0.get_unchecked(q as usize) - sp0;
                    let v1 = *small1.get_unchecked(q as usize) - sp1;
                    if mod1 {
                        sub_range_i32(&mut small0, &mut small1, lo, i, v0, v1);
                    } else {
                        sub_range_i32(&mut small0, &mut small1, lo, i, v1, v0);
                    }
                    i = lo - 1;
                }
            }
        }
    }

    // Remove count of 1
    for i in 1..big_size {
        big0[i] -= 1;
    }
    for i in 1..=lu {
        small0[i] -= 1;
    }
    drop(big1);
    drop(small1);

    let adj: Vec<i32> = primes
        .iter()
        .map(|&p| -small0[p as usize] + if p & 3 == 1 { 1 } else { 0 })
        .collect();

    let ctx = Ctx {
        primes: &primes,
        p2s: &p2s,
        p_k: &p_k,
        adj: &adj,
        small0: &small0,
        big0: &big0,
        n_val,
        l,
        last,
    };

    // Flatten top-level prime-power frames
    let root = Frame { min_idx: 0, n: 1, p_val: 1, skip: true };
    let mut initial_frames = Vec::with_capacity(100_000);

    // Expand level 1 (from root)
    let mut top_frames = Vec::new();
    for idx in 0..last {
        let pp = p2s[idx];
        if pp > n_val {
            break;
        }
        push_powers(idx, 1, n_val, 1, &ctx, &mut top_frames);
    }

    // For heavy frames (e.g. powers of 2 or small n), expand one more level into initial_frames
    let mut direct_contrib = node_contrib(&root, n_val, &ctx);
    for f in top_frames {
        let nlim = n_val / f.n;
        if f.n <= 128 {
            direct_contrib += node_contrib(&f, nlim, &ctx);
            let mut idx = f.min_idx;
            while idx < ctx.last {
                let pp = unsafe { *ctx.p2s.get_unchecked(idx) };
                if pp > nlim {
                    break;
                }
                push_powers(idx, f.n, nlim, f.p_val, &ctx, &mut initial_frames);
                idx += 1;
            }
        } else {
            initial_frames.push(f);
        }
    }

    let ans_parallel: i64 = initial_frames
        .into_par_iter()
        .map(|f| dfs_seq(f, &ctx))
        .sum();
    let ans = direct_contrib + ans_parallel;

    println!("{}", ans);
}

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}
