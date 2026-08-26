// Problem 949 - Left vs Right II
// G(20, 7) mod 1001001011
//
// Game values are dyadic rationals scaled by 2^n. Length-L words depend only
// on length L-1, so u_raw/d_raw collapse to a max/min of the previous suffix
// and prefix. Histograms stay sparse and are stored as sorted Vec<(value, count)>
// with generate-and-sort convolution (no HashMap).

use rayon::prelude::*;

const MOD: u64 = 1_001_001_011;

type Hist = Vec<(i64, u64)>;

/// Ceiling division of x by 2^s.
#[inline]
fn ceil_div_pow2(x: i64, s: u32) -> i64 {
    if s == 0 {
        return x;
    }
    let d = 1i64 << s;
    if x >= 0 {
        (x + (d - 1)) >> s
    } else {
        -((-x) >> s)
    }
}

/// Find the "simplest" dyadic rational (fewest bits in denominator) strictly
/// between u and d, scaled by 2^e. Returns the numerator times the remaining scale.
#[inline]
fn simplest_between(u: i64, d: i64, e: u32) -> i64 {
    for m in 0..=e {
        let s = e - m;
        let p_min = (u >> s) + 1;
        let p_max = ceil_div_pow2(d, s) - 1;
        if p_min <= p_max {
            let mut p = if p_min > 0 {
                p_min
            } else if p_max < 0 {
                p_max
            } else {
                0
            };
            if m > 0 && p != 0 && (p & 1) == 0 {
                if p + 1 <= p_max && ((p + 1) & 1) != 0 {
                    p += 1;
                } else if p - 1 >= p_min && ((p - 1) & 1) != 0 {
                    p -= 1;
                }
            }
            return p << s;
        }
    }
    0
}

/// Game values (`u`) and hot flags for all 2^n words of length n.
///
/// A word of length L only needs its length-(L-1) suffix and prefix:
///   u_raw(w) = max(dp_u, dp_d) of the proper suffix of length L-1
///   d_raw(w) = min(dp_u, dp_d) of the proper prefix of length L-1
fn compute_u_hot(n: u32) -> (Vec<i64>, Vec<bool>) {
    let scale = 1i64 << n;
    let final_size = 1usize << n;
    let mut prev_u = Vec::with_capacity(final_size);
    let mut prev_d = Vec::with_capacity(final_size);
    prev_u.extend_from_slice(&[scale, -scale]);
    prev_d.extend_from_slice(&[scale, -scale]);
    let mut cur_u = Vec::with_capacity(final_size);
    let mut cur_d = Vec::with_capacity(final_size);

    for length in 2..n {
        let size = 1usize << length;
        cur_u.clear();
        cur_d.clear();
        cur_u.resize(size, 0);
        cur_d.resize(size, 0);
        fill_layer(&prev_u, &prev_d, &mut cur_u, &mut cur_d, size / 2 - 1, n, length);
        std::mem::swap(&mut prev_u, &mut cur_u);
        std::mem::swap(&mut prev_d, &mut cur_d);
    }

    let mut u_full = vec![0i64; final_size];
    let mut hot = vec![false; final_size];
    fill_last_layer(&prev_u, &prev_d, &mut u_full, &mut hot, final_size / 2 - 1, n);
    (u_full, hot)
}

fn fill_layer(
    prev_u: &[i64],
    prev_d: &[i64],
    cur_u: &mut [i64],
    cur_d: &mut [i64],
    mask: usize,
    e: u32,
    length: u32,
) {
    let eval = |bits: usize| -> (i64, i64) {
        let suf = bits & mask;
        let pre = bits >> 1;
        let u_raw = prev_u[suf].max(prev_d[suf]);
        let d_raw = prev_u[pre].min(prev_d[pre]);
        if u_raw < d_raw {
            let x = simplest_between(u_raw, d_raw, e);
            (x, x)
        } else {
            (u_raw, d_raw)
        }
    };

    if length >= 18 {
        cur_u
            .par_iter_mut()
            .zip(cur_d.par_iter_mut())
            .enumerate()
            .for_each(|(bits, (u, d))| {
                let (uu, dd) = eval(bits);
                *u = uu;
                *d = dd;
            });
    } else {
        for bits in 0..cur_u.len() {
            let (uu, dd) = eval(bits);
            cur_u[bits] = uu;
            cur_d[bits] = dd;
        }
    }
}

fn fill_last_layer(
    prev_u: &[i64],
    prev_d: &[i64],
    cur_u: &mut [i64],
    hot: &mut [bool],
    mask: usize,
    e: u32,
) {
    cur_u
        .par_iter_mut()
        .zip(hot.par_iter_mut())
        .enumerate()
        .for_each(|(bits, (u, h))| {
            let suf = bits & mask;
            let pre = bits >> 1;
            let u_raw = prev_u[suf].max(prev_d[suf]);
            let d_raw = prev_u[pre].min(prev_d[pre]);
            if u_raw < d_raw {
                *u = simplest_between(u_raw, d_raw, e);
                *h = false;
            } else {
                *u = u_raw;
                *h = true;
            }
        });
}

/// Run-length encode a sorted slice of values into a modulus histogram.
fn rle_hist(sorted: &[i64], modulus: u64) -> Hist {
    let mut out = Vec::new();
    let mut i = 0;
    let n = sorted.len();
    while i < n {
        let v = sorted[i];
        let mut j = i + 1;
        while j < n && sorted[j] == v {
            j += 1;
        }
        let c = ((j - i) as u64) % modulus;
        if c != 0 {
            out.push((v, c));
        }
        i = j;
    }
    out
}

/// Merge a sorted list of (sum, count) pairs, adding counts for equal keys.
fn merge_sorted_pairs(pairs: &[(i64, u64)], modulus: u64) -> Hist {
    let mut out = Vec::new();
    let mut i = 0;
    let n = pairs.len();
    while i < n {
        let s = pairs[i].0;
        let mut c = 0u64;
        while i < n && pairs[i].0 == s {
            c += pairs[i].1;
            i += 1;
        }
        c %= modulus;
        if c != 0 {
            out.push((s, c));
        }
    }
    out
}

/// Sparse convolution of two sorted histograms: generate all pairwise sums,
/// sort, and merge. Parallel for the large (n=20, k=7) products.
fn convolve(a: &Hist, b: &Hist, modulus: u64) -> Hist {
    if a.is_empty() || b.is_empty() {
        return Hist::new();
    }
    let (small, large) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    let n = small.len() * large.len();

    let mut pairs = vec![(0i64, 0u64); n];
    if n >= 50_000 {
        pairs
            .par_chunks_mut(large.len())
            .zip(small.par_iter())
            .for_each(|(chunk, &(xa, ca))| {
                for (slot, &(xb, cb)) in chunk.iter_mut().zip(large.iter()) {
                    *slot = (xa + xb, ca * cb % modulus);
                }
            });
        pairs.par_sort_unstable_by_key(|&(s, _)| s);
    } else {
        let mut k = 0;
        for &(xa, ca) in small {
            for &(xb, cb) in large {
                pairs[k] = (xa + xb, ca * cb % modulus);
                k += 1;
            }
        }
        pairs.sort_unstable_by_key(|&(s, _)| s);
    }
    merge_sorted_pairs(&pairs, modulus)
}

/// hist convolved with itself t times. No clone of the running product.
fn pow_small(hist: &Hist, t: u32, modulus: u64) -> Hist {
    match t {
        0 => vec![(0, 1)],
        1 => hist.clone(),
        _ => {
            let mut d = convolve(hist, hist, modulus);
            for _ in 2..t {
                d = convolve(&d, hist, modulus);
            }
            d
        }
    }
}

/// Count weighted pairs with sa + sb < 0. Two-pointer on sorted histograms.
fn count_sum_lt_zero(a: &Hist, b: &Hist, modulus: u64) -> u64 {
    let mut pref = Vec::with_capacity(b.len() + 1);
    pref.push(0u64);
    let mut run = 0u64;
    for &(_, c) in b {
        run = (run + c) % modulus;
        pref.push(run);
    }

    let mut ans = 0u64;
    let mut j = b.len();
    for &(sa, ca) in a {
        let target = -sa;
        while j > 0 && b[j - 1].0 >= target {
            j -= 1;
        }
        ans = (ans + ca * pref[j] % modulus) % modulus;
    }
    ans
}

/// Count weighted pairs with sa + sb == 0. Two-pointer on sorted histograms.
fn count_sum_eq_zero(a: &Hist, b: &Hist, modulus: u64) -> u64 {
    let mut i = 0usize;
    let mut j = b.len();
    let mut ans = 0u64;
    while i < a.len() && j > 0 {
        let s = a[i].0 + b[j - 1].0;
        if s == 0 {
            ans = (ans + a[i].1 * b[j - 1].1 % modulus) % modulus;
            i += 1;
            j -= 1;
        } else if s > 0 {
            j -= 1;
        } else {
            i += 1;
        }
    }
    ans
}

/// Compute G(n, k) mod modulus.
fn g(n: u32, k: u32, modulus: u64) -> u64 {
    assert!(k % 2 == 1, "k must be odd");
    assert!(n > 0, "n must be positive");

    let (mut u_full, hot) = compute_u_hot(n);

    let mut cold_values = Vec::with_capacity(u_full.len() / 16);
    for (&v, &h) in u_full.iter().zip(hot.iter()) {
        if !h {
            cold_values.push(v);
        }
    }

    u_full.par_sort_unstable();
    let u_hist = rle_hist(&u_full, modulus);
    cold_values.sort_unstable();
    let cold_hist = rle_hist(&cold_values, modulus);

    let a = k / 2;
    // For odd k, b = a + 1, so hist^b = hist^a * hist.

    let (neg, zero_cold) = rayon::join(
        || {
            let dist_a = pow_small(&u_hist, a, modulus);
            let dist_b = convolve(&dist_a, &u_hist, modulus);
            count_sum_lt_zero(&dist_a, &dist_b, modulus)
        },
        || {
            let cold_a = pow_small(&cold_hist, a, modulus);
            let cold_b = convolve(&cold_a, &cold_hist, modulus);
            count_sum_eq_zero(&cold_a, &cold_b, modulus)
        },
    );

    (neg + zero_cold) % modulus
}

fn main() {
    println!("{}", g(20, 7, MOD));
}
