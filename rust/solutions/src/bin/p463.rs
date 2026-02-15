// Project Euler 463: A weird recurrence relation
use std::collections::HashMap;

const M: i64 = 1_000_000_000;

fn sum_f(n: u64, cache_sf: &mut HashMap<u64, i64>, cache_sof: &mut HashMap<u64, i64>) -> i64 {
    if n == 0 { return 0; }
    if let Some(&v) = cache_sf.get(&n) { return v; }

    let mut result: i64 = 0;
    if n >= 1 { result += 1; }
    if n >= 2 { result += 1; }
    if n >= 3 { result += 3; }
    if n >= 4 {
        result += sum_f(n / 4, cache_sf, cache_sof);
        result += 2 * sum_odd_f((n - 1) / 4, cache_sf, cache_sof);
        result -= sum_f((n - 1) / 4, cache_sf, cache_sof);
        result += sum_odd_f((n - 2) / 4, cache_sf, cache_sof);
        result += 3 * sum_odd_f((n - 3) / 4, cache_sf, cache_sof);
        result -= 2 * sum_f((n - 3) / 4, cache_sf, cache_sof);
    }
    result = ((result % M) + M) % M;
    cache_sf.insert(n, result);
    result
}

fn sum_odd_f(n: u64, cache_sf: &mut HashMap<u64, i64>, cache_sof: &mut HashMap<u64, i64>) -> i64 {
    if n == 0 { return 0; }
    if let Some(&v) = cache_sof.get(&n) { return v; }

    let mut result: i64 = 0;
    if n >= 1 { result += 3; }
    if n >= 2 {
        result += 2 * sum_odd_f(n / 2, cache_sf, cache_sof);
        result -= sum_f(n / 2, cache_sf, cache_sof);
        result += 3 * sum_odd_f((n - 1) / 2, cache_sf, cache_sof);
        result -= 2 * sum_f((n - 1) / 2, cache_sf, cache_sof);
    }
    result = ((result % M) + M) % M;
    cache_sof.insert(n, result);
    result
}

fn main() {
    let mut big_n: u64 = 1;
    for _ in 0..37 { big_n *= 3; }

    let mut cache_sf = HashMap::new();
    let mut cache_sof = HashMap::new();
    let ans = sum_f(big_n, &mut cache_sf, &mut cache_sof);
    println!("{ans}");
}
