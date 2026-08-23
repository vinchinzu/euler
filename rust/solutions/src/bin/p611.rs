// Project Euler 611 - Hallway of square steps
// Lucy DP for primes by residue mod 4 + DFS over prime powers

use rayon::prelude::*;

struct Frame {
    min_idx: usize,
    n: i64,
    p_val: i64,
    skip: bool,
}

struct Ctx<'a> {
    primes: &'a [i64],
    p2s: &'a [i64],
    adj: &'a [i64],
    small0: &'a [i64],
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
            unsafe { *ctx.small0.get_unchecked(nlim as usize) }
        };
        ans += pi1 + unsafe { *ctx.adj.get_unchecked(f.min_idx) };
    }
    ans
}

#[inline(always)]
fn push_powers(idx: usize, n: i64, nlim: i64, p_val: i64, ctx: &Ctx, stack: &mut Vec<Frame>) {
    let p = unsafe { *ctx.primes.get_unchecked(idx) };
    let k = p & 3;
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

fn add_powers(idx: usize, n: i64, nlim: i64, p_val: i64, ctx: &Ctx, par: bool) -> i64 {
    let p = unsafe { *ctx.primes.get_unchecked(idx) };
    let k = p & 3;
    let next = idx + 1;
    let mut local = 0i64;
    if k == 3 {
        let pp = unsafe { *ctx.p2s.get_unchecked(idx) };
        let mut pe = pp;
        loop {
            let fr = Frame { min_idx: next, n: n * pe, p_val, skip: false };
            local += if par { dfs_par(fr, ctx) } else { dfs_seq(fr, ctx) };
            if pe > nlim / pp {
                break;
            }
            pe *= pp;
        }
    } else if k == 1 {
        let mut pe = p;
        let mut e = 1i64;
        loop {
            let fr = Frame {
                min_idx: next,
                n: n * pe,
                p_val: p_val * (e + 1),
                skip: e == 1,
            };
            local += if par { dfs_par(fr, ctx) } else { dfs_seq(fr, ctx) };
            if pe > nlim / p {
                break;
            }
            pe *= p;
            e += 1;
        }
    } else {
        let mut pe = 2i64;
        loop {
            let fr = Frame { min_idx: next, n: n * pe, p_val, skip: false };
            local += if par { dfs_par(fr, ctx) } else { dfs_seq(fr, ctx) };
            if pe > nlim / 2 {
                break;
            }
            pe *= 2;
        }
    }
    local
}

fn dfs_par(f: Frame, ctx: &Ctx) -> i64 {
    let nlim = ctx.n_val / f.n;
    let mut ans = node_contrib(&f, nlim, ctx);
    let mut end = f.min_idx;
    while end < ctx.last {
        let pp = unsafe { *ctx.p2s.get_unchecked(end) };
        if pp > nlim {
            break;
        }
        end += 1;
    }
    if end <= f.min_idx {
        return ans;
    }
    // Nested split only for tiny n (heavy subtrees). n=1 handled at root.
    if f.n <= 16 && end - f.min_idx > 32 {
        ans += (f.min_idx..end)
            .into_par_iter()
            .map(|idx| add_powers(idx, f.n, nlim, f.p_val, ctx, false))
            .sum::<i64>();
    } else {
        for idx in f.min_idx..end {
            ans += add_powers(idx, f.n, nlim, f.p_val, ctx, false);
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

    let big_size = (n_val / l + 1) as usize;
    let mut big = vec![[0i64; 2]; big_size];
    let mut small = vec![[0i64; 2]; lu + 1];

    for i in 1..big_size {
        let v = n_val / i as i64;
        big[i][0] = (v + 3) / 4;
        big[i][1] = (v + 1) / 4;
    }
    for i in 1..=lu {
        small[i][0] = (i as i64 + 3) / 4;
        small[i][1] = (i as i64 + 1) / 4;
    }

    // Lucy DP: loop-carried over primes; inner i-updates use old larger indices.
    let bs_m1 = (big_size - 1) as i64;
    for pi in 1..primes.len() {
        let p = primes[pi];
        let p2 = p2s[pi];
        let sp0 = small[(p - 1) as usize][0];
        let sp1 = small[(p - 1) as usize][1];
        let mod1 = p & 3 == 1;
        let pu = p as usize;

        let i_max = (n_val / p2).min(bs_m1);
        if i_max >= 1 {
            let i_mid = (bs_m1 / p).min(i_max);
            unsafe {
                if mod1 {
                    let mut ip = pu;
                    for i in 1..=i_mid as usize {
                        let v0 = big.get_unchecked(ip)[0] - sp0;
                        let v1 = big.get_unchecked(ip)[1] - sp1;
                        big.get_unchecked_mut(i)[0] -= v0;
                        big.get_unchecked_mut(i)[1] -= v1;
                        ip += pu;
                    }
                    let m = n_val / p;
                    let mut i = i_mid + 1;
                    while i <= i_max {
                        let q = m / i;
                        let mut i_last = m / q;
                        if i_last > i_max {
                            i_last = i_max;
                        }
                        let v0 = small.get_unchecked(q as usize)[0] - sp0;
                        let v1 = small.get_unchecked(q as usize)[1] - sp1;
                        while i <= i_last {
                            big.get_unchecked_mut(i as usize)[0] -= v0;
                            big.get_unchecked_mut(i as usize)[1] -= v1;
                            i += 1;
                        }
                    }
                } else {
                    let mut ip = pu;
                    for i in 1..=i_mid as usize {
                        let v0 = big.get_unchecked(ip)[0] - sp0;
                        let v1 = big.get_unchecked(ip)[1] - sp1;
                        big.get_unchecked_mut(i)[0] -= v1;
                        big.get_unchecked_mut(i)[1] -= v0;
                        ip += pu;
                    }
                    let m = n_val / p;
                    let mut i = i_mid + 1;
                    while i <= i_max {
                        let q = m / i;
                        let mut i_last = m / q;
                        if i_last > i_max {
                            i_last = i_max;
                        }
                        let v0 = small.get_unchecked(q as usize)[0] - sp0;
                        let v1 = small.get_unchecked(q as usize)[1] - sp1;
                        while i <= i_last {
                            big.get_unchecked_mut(i as usize)[0] -= v1;
                            big.get_unchecked_mut(i as usize)[1] -= v0;
                            i += 1;
                        }
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
                    let v0 = small.get_unchecked(q as usize)[0] - sp0;
                    let v1 = small.get_unchecked(q as usize)[1] - sp1;
                    if mod1 {
                        let mut j = lo;
                        while j <= i {
                            small.get_unchecked_mut(j as usize)[0] -= v0;
                            small.get_unchecked_mut(j as usize)[1] -= v1;
                            j += 1;
                        }
                    } else {
                        let mut j = lo;
                        while j <= i {
                            small.get_unchecked_mut(j as usize)[0] -= v1;
                            small.get_unchecked_mut(j as usize)[1] -= v0;
                            j += 1;
                        }
                    }
                    i = lo - 1;
                }
            }
        }
    }

    // Remove count of 1
    for i in 1..big_size {
        big[i][0] -= 1;
    }
    for i in 1..=lu {
        small[i][0] -= 1;
    }

    let small0: Vec<i64> = small.iter().map(|s| s[0]).collect();
    let big0: Vec<i64> = big.iter().map(|s| s[0]).collect();
    drop(small);
    drop(big);

    let adj: Vec<i64> = primes
        .iter()
        .map(|&p| -small0[p as usize] + if p & 3 == 1 { 1 } else { 0 })
        .collect();

    let ctx = Ctx {
        primes: &primes,
        p2s: &p2s,
        adj: &adj,
        small0: &small0,
        big0: &big0,
        n_val,
        l,
        last,
    };

    // Root frame (n=1, skip): qv contribution + parallel prime-power subtrees
    let root = Frame { min_idx: 0, n: 1, p_val: 1, skip: true };
    let nlim = n_val;
    let mut ans = node_contrib(&root, nlim, &ctx);
    ans += (0..last)
        .into_par_iter()
        .map(|idx| {
            let pp = p2s[idx];
            if pp > nlim {
                return 0;
            }
            // p=2 (idx 0) subtree is huge: split its small-n frames further
            add_powers(idx, 1, nlim, 1, &ctx, idx == 0)
        })
        .sum::<i64>();

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
