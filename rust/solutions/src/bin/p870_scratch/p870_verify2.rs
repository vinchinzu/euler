// Quick verification: count transitions between r=1 and r=10
// with smaller max_n and compare with the recurrence-based algorithm.

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
    // Compare transitions found with different max_n values
    for max_n in [50, 100, 150, 200] {
        let max_den = 200u64;

        let mut rats: Vec<(u64, u64)> = Vec::new();
        for d in 1..=max_den {
            for n in d..=(10 * d) {
                let g = gcd(n, d);
                rats.push((n / g, d / g));
            }
        }
        rats.sort_by(|a, b| (a.0 as u128 * b.1 as u128).cmp(&(b.0 as u128 * a.1 as u128)));
        rats.dedup();

        let mut prev = compute_l_brute(rats[0].0, rats[0].1, max_n);
        let mut count = 0;

        for i in 1..rats.len() {
            let (a, b) = rats[i];
            let l = compute_l_brute(a, b, max_n);
            if l != prev {
                count += 1;
            }
            prev = l;
        }

        println!("max_n={}: {} transitions between 1 and 10", max_n, count);
    }
}
