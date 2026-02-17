// Project Euler Problem 828: Numbers Challenge
// Compute sum of min scores for all targets

use std::collections::HashSet;

fn read_file() -> Vec<(i64, Vec<i64>)> {
    let data = include_str!("../../../../data/0828_number_challenges.txt");

    let mut result = Vec::new();

    for line in data.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Format: "target:n1,n2,n3,n4,n5,n6"
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

fn recursive_generate(
    a: &[i64],
    memo: &mut std::collections::HashMap<Vec<i64>, HashSet<i64>>,
) -> HashSet<i64> {
    let key = a.to_vec();
    if let Some(result) = memo.get(&key) {
        return result.clone();
    }

    let mut values = HashSet::new();
    if a.len() == 1 {
        values.insert(a[0]);
        memo.insert(key, values.clone());
        return values;
    }

    // Check for duplicates
    let mut freq = std::collections::HashMap::new();
    for &x in a {
        *freq.entry(x).or_insert(0) += 1;
    }
    let multiples_flag = freq.values().any(|&v| v > 1);

    for k in 1..a.len() {
        let combs1: Vec<Vec<i64>> = combinations(a, k);
        let combs2: Vec<Vec<i64>> = combinations(a, a.len() - k);

        for c1 in &combs1 {
            for c2 in &combs2 {
                let set1: HashSet<i64> = c1.iter().cloned().collect();
                let set2: HashSet<i64> = c2.iter().cloned().collect();
                let intersection: HashSet<_> = set1.intersection(&set2).collect();

                let mut flag = false;
                if intersection.is_empty() {
                    flag = true;
                } else if multiples_flag {
                    flag = true;
                    for &x in &intersection {
                        let xcount = c1.iter().filter(|&&v| v == *x).count()
                            + c2.iter().filter(|&&v| v == *x).count();
                        if xcount > freq[x] {
                            flag = false;
                            break;
                        }
                    }
                }

                if flag {
                    let t1 = recursive_generate(c1, memo);
                    let t2 = recursive_generate(c2, memo);

                    for &v1 in &t1 {
                        for &v2 in &t2 {
                            values.insert(v1 + v2);
                            values.insert(v1 * v2);
                            if v1 > v2 {
                                values.insert(v1 - v2);
                            }
                            if v2 != 0 && v1 % v2 == 0 {
                                values.insert(v1 / v2);
                            }
                        }
                    }
                }
            }
        }
    }

    memo.insert(key, values.clone());
    values
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
    let mut memo = std::collections::HashMap::new();
    let mut min_sum = i64::MAX;

    for k in 1..=numbers.len() {
        let combs = combinations(numbers, k);
        for combo in &combs {
            let values = recursive_generate(combo, &mut memo);
            if values.contains(&target) {
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
    let mut total = 0i64;

    for (n, (target, numbers)) in data.iter().enumerate() {
        let s = min_score(*target, numbers);
        total = (total + mod_pow(3, (n + 1) as i64, mod_val) * s) % mod_val;
    }

    total
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
