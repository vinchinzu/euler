use std::fs;

fn is_special_sum_set(set: &[i32]) -> bool {
    let n = set.len();
    let total_subsets = 1 << n;
    let mut sums = vec![0i32; total_subsets];
    let mut sizes = vec![0usize; total_subsets];

    for mask in 0..total_subsets {
        let mut s = 0;
        let mut sz = 0;
        for i in 0..n {
            if mask & (1 << i) != 0 {
                s += set[i];
                sz += 1;
            }
        }
        sums[mask] = s;
        sizes[mask] = sz;
    }

    for i in 1..total_subsets {
        for j in (i + 1)..total_subsets {
            if i & j != 0 { continue; }
            if sums[i] == sums[j] { return false; }
            if sizes[i] > sizes[j] && sums[i] <= sums[j] { return false; }
            if sizes[j] > sizes[i] && sums[j] <= sums[i] { return false; }
        }
    }
    true
}

fn main() {
    let paths = [
        "solutions/sets.txt",
        "data/sets.txt",
        "../data/sets.txt",
        "../solutions/sets.txt",
    ];
    let mut content = String::new();
    for p in &paths {
        if let Ok(c) = fs::read_to_string(p) {
            content = c;
            break;
        }
    }
    if content.is_empty() {
        println!("0");
        return;
    }

    let mut total_sum = 0i32;
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        let mut nums: Vec<i32> = line.split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        if nums.is_empty() { continue; }
        nums.sort();
        if is_special_sum_set(&nums) {
            total_sum += nums.iter().sum::<i32>();
        }
    }
    println!("{}", total_sum);
}
