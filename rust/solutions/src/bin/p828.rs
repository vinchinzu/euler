// Project Euler Problem 828: Numbers Challenge
// Compute sum of min scores for all targets

use rayon::prelude::*;
use std::collections::HashMap;

fn read_file() -> Vec<(i64, Vec<i64>)> {
    let data = include_str!("../../../../data/0828_number_challenges.txt");

    let mut result = Vec::new();

    for line in data.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut split = line.splitn(2, ':');
        let target_str = match split.next() {
            Some(s) => s,
            None => continue,
        };
        let nums_str = match split.next() {
            Some(s) => s,
            None => continue,
        };
        let target = match target_str.parse::<i64>() {
            Ok(t) => t,
            Err(_) => continue,
        };
        let numbers: Vec<i64> = nums_str
            .split(',')
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();
        result.push((target, numbers));
    }

    result
}

fn compute_key(a: &[i64]) -> Vec<i64> {
    let mut sorted = a.to_vec();
    sorted.sort_unstable();
    sorted
}

fn recursive_generate(
    a: &[i64],
    memo: &mut HashMap<Vec<i64>, Vec<i64>>,
) -> Vec<i64> {
    let key = compute_key(a);
    if let Some(result) = memo.get(&key) {
        return result.clone();
    }

    if a.len() == 1 {
        let values = vec![a[0]];
        memo.insert(key, values.clone());
        return values;
    }

    let mut freq: HashMap<i64, u8> = HashMap::new();
    for &x in a {
        *freq.entry(x).or_insert(0) += 1;
    }
    let multiples_flag = freq.values().any(|&v| v > 1);

    let mut all_values = Vec::with_capacity(10000);
    let mut count1: HashMap<i64, u8> = HashMap::with_capacity(8);
    let mut count2: HashMap<i64, u8> = HashMap::with_capacity(8);
    
    for k in 1..a.len() {
        let combs1 = combinations(a, k);
        let combs2 = combinations(a, a.len() - k);

        for c1 in &combs1 {
            for c2 in &combs2 {
                let valid = if multiples_flag {
                    count1.clear();
                    count2.clear();
                    for &x in c1 {
                        *count1.entry(x).or_insert(0) += 1;
                    }
                    for &x in c2 {
                        *count2.entry(x).or_insert(0) += 1;
                    }
                    
                    count1.iter().all(|(&x, &c1_count)| {
                        count2.get(&x).map_or(true, |&c2_count| c1_count + c2_count <= freq[&x])
                    })
                } else {
                    c1.iter().all(|x| c2.iter().all(|y| x != y))
                };

                if valid {
                    let t1 = recursive_generate(c1, memo);
                    let t2 = recursive_generate(c2, memo);

                    for &v1 in &t1 {
                        for &v2 in &t2 {
                            all_values.push(v1 + v2);
                            all_values.push(v1 * v2);
                            if v1 > v2 {
                                all_values.push(v1 - v2);
                            }
                            if v2 != 0 && v1 % v2 == 0 {
                                all_values.push(v1 / v2);
                            }
                        }
                    }
                }
            }
        }
    }

    all_values.sort_unstable();
    all_values.dedup();
    memo.insert(key, all_values.clone());
    all_values
}

fn combinations(arr: &[i64], k: usize) -> Vec<Vec<i64>> {
    if k == 0 {
        return vec![Vec::new()];
    }
    if arr.len() < k {
        return Vec::new();
    }

    let mut result = Vec::new();
    fn helper(
        arr: &[i64],
        k: usize,
        start: usize,
        current: &mut Vec<i64>,
        result: &mut Vec<Vec<i64>>,
    ) {
        if current.len() == k {
            result.push(current.clone());
            return;
        }
        for i in start..arr.len() {
            current.push(arr[i]);
            helper(arr, k, i + 1, current, result);
            current.pop();
        }
    }

    let mut current = Vec::new();
    helper(arr, k, 0, &mut current, &mut result);
    result
}

fn min_score(target: i64, numbers: &[i64]) -> i64 {
    let mut memo = HashMap::new();
    let mut min_sum = i64::MAX;

    for k in 1..=numbers.len() {
        let combs = combinations(numbers, k);
        for combo in &combs {
            let values = recursive_generate(combo, &mut memo);
            if values.binary_search(&target).is_ok() {
                let sum: i64 = combo.iter().sum();
                if sum < min_sum {
                    min_sum = sum;
                }
            }
        }
    }

    if min_sum == i64::MAX {
        0
    } else {
        min_sum
    }
}

fn compute() -> i64 {
    let data = read_file();
    let mod_val = 1005075251i64;

    // Each challenge independent (work varies) — parallelize
    data.par_iter()
        .enumerate()
        .map(|(n, (target, numbers))| {
            let s = min_score(*target, numbers);
            mod_pow(3, (n + 1) as i64, mod_val) * s % mod_val
        })
        .sum::<i64>()
        % mod_val
}

fn mod_pow(mut a: i64, mut e: i64, m: i64) -> i64 {
    let mut r = 1i64;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % m;
        }
        a = a * a % m;
        e >>= 1;
    }
    r
}

fn main() {
    // Test: min_score(211, [2, 3, 4, 6, 7, 25]) = 40
    assert_eq!(min_score(211, &[2, 3, 4, 6, 7, 25]), 40);

    println!("{}", compute());
}
