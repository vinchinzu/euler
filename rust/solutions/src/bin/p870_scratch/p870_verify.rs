// Count ALL transitions from T(1)=1 up to some limit by brute force with large max_n.
// Use max_n = 300 and max_den = 300 between r=1 and r=10.

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
    let max_n = 300;
    let max_den = 300u64;

    // Build sorted rationals from 1 to 10
    let mut rats: Vec<(u64, u64)> = Vec::new();
    for d in 1..=max_den {
        for n in d..=(10 * d) {
            let g = gcd(n, d);
            rats.push((n / g, d / g));
        }
    }
    rats.sort_by(|a, b| (a.0 as u128 * b.1 as u128).cmp(&(b.0 as u128 * a.1 as u128)));
    rats.dedup();

    println!("Checking {} rationals from 1 to 10 (max_n={})...", rats.len(), max_n);

    let mut prev = compute_l_brute(rats[0].0, rats[0].1, max_n);
    let mut count = 0;
    let mut transitions: Vec<(u64, u64)> = Vec::new();

    for i in 1..rats.len() {
        let (a, b) = rats[i];
        let l = compute_l_brute(a, b, max_n);
        if l != prev {
            count += 1;
            transitions.push((a, b));
        }
        prev = l;
    }

    println!("Total transitions from 1 to 10 (max_n={}): {}", max_n, count);
    println!("\nAll transitions:");
    for (i, &(a, b)) in transitions.iter().enumerate() {
        println!("T({}) = {}/{} = {:.10}", i + 1, a, b, a as f64 / b as f64);
    }
}
