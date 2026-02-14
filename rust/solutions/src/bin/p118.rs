// Project Euler 118: Pandigital Prime Sets
// Count sets of pandigital primes using digits 1-9 exactly once.

use euler_utils::miller_rabin;

const ALL_MASK: usize = 511; // (1 << 9) - 1

fn main() {
    // For each bitmask of digits, store sorted unique primes formed from those digits
    let mut primes_by_mask: Vec<Vec<u64>> = vec![Vec::new(); 512];

    // Precompute digit lists for each mask
    let mut digit_lists: Vec<Vec<u8>> = vec![Vec::new(); 512];
    for mask in 1..=ALL_MASK {
        for i in 0..9u8 {
            if mask & (1 << i) != 0 {
                digit_lists[mask].push(i + 1);
            }
        }
    }

    // Generate permutations for each mask and check primality
    for mask in 1..=ALL_MASK {
        let mut digits = digit_lists[mask].clone();
        let n = digits.len();
        gen_perms(&mut digits, n, 0, 0, mask, &mut primes_by_mask);

        // Sort and deduplicate
        primes_by_mask[mask].sort_unstable();
        primes_by_mask[mask].dedup();
    }

    // DFS with memoization
    let mut memo = std::collections::HashMap::new();
    let ans = dfs(ALL_MASK, 0, &primes_by_mask, &mut memo);
    println!("{}", ans);
}

fn gen_perms(
    digits: &mut Vec<u8>,
    n: usize,
    depth: usize,
    current: u64,
    mask: usize,
    primes_by_mask: &mut Vec<Vec<u64>>,
) {
    if depth == n {
        if miller_rabin(current) {
            primes_by_mask[mask].push(current);
        }
        return;
    }
    for i in depth..n {
        digits.swap(depth, i);

        // Skip multi-digit numbers ending in even or 5
        if depth == n - 1 && n > 1 {
            let last = digits[depth];
            if last % 2 == 0 || last == 5 {
                digits.swap(depth, i);
                continue;
            }
        }

        gen_perms(digits, n, depth + 1, current * 10 + digits[depth] as u64, mask, primes_by_mask);
        digits.swap(depth, i);
    }
}

fn dfs(
    mask: usize,
    last: u64,
    primes_by_mask: &[Vec<u64>],
    memo: &mut std::collections::HashMap<(usize, u64), i64>,
) -> i64 {
    if mask == 0 {
        return 1;
    }
    if let Some(&v) = memo.get(&(mask, last)) {
        return v;
    }

    let mut total: i64 = 0;
    let mut sub = mask;
    while sub > 0 {
        for &p in &primes_by_mask[sub] {
            if p > last {
                total += dfs(mask ^ sub, p, primes_by_mask, memo);
            }
        }
        sub = (sub - 1) & mask;
    }

    memo.insert((mask, last), total);
    total
}
