// Check: does L(r) actually change at the computed transition values?
// Compare with brute force.

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

fn main() {
    let max_n = 100;

    // Check transitions near 173/25 = 6.92 and 7/1
    let candidates = vec![
        (173u64, 25u64), // 6.92
        (346u64, 50u64), // same reduced
        (69301u64, 10000u64), // 6.9301 (just above 6.92)
        (694u64, 100u64), // 6.94
        (696u64, 100u64), // 6.96
        (698u64, 100u64), // 6.98
        (699u64, 100u64), // 6.99
        (6999u64, 1000u64), // 6.999
        (7u64, 1u64), // 7.0
    ];

    for &(a, b) in &candidates {
        let l = compute_l_brute(a, b, max_n);
        println!("L({}/{} = {:.4}) = {:?}", a, b, a as f64 / b as f64, l);
    }

    println!();

    // Check around T(6) = 11/3 = 3.667
    // My algorithm says T(7) = 43/11 = 3.909
    // Brute force found T(6) = 43/11 = 3.909 (same!)
    // But between 11/3 and 43/11, does L change?
    let test_vals = vec![
        (11u64, 3u64), // 3.667
        (37u64, 10u64), // 3.7
        (38u64, 10u64), // 3.8
        (39u64, 10u64), // 3.9
        (43u64, 11u64), // 3.909
    ];

    println!("Around 3.667 - 3.909:");
    for &(a, b) in &test_vals {
        let l = compute_l_brute(a, b, max_n);
        println!("L({}/{} = {:.4}) = {:?}", a, b, a as f64 / b as f64, l);
    }

    println!();

    // Check around T(27) = 34/5 = 6.8, T(28) = 173/25 = 6.92
    // Is there really a transition at 173/25?
    let test_vals2 = vec![
        (34u64, 5u64), // 6.8
        (685u64, 100u64), // 6.85
        (69u64, 10u64), // 6.9
        (691u64, 100u64), // 6.91
        (692u64, 100u64), // 6.92
        (173u64, 25u64), // 6.92 exact
        (693u64, 100u64), // 6.93
        (7u64, 1u64), // 7.0
    ];

    println!("Around 6.8 - 7.0:");
    for &(a, b) in &test_vals2 {
        let l = compute_l_brute(a, b, max_n);
        println!("L({}/{} = {:.4}) = {:?}", a, b, a as f64 / b as f64, l);
    }
}
