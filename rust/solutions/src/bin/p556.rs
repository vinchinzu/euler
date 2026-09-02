// Project Euler 556 - Squarefree Gaussian Integers
//
// Count proper squarefree Gaussian integers a+bi with a^2+b^2 <= N.
// Uses Mobius-like recursion with hash table memoization.

use fxhash::FxHashMap;

const N: i64 = 100_000_000_000_000; // 10^14

fn isqrt(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut r = (n as f64).sqrt() as i64;
    if r * r > n { r -= 1; }
    else if (r + 1) * (r + 1) <= n { r += 1; }
    r
}

fn count_gauss(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let sq = isqrt(n);
    let k = isqrt(n / 2);
    let mut b = sq;
    let mut b_sq = b * b;
    let mut inner: i64 = 0;
    let mut a_sq: i64 = 1;
    let mut two_a_plus_1: i64 = 3;
    for a in 1..=k {
        let target = n - a_sq;
        a_sq += two_a_plus_1;
        two_a_plus_1 += 2;
        while b_sq > target {
            b -= 1;
            b_sq -= 2 * b + 1;
        }
        inner += b - a;
    }
    sq + k + 2 * inner
}

struct Precomputed {
    prefix: Vec<u32>,
    small_pairs: Vec<(i64, i64)>,
}

fn f(
    n: i64,
    pre: &Precomputed,
    small_cache: &mut [i64],
    cache: &mut FxHashMap<i64, i64>,
) -> i64 {
    if n == 0 {
        return 0;
    }
    if (n as usize) < small_cache.len() {
        let v = small_cache[n as usize];
        if v != -1 {
            return v;
        }
    } else if let Some(&v) = cache.get(&n) {
        return v;
    }

    let sn = isqrt(n);
    if sn < 2 {
        let res = if n == 1 { 1 } else { 2 };
        if (n as usize) < small_cache.len() {
            small_cache[n as usize] = res;
        } else {
            cache.insert(n, res);
        }
        return res;
    }

    let mut result = if (n as usize) < pre.prefix.len() {
        pre.prefix[n as usize] as i64
    } else {
        count_gauss(n)
    };

    if n <= 250_000 {
        for &(m, cnt) in &pre.small_pairs {
            if m > sn {
                break;
            }
            let arg = n / (m * m);
            result -= cnt * f(arg, pre, small_cache, cache);
        }
        small_cache[n as usize] = result;
        return result;
    }

    let m_thresh = (n as f64).cbrt() as i64;

    // Part 1: m <= M
    for &(m, cnt) in &pre.small_pairs {
        if m > m_thresh {
            break;
        }
        let arg = n / (m * m);
        result -= cnt * f(arg, pre, small_cache, cache);
    }

    // Part 2: m > M via hyperbola chunking
    let v_max = n / ((m_thresh + 1) * (m_thresh + 1));
    let mut curr_high = sn;
    for v in 1..=v_max {
        let next_low = isqrt(n / (v + 1));
        let low_m = next_low.max(m_thresh);
        if curr_high > low_m {
            let cnt = (pre.prefix[curr_high as usize] - pre.prefix[low_m as usize]) as i64;
            if cnt > 0 {
                result -= cnt * f(v, pre, small_cache, cache);
            }
        }
        if next_low <= m_thresh {
            break;
        }
        curr_high = next_low;
    }

    cache.insert(n, result);
    result
}

fn main() {
    let limit = 10_000_000usize;
    let mut prefix = vec![0u32; limit + 1];
    let fourth = isqrt(limit as i64) as usize;
    for a in 1..=fourth {
        let a_sq = a * a;
        prefix[a_sq] += 1;
        if 2 * a_sq <= limit {
            prefix[2 * a_sq] += 1;
        }
        let max_b = (a - 1).min(isqrt((limit - a_sq) as i64) as usize);
        let mut b_sq = 1usize;
        let mut two_b_plus_1 = 3usize;
        for _ in 1..=max_b {
            prefix[a_sq + b_sq] += 2;
            b_sq += two_b_plus_1;
            two_b_plus_1 += 2;
        }
    }
    for m in 1..=limit {
        prefix[m] += prefix[m - 1];
    }

    let m_limit = 50_000usize;
    let mut small_pairs = Vec::new();
    for m in 2..=m_limit {
        let r_m = prefix[m] - prefix[m - 1];
        if r_m > 0 {
            small_pairs.push((m as i64, r_m as i64));
        }
    }

    let pre = Precomputed {
        prefix,
        small_pairs,
    };

    let mut small_cache = vec![-1i64; 250_001];
    let mut cache: FxHashMap<i64, i64> = FxHashMap::default();
    let ans = f(N, &pre, &mut small_cache, &mut cache);
    println!("{}", ans);
}
