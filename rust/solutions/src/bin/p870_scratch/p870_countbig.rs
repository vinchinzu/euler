// Count transitions between r=1 and r=5 with max_n = 500 by brute force.
// Use only integer (a/1) and half-integer (a/2) transitions to speed up.
// Then compare with recurrence-based algorithm.

fn compute_l_brute(r_num: u64, r_den: u64, max_n: usize) -> Vec<usize> {
    let size = (max_n + 1) * (max_n + 1);
    let mut is_winning = vec![false; size];
    let idx = |rem: usize, last: usize| -> usize { rem * (max_n + 1) + last };

    for remaining in 1..=max_n {
        for last in 1..=max_n {
            let max_take = ((r_num * last as u64) / r_den) as usize;
            let max_take = max_take.min(remaining);
            let mut can_win = false;
            for take in 1..=max_take {
                if !is_winning[idx(remaining - take, take)] {
                    can_win = true;
                    break;
                }
            }
            is_winning[idx(remaining, last)] = can_win;
        }
    }

    let mut result = vec![1usize];
    for n in 2..=max_n {
        let all_p2_wins = (1..n).all(|k| is_winning[idx(n - k, k)]);
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
    // Count transitions between r=2 and r=3 with increasing max_n
    // to see if the count converges
    let max_den = 50u64;

    for &max_n in &[50usize, 100, 200, 300, 400, 500] {
        let mut rats: Vec<(u64, u64)> = Vec::new();
        for d in 1..=max_den {
            for n in (2*d)..=(3*d) {
                let g = gcd(n, d);
                rats.push((n/g, d/g));
            }
        }
        rats.sort_by(|a, b| (a.0 as u128 * b.1 as u128).cmp(&(b.0 as u128 * a.1 as u128)));
        rats.dedup();

        let mut prev = compute_l_brute(rats[0].0, rats[0].1, max_n);
        let mut count = 0;
        for i in 1..rats.len() {
            let l = compute_l_brute(rats[i].0, rats[i].1, max_n);
            if l != prev { count += 1; }
            prev = l;
        }
        println!("max_n={}, max_den={}: {} transitions between 2 and 3", max_n, max_den, count);
    }
}
