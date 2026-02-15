// Project Euler 384: Rudin-Shapiro sequence
use std::collections::HashMap;

fn bit_length(n: i64) -> u32 {
    64 - n.leading_zeros()
}

fn s_func(n: i64, cache: &mut HashMap<i64, i64>) -> i64 {
    if n <= 1 { return n + 1; }
    if let Some(&v) = cache.get(&n) { return v; }

    let x = 1i64 << (bit_length(n) - 1);
    let result = if n >= x + x / 2 {
        s_func(x - 1, cache) + 2 * s_func(x / 2 - 1, cache) - s_func(n - x, cache)
    } else {
        s_func(x - 1, cache) + s_func(n - x, cache)
    };
    cache.insert(n, result);
    result
}

fn isqrt_ll(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut x = 1i64;
    while x * x <= n && x < (1i64 << 31) { x *= 2; }
    let mut lo = x / 2;
    let mut hi = x;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if mid <= n / mid { lo = mid + 1; }
        else { hi = mid; }
    }
    lo - 1
}

fn count_func(n: i64, val: i64, s_cache: &mut HashMap<i64, i64>, c_cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    if val < 0 || (n >= 0 && val > 6 * isqrt_ll(n + 1)) { return 0; }
    if n == -1 { return if val == 0 { 1 } else { 0 }; }

    if let Some(&v) = c_cache.get(&(n, val)) { return v; }

    let x = 1i64 << (bit_length(n + 1) - 1);
    let mut c = count_func(x - 2, val, s_cache, c_cache);

    if n >= x + x / 2 {
        let s_x = s_func(x - 1, s_cache);
        let s_mid = s_func(x + x / 2 - 1, s_cache);
        let mirror_val = 2 * s_mid - s_x - val;
        c += count_func(x / 2 - 2, val - s_x, s_cache, c_cache)
           + count_func(n - x, mirror_val, s_cache, c_cache)
           - count_func(x / 2 - 2, mirror_val, s_cache, c_cache);
    } else {
        c += count_func(n - x, val - s_func(x - 1, s_cache), s_cache, c_cache);
    }

    c_cache.insert((n, val), c);
    c
}

fn g_func(t: i64, c_target: i64, s_cache: &mut HashMap<i64, i64>, c_cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    let mut hi = 1i64;
    while count_func(hi, t, s_cache, c_cache) < c_target {
        if hi > (1i64 << 62) { break; }
        hi *= 2;
    }

    let mut lo = 0i64;
    while lo + 1 < hi {
        let mid = lo + (hi - lo) / 2;
        if count_func(mid, t, s_cache, c_cache) < c_target {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    hi
}

fn main() {
    let mut fib = [0i64; 50];
    fib[0] = 0; fib[1] = 1;
    for i in 2..50 { fib[i] = fib[i - 1] + fib[i - 2]; }

    let mut total = 0i64;
    for t in 2..=45 {
        let mut s_cache: HashMap<i64, i64> = HashMap::new();
        let mut c_cache: HashMap<(i64, i64), i64> = HashMap::new();
        let result = g_func(fib[t + 1], fib[t], &mut s_cache, &mut c_cache);
        total += result;
    }
    println!("{}", total);
}
