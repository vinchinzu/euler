use std::collections::HashMap;

const MODULUS: i64 = 1_000_000_000;

fn mod_pow_val(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 != 0 { result = result * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn first_index_with_len(length: i64, cache: &mut HashMap<i64, i64>) -> i64 {
    if length <= 3 { return 1i64 << (length - 1); }
    if let Some(&v) = cache.get(&length) { return v; }
    let result = if length % 2 == 0 {
        first_index_with_len(length / 2, cache) + 3 * first_index_with_len(length / 2 + 1, cache) - 7
    } else {
        3 * first_index_with_len(length / 2 + 1, cache) + first_index_with_len(length / 2 + 2, cache) - 7
    };
    cache.insert(length, result);
    result
}

fn num_values_with_len(length: i64, cache: &mut HashMap<i64, i64>) -> i64 {
    first_index_with_len(length + 1, cache) - first_index_with_len(length, cache)
}

fn position_in_t(length: i64, index: i64, cache: &mut HashMap<i64, i64>) -> i64 {
    if length <= 1 { return length; }
    let nv = num_values_with_len((length + 1) / 2, cache);
    if index < nv {
        let position = position_in_t((length + 1) / 2, index, cache);
        position * 2
    } else {
        let nvl = num_values_with_len(length, cache);
        let position = position_in_t(length / 2 + 1, nvl - index - 1, cache);
        let val = position + length - 1;
        let mut highest = 0i64;
        if val > 0 {
            highest = 1;
            while highest * 2 <= val { highest *= 2; }
        }
        (position + highest * 2) * 2 + 1
    }
}

fn highest_pow2_leq(n: i64) -> i64 {
    let mut h = 1i64;
    while h * 2 <= n { h *= 2; }
    h
}

fn prefix_of_t(length: i64, pcache: &mut HashMap<i64, i64>) -> i64 {
    if length <= 0 { return 0; }
    if length == 1 { return 0; }
    if let Some(&v) = pcache.get(&length) { return v; }
    let half = highest_pow2_leq(length - 1);
    let result = ((prefix_of_t(half, pcache) + 1) % MODULUS * mod_pow_val(2, length - half, MODULUS) % MODULUS
        - prefix_of_t(length - half, pcache) - 1 + 2 * MODULUS) % MODULUS;
    pcache.insert(length, result);
    result
}

fn compute_a(n: i64, fiwl_cache: &mut HashMap<i64, i64>, pcache: &mut HashMap<i64, i64>) -> i64 {
    let mut low = 0i64;
    let mut high = 10_000_000_000i64;
    while low + 1 < high {
        let mid = (low + high) / 2;
        if first_index_with_len(mid, fiwl_cache) > n {
            high = mid;
        } else {
            low = mid;
        }
    }
    let length = low;
    let position = position_in_t(length, n - first_index_with_len(length, fiwl_cache), fiwl_cache);
    (prefix_of_t(position + length, pcache)
        - prefix_of_t(position, pcache) % MODULUS * mod_pow_val(2, length, MODULUS) % MODULUS
        + 2 * MODULUS * MODULUS) % MODULUS
}

fn main() {
    let mut fiwl_cache = HashMap::new();
    let mut pcache = HashMap::new();

    let mut ans = 0i64;
    let mut power = 10i64;
    for _ in 1..=18 {
        ans = (ans + compute_a(power, &mut fiwl_cache, &mut pcache)) % MODULUS;
        power *= 10;
    }
    println!("{}", ans);
}
