// Count transitions between 1 and 10 with max_n = 500 and max_den = 500
// This takes a while but should give a definitive count.

fn compute_l_brute_flat(r_num: u64, r_den: u64, max_n: usize) -> Vec<usize> {
    // Use flat array for performance
    let nn = max_n + 1;
    let mut is_winning = vec![false; nn * nn];

    for remaining in 1..=max_n {
        let base = remaining * nn;
        for last in 1..=max_n {
            let max_take = ((r_num * last as u64) / r_den) as usize;
            let max_take = max_take.min(remaining);
            let mut can_win = false;
            for take in 1..=max_take {
                if !is_winning[(remaining - take) * nn + take] {
                    can_win = true;
                    break;
                }
            }
            is_winning[base + last] = can_win;
        }
    }

    let mut result = vec![1usize];
    for n in 2..=max_n {
        let all_p2_wins = (1..n).all(|k| is_winning[(n - k) * nn + k]);
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
    let max_n = 300;
    let max_den = 300u64;

    // Build sorted rationals from 1 to 5
    let mut rats: Vec<(u64, u64)> = Vec::new();
    for d in 1..=max_den {
        for n in d..=(5 * d) {
            let g = gcd(n, d);
            rats.push((n / g, d / g));
        }
    }
    rats.sort_by(|a, b| (a.0 as u128 * b.1 as u128).cmp(&(b.0 as u128 * a.1 as u128)));
    rats.dedup();

    eprintln!("Checking {} rationals from 1 to 5 (max_n={})...", rats.len(), max_n);

    let mut prev = compute_l_brute_flat(rats[0].0, rats[0].1, max_n);
    let mut count = 0;
    let mut transitions: Vec<(u64, u64)> = Vec::new();

    for i in 1..rats.len() {
        let (a, b) = rats[i];
        let l = compute_l_brute_flat(a, b, max_n);
        if l != prev {
            count += 1;
            transitions.push((a, b));
        }
        prev = l;
    }

    println!("Total transitions from 1 to 5 (max_n={}, max_den={}): {}", max_n, max_den, count);
    for (i, &(a, b)) in transitions.iter().enumerate() {
        if i < 50 {
            println!("  T({}) = {}/{} = {:.10}", i + 1, a, b, a as f64 / b as f64);
        }
    }
}
