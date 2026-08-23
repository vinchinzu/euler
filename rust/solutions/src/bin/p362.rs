// Project Euler 362: Squarefree factors
// Sum of Fsf(k) for k=1 to N=10^10.
// Floor-indexed nsf arrays (no HashMap), blocked Möbius Q, rayon DFS.

use rayon::prelude::*;

const N_VAL: i64 = 10_000_000_000;
const PAR_REM: i64 = 2_000_000;

#[inline(always)]
fn isqrt(n: i64) -> i64 {
    (n as u64).isqrt() as i64
}

struct Ctx {
    n: i64,
    l: i64,
    square_frees: Vec<i64>,
    sf_cumul: Vec<i64>,
    large: Vec<i64>,
}

#[inline(always)]
fn nsf_quot(ctx: &Ctx, prod: i64, x: i64) -> i64 {
    if x <= ctx.l {
        // SAFETY: 0 <= x <= l, sf_cumul.len() == l+1
        unsafe { *ctx.sf_cumul.get_unchecked(x as usize) }
    } else {
        // SAFETY: x = n/prod > l ⇒ prod < l, large.len() == l+1
        unsafe { *ctx.large.get_unchecked(prod as usize) }
    }
}

fn count_square_free(x: i64, mu_pref: &[i64], s_max: i64) -> i64 {
    if x < 1 {
        return 0;
    }
    let s = isqrt(x).min(s_max);
    let mut total = 0i64;
    let mut d = 1i64;
    while d <= s {
        let v = x / (d * d);
        let mut d2 = isqrt(x / v);
        if d2 > s {
            d2 = s;
        }
        // SAFETY: 0 <= d-1 < d <= d2 <= s <= l
        let pref_hi = unsafe { *mu_pref.get_unchecked(d2 as usize) };
        let pref_lo = unsafe { *mu_pref.get_unchecked((d - 1) as usize) };
        total += (pref_hi - pref_lo) * v;
        d = d2 + 1;
    }
    total
}

fn dfs_seq(ctx: &Ctx, prev_index: usize, prod: i64) -> i64 {
    let x = ctx.n / prod;
    let nsf = nsf_quot(ctx, prod, x);
    let mut ans = nsf - prev_index as i64;
    if x < 4 {
        return ans;
    }
    let max_sf = isqrt(x);
    if max_sf < 2 {
        return ans;
    }
    // SAFETY: max_sf = isqrt(n/prod) <= l
    let last_count = unsafe { *ctx.sf_cumul.get_unchecked(max_sf as usize) };
    if last_count <= prev_index as i64 {
        return ans;
    }
    let last = last_count as usize - 1;
    for index in prev_index..=last {
        // SAFETY: last < square_frees.len()
        let sf = unsafe { *ctx.square_frees.get_unchecked(index) };
        ans += dfs_seq(ctx, index, prod * sf);
    }
    ans
}

fn add_children(ctx: &Ctx, prod: i64, lo: usize, hi: usize) -> i64 {
    let mut split = lo;
    while split <= hi {
        // SAFETY: lo..=hi are valid square_frees indices
        let sf = unsafe { *ctx.square_frees.get_unchecked(split) };
        if ctx.n / (prod * sf) < PAR_REM {
            break;
        }
        split += 1;
    }
    let heavy = if split > lo {
        (lo..split)
            .into_par_iter()
            .map(|index| {
                // SAFETY: index in lo..split ⊆ lo..=hi
                let sf = unsafe { *ctx.square_frees.get_unchecked(index) };
                dfs_par(ctx, index, prod * sf)
            })
            .sum::<i64>()
    } else {
        0
    };
    let mut light = 0i64;
    for index in split..=hi {
        // SAFETY: index in split..=hi
        let sf = unsafe { *ctx.square_frees.get_unchecked(index) };
        light += dfs_seq(ctx, index, prod * sf);
    }
    heavy + light
}

fn dfs_par(ctx: &Ctx, prev_index: usize, prod: i64) -> i64 {
    let x = ctx.n / prod;
    if x < PAR_REM {
        return dfs_seq(ctx, prev_index, prod);
    }
    let nsf = nsf_quot(ctx, prod, x);
    let mut ans = nsf - prev_index as i64;
    if x < 4 {
        return ans;
    }
    let max_sf = isqrt(x);
    if max_sf < 2 {
        return ans;
    }
    // SAFETY: max_sf = isqrt(n/prod) <= l
    let last_count = unsafe { *ctx.sf_cumul.get_unchecked(max_sf as usize) };
    if last_count <= prev_index as i64 {
        return ans;
    }
    let last = last_count as usize - 1;
    ans += add_children(ctx, prod, prev_index, last);
    ans
}

fn main() {
    let l = isqrt(N_VAL) as usize;

    let mut mu = vec![1i8; l + 1];
    mu[0] = 0;
    let mut is_prime_arr = vec![true; l + 1];
    is_prime_arr[0] = false;
    is_prime_arr[1] = false;

    {
        let mut i = 2usize;
        while i * i <= l {
            if is_prime_arr[i] {
                let mut j = i * i;
                while j <= l {
                    is_prime_arr[j] = false;
                    j += i;
                }
                let sq = i * i;
                let mut j = sq;
                while j <= l {
                    mu[j] = 0;
                    j += sq;
                }
            }
            i += 1;
        }
    }

    for i in 2..=l {
        if mu[i] == 0 {
            continue;
        }
        if is_prime_arr[i] {
            let mut j = i;
            while j <= l {
                mu[j] = -mu[j];
                j += i;
            }
        }
    }

    let mut mu_pref = vec![0i64; l + 1];
    for i in 1..=l {
        mu_pref[i] = mu_pref[i - 1] + mu[i] as i64;
    }

    let mut sf_cumul = vec![0i64; l + 1];
    let mut square_frees = Vec::with_capacity(l * 2 / 3 + 8);
    for i in 2..=l {
        sf_cumul[i] = sf_cumul[i - 1] + i64::from(mu[i] != 0);
        if mu[i] != 0 {
            square_frees.push(i as i64);
        }
    }

    let li = l as i64;
    let mut large = vec![0i64; l + 1];
    {
        let mut i = 1i64;
        while i <= li {
            let q = N_VAL / i;
            let last = li.min(N_VAL / q);
            let val = if q <= li {
                sf_cumul[q as usize]
            } else {
                count_square_free(q, &mu_pref, li) - 1
            };
            for k in i..=last {
                large[k as usize] = val;
            }
            i = last + 1;
        }
    }

    drop(mu);
    drop(mu_pref);
    drop(is_prime_arr);

    let ctx = Ctx {
        n: N_VAL,
        l: li,
        square_frees,
        sf_cumul,
        large,
    };

    let ans = dfs_par(&ctx, 0, 1);
    println!("{}", ans);
}
