// Problem 949 - Left vs Right II
// G(20, 7) mod 1001001011
//
// Port of the Python reference using game theory with dyadic rationals.
// For each word of length n (encoded as bit-string of L=0/R=1), we compute
// a game value via DP over all subwords. Then we count k-tuples where Right wins.

use std::collections::HashMap;

const MOD: u64 = 1_001_001_011;

/// Ceiling division of x by 2^s.
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

/// Compute game values (u) and hot/cold flags for all 2^n words of length n.
/// Returns (u_full, hot) where u_full[bits] is the game value for word `bits`
/// and hot[bits] is true if the position is "hot" (confused).
fn compute_u_hot(n: u32) -> (Vec<i64>, Vec<bool>) {
    let e = n;
    let scale = 1i64 << e;
    let total = (1usize << (n + 1)) - 1;
    let mut dp_u = vec![0i64; total];
    let mut dp_d = vec![0i64; total];

    // Base case: length-1 words
    // Index scheme: words of length L start at index (1<<L)-1.
    // For length 1: start=1, bit 0 -> index 1, bit 1 -> index 2
    let start1: usize = 1;
    dp_u[start1] = scale;      // L -> value +1 (scaled)
    dp_d[start1] = scale;
    dp_u[start1 + 1] = -scale; // R -> value -1 (scaled)
    dp_d[start1 + 1] = -scale;

    let num_words = 1usize << n;
    let mut hot = vec![false; num_words];

    for length in 2..=n {
        let size = 1usize << length;
        let start = (1usize << length) - 1;
        for bits in 0..size {
            // u_raw = max over all proper suffixes of dp_d[suffix]
            let mut u_raw = i64::MIN;
            for s_len in 1..length {
                let suf = bits & ((1usize << s_len) - 1);
                let cand = dp_d[((1usize << s_len) - 1) + suf];
                if cand > u_raw {
                    u_raw = cand;
                }
            }
            // d_raw = min over all proper prefixes of dp_u[prefix]
            let mut d_raw = i64::MAX;
            for p_len in 1..length {
                let pre = bits >> (length - p_len);
                let cand = dp_u[((1usize << p_len) - 1) + pre];
                if cand < d_raw {
                    d_raw = cand;
                }
            }
            let idx = start + bits;
            if u_raw < d_raw {
                let x = simplest_between(u_raw, d_raw, e);
                dp_u[idx] = x;
                dp_d[idx] = x;
                if length == n {
                    hot[bits] = false; // cold
                }
            } else {
                dp_u[idx] = u_raw;
                dp_d[idx] = d_raw;
                if length == n {
                    hot[bits] = true; // hot
                }
            }
        }
    }

    let start_n = (1usize << n) - 1;
    let u_full = dp_u[start_n..start_n + num_words].to_vec();
    (u_full, hot)
}

/// Build a histogram: value -> count (mod MOD).
fn hist_from_values(values: &[i64], modulus: u64) -> HashMap<i64, u64> {
    let mut hist: HashMap<i64, u64> = HashMap::new();
    for &v in values {
        let e = hist.entry(v).or_insert(0);
        *e = (*e + 1) % modulus;
    }
    hist.retain(|_, c| *c != 0);
    hist
}

/// Sparse convolution of two histograms (sum of independent random variables).
fn convolve(a: &HashMap<i64, u64>, b: &HashMap<i64, u64>, modulus: u64) -> HashMap<i64, u64> {
    if a.is_empty() || b.is_empty() {
        return HashMap::new();
    }
    // Iterate over the smaller one in the outer loop
    let (small, large) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    let mut out: HashMap<i64, u64> = HashMap::with_capacity(small.len() * large.len());
    for (&xa, &ca) in small.iter() {
        for (&xb, &cb) in large.iter() {
            let k = xa + xb;
            let prod = (ca as u128 * cb as u128 % modulus as u128) as u64;
            let e = out.entry(k).or_insert(0);
            *e = (*e + prod) % modulus;
        }
    }
    out.retain(|_, c| *c != 0);
    out
}

/// Repeated convolution: hist convolved with itself t times.
fn pow_small(hist: &HashMap<i64, u64>, t: u32, modulus: u64) -> HashMap<i64, u64> {
    if t == 0 {
        let mut r = HashMap::new();
        r.insert(0, 1u64);
        return r;
    }
    let mut d = hist.clone();
    for _ in 1..t {
        d = convolve(&d, hist, modulus);
    }
    d
}

/// Count pairs (sa, sb) with sa in a, sb in b such that sa + sb < 0,
/// weighted by counts, mod modulus.
fn count_sum_lt_zero(a: &HashMap<i64, u64>, b: &HashMap<i64, u64>, modulus: u64) -> u64 {
    let mut b_items: Vec<(i64, u64)> = b.iter().map(|(&s, &c)| (s, c)).collect();
    b_items.sort_by_key(|&(s, _)| s);
    let b_sums: Vec<i64> = b_items.iter().map(|&(s, _)| s).collect();

    // prefix sums of counts
    let mut pref = Vec::with_capacity(b_items.len() + 1);
    pref.push(0u64);
    let mut run = 0u64;
    for &(_, c) in &b_items {
        run = (run + c) % modulus;
        pref.push(run);
    }

    let mut ans = 0u64;
    for (&sa, &ca) in a.iter() {
        // We need sb < -sa, i.e., sb in b_sums where sb < -sa
        let target = -sa;
        let idx = b_sums.partition_point(|&x| x < target);
        let count_b = pref[idx];
        ans = (ans + (ca as u128 * count_b as u128 % modulus as u128) as u64) % modulus;
    }
    ans
}

/// Count pairs (sa, sb) with sa in a, sb in b such that sa + sb == 0,
/// weighted by counts, mod modulus.
fn count_sum_eq_zero(a: &HashMap<i64, u64>, b: &HashMap<i64, u64>, modulus: u64) -> u64 {
    let (small, large) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    let mut ans = 0u64;
    for (&s, &ca) in small.iter() {
        if let Some(&cb) = large.get(&(-s)) {
            ans = (ans + (ca as u128 * cb as u128 % modulus as u128) as u64) % modulus;
        }
    }
    ans
}

/// Compute G(n, k) mod modulus.
fn g(n: u32, k: u32, modulus: u64) -> u64 {
    assert!(k % 2 == 1, "k must be odd");
    assert!(n > 0, "n must be positive");

    let (u_full, hot) = compute_u_hot(n);

    let u_hist = hist_from_values(&u_full, modulus);

    let cold_values: Vec<i64> = u_full
        .iter()
        .zip(hot.iter())
        .filter(|&(_, &h)| !h)
        .map(|(&v, _)| v)
        .collect();
    let cold_hist = hist_from_values(&cold_values, modulus);

    let a = k / 2;
    let b = k - a;

    let dist_a = pow_small(&u_hist, a, modulus);
    let dist_b = pow_small(&u_hist, b, modulus);
    let neg = count_sum_lt_zero(&dist_a, &dist_b, modulus);

    let cold_a = pow_small(&cold_hist, a, modulus);
    let cold_b = pow_small(&cold_hist, b, modulus);
    let zero_cold = count_sum_eq_zero(&cold_a, &cold_b, modulus);

    (neg + zero_cold) % modulus
}

fn main() {
    println!("{}", g(20, 7, MOD));
}
