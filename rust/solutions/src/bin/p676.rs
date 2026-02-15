// Project Euler 676 - Matching Digit Sums
// Digit DP with LCM-based chunking for M(10^16, k, l).

use std::collections::HashMap;

fn gcd_f(a: i32, b: i32) -> i32 { if b == 0 { a } else { gcd_f(b, a % b) } }

fn get_diff(v: usize, k: i32, l: i32, lcm_val: i32) -> i32 {
    let mask_k = (1 << k) - 1;
    let mask_l = (1 << l) - 1;
    let mut sk = 0i32;
    let mut sl = 0i32;
    for i in 0..(lcm_val / k) { sk += ((v >> (i * k) as usize) as i32) & mask_k; }
    for i in 0..(lcm_val / l) { sl += ((v >> (i * l) as usize) as i32) & mask_l; }
    sl - sk
}

fn m_func(n: i64, k: i32, l: i32) -> i64 {
    let g = gcd_f(k, l);
    let lcm_val = k * l / g;
    let max_v = 1usize << lcm_val;
    let diff_lookup: Vec<i32> = (0..max_v).map(|v| get_diff(v, k, l, lcm_val)).collect();
    let mut delta_stats: HashMap<i32, (i64, i64)> = HashMap::new();
    for d in 0..max_v {
        let e = delta_stats.entry(diff_lookup[d]).or_insert((0, 0));
        e.0 += 1; e.1 += d as i64;
    }
    let mut digits = Vec::new();
    let mut nn = n as u64;
    while nn > 0 { digits.push((nn % max_v as u64) as usize); nn /= max_v as u64; }
    digits.reverse();
    if digits.is_empty() { return 0; }
    let mut tight: HashMap<i32, (i64, i64)> = HashMap::new();
    let mut free_map: HashMap<i32, (i64, i64)> = HashMap::new();
    tight.insert(0, (1, 0));
    let mut pos_val = 1i64;
    for _ in 0..digits.len()-1 { pos_val *= max_v as i64; }
    for pos in 0..digits.len() {
        let limit = digits[pos];
        let mut new_tight: HashMap<i32, (i64, i64)> = HashMap::new();
        let mut new_free: HashMap<i32, (i64, i64)> = HashMap::new();
        for (&old_diff, &(cnt, sm)) in &tight {
            if cnt == 0 { continue; }
            for d in 0..=limit {
                let new_diff = old_diff + diff_lookup[d];
                let contrib = d as i64 * pos_val;
                if d < limit {
                    let e = new_free.entry(new_diff).or_insert((0, 0));
                    e.0 += cnt; e.1 += sm + cnt * contrib;
                } else {
                    let e = new_tight.entry(new_diff).or_insert((0, 0));
                    e.0 += cnt; e.1 += sm + cnt * contrib;
                }
            }
        }
        for (&old_diff, &(cnt, sm)) in &free_map {
            if cnt == 0 { continue; }
            for (&delta, &(dc, ds)) in &delta_stats {
                let new_diff = old_diff + delta;
                let e = new_free.entry(new_diff).or_insert((0, 0));
                e.0 += cnt * dc; e.1 += dc * sm + cnt * ds * pos_val;
            }
        }
        tight = new_tight;
        free_map = new_free;
        pos_val /= max_v as i64;
    }
    let mut total = 0i64;
    if let Some(&(_, sm)) = tight.get(&0) { total += sm; }
    if let Some(&(_, sm)) = free_map.get(&0) { total += sm; }
    total
}

fn main() {
    let mut total = 0i64;
    for k in 3..=6 {
        for l in 1..k-1 {
            total += m_func(10_000_000_000_000_000, k, l);
        }
    }
    let mod16 = 10_000_000_000_000_000i64;
    total = ((total % mod16) + mod16) % mod16;
    println!("{}", total);
}
