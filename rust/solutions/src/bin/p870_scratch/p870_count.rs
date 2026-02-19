// Count ACTUAL transitions by brute force with large max_n
// between r=6 and r=8 with fine granularity.
// Use rational arithmetic for precision.

fn compute_l_brute(r_num: u64, r_den: u64, max_n: usize) -> Vec<usize> {
    let mut is_winning = vec![vec![false; max_n + 1]; max_n + 1];

    for remaining in 1..=max_n {
        for last in 1..=max_n {
            let max_take = ((r_num * last as u64) / r_den) as usize;
            let max_take = max_take.min(remaining);
            let mut can_win = false;
            for take in 1..=max_take {
                if !is_winning[remaining - take][take] {
                    can_win = true;
                    break;
                }
            }
            is_winning[remaining][last] = can_win;
        }
    }

    let mut result = vec![1usize];
    for n in 2..=max_n {
        let all_p2_wins = (1..n).all(|k| is_winning[n - k][k]);
        if all_p2_wins {
            result.push(n);
        }
    }
    result
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    // Count transitions between r=6 and r=7 with denominator up to 200
    // Using max_n = 200 to be more thorough
    let max_n = 200;
    let r_lo = 6u64;
    let r_hi = 7u64;
    let max_den = 200u64;

    // Build sorted rationals
    let mut rats: Vec<(u64, u64)> = Vec::new();
    for d in 1..=max_den {
        for n in (r_lo * d)..=(r_hi * d) {
            let g = gcd(n, d);
            rats.push((n / g, d / g));
        }
    }
    rats.sort_by(|a, b| (a.0 as u128 * b.1 as u128).cmp(&(b.0 as u128 * a.1 as u128)));
    rats.dedup();

    println!("Checking {} rationals between {} and {} (max_n={})...", rats.len(), r_lo, r_hi, max_n);

    let mut prev = compute_l_brute(rats[0].0, rats[0].1, max_n);
    let mut count = 0;
    let mut transitions: Vec<(u64, u64)> = Vec::new();

    for i in 1..rats.len() {
        let (a, b) = rats[i];
        let l = compute_l_brute(a, b, max_n);
        if l != prev {
            count += 1;
            transitions.push((a, b));
            if count <= 20 {
                println!("Transition #{}: r={}/{} = {:.10}", count, a, b, a as f64 / b as f64);
                // Show what changed
                let min_len = prev.len().min(l.len());
                for j in 0..min_len {
                    if prev[j] != l[j] {
                        println!("  First diff at index {}: {} vs {}", j, prev[j], l[j]);
                        break;
                    }
                }
                if prev.len() != l.len() && prev.len() <= l.len() {
                    // Different lengths but same prefix
                    if prev.iter().zip(l.iter()).all(|(a, b)| a == b) {
                        println!("  New longer set: {} -> {} elements", prev.len(), l.len());
                    }
                }
            }
        }
        prev = l;
    }

    println!("\nTotal transitions between {} and {} (max_n={}, max_den={}): {}", r_lo, r_hi, max_n, max_den, count);
    println!("\nFirst 30 transitions:");
    for (i, &(a, b)) in transitions.iter().take(30).enumerate() {
        println!("  T({}) = {}/{} = {:.10}", i + 1, a, b, a as f64 / b as f64);
    }
}
