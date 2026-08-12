// Problem 988: Non-attacking Frogs
// Ported from the Python reference: DP over Ferrers diagram column heights for F(a,b).

use std::collections::HashMap;

fn frog_sum(mut a: i64, mut b: i64) -> i64 {
    if a <= 0 || b <= 0 {
        panic!("a and b must be positive");
    }

    if a > b {
        std::mem::swap(&mut a, &mut b);
    }

    if a == 1 {
        return 0;
    }

    let width = (b - 1) as usize;

    // h[i] = number of cells in column i of the Ferrers diagram (1-indexed).
    let mut h = vec![0i64; width + 1];
    for i in 1..=width {
        h[i] = (a * b - a * (i as i64) - 1) / b;
    }

    // dp[height] = (count_of_prefixes, total_weight_of_finished_maximal_elements)
    let mut dp: HashMap<i64, (i64, i64)> = HashMap::new();
    for t in 0..=h[1] {
        dp.insert(t, (1, 0));
    }

    for i in 2..=width {
        let mut next_dp: HashMap<i64, (i64, i64)> = HashMap::new();
        for (&prev_height, &(count, total)) in &dp {
            let limit = prev_height.min(h[i]);
            for cur_height in 0..=limit {
                let add = if prev_height > cur_height && prev_height > 0 {
                    a * b - a * ((i as i64) - 1) - b * prev_height
                } else {
                    0
                };
                let entry = next_dp.entry(cur_height).or_insert((0, 0));
                entry.0 += count;
                entry.1 += total + count * add;
            }
        }
        dp = next_dp;
    }

    // Final sentinel column of height 0 closes the last maximal element, if any.
    let mut answer = 0i64;
    let last_column = width as i64;
    for (&prev_height, &(count, total)) in &dp {
        let add = if prev_height > 0 {
            a * b - a * last_column - b * prev_height
        } else {
            0
        };
        answer += total + count * add;
    }

    answer
}

fn main() {
    debug_assert_eq!(frog_sum(3, 5), 23);
    debug_assert_eq!(frog_sum(5, 13), 16336);
    println!("{}", frog_sum(19, 53));
}
