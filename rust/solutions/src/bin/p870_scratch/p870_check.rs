// Carefully verify the recurrence against the brute-force game analysis.
// Test at r = 3.5 (11/2).

fn compute_l_brute(r_num: u64, r_den: u64, max_n: usize) -> Vec<usize> {
    // Full game-theoretic analysis.
    // State: (remaining, last_move).
    // is_winning[remaining][last] = can current player win?
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

    let mut result = Vec::new();
    result.push(1); // n=1 always loses for player 1
    for n in 2..=max_n {
        // n in L(r) iff for ALL k in [1, n-1], player 2 wins from (n-k, k)
        // i.e., is_winning[n-k][k] = true for all k
        let all_p2_wins = (1..n).all(|k| is_winning[n - k][k]);
        if all_p2_wins {
            result.push(n);
        }
    }
    result
}

fn compute_l_recurrence(rn: u64, rd: u64, count: usize) -> Vec<u64> {
    let mut p: Vec<u64> = vec![1];
    for _ in 1..count {
        let pk = *p.last().unwrap();
        let target = pk as u128 * rd as u128;
        let mut m = 0;
        while m < p.len() {
            if rn as u128 * p[m] as u128 >= target {
                break;
            }
            m += 1;
        }
        if m >= p.len() {
            panic!("Failed at pk={}", pk);
        }
        p.push(pk + p[m]);
    }
    p
}

fn main() {
    let test_cases: Vec<(u64, u64, &str)> = vec![
        (1, 1, "r=1"),
        (2, 1, "r=2"),
        (5, 2, "r=2.5"),
        (3, 1, "r=3"),
        (7, 2, "r=3.5"),
        (11, 3, "r=11/3"),
        (4, 1, "r=4"),
        (9, 2, "r=4.5"),
        (5, 1, "r=5"),
        (6, 1, "r=6"),
        (7, 1, "r=7"),
        (8, 1, "r=8"),
        (10, 1, "r=10"),
    ];

    let max_n = 100;

    for (rn, rd, label) in test_cases {
        let brute = compute_l_brute(rn, rd, max_n);
        let rec = compute_l_recurrence(rn, rd, brute.len());

        let match_all = brute.iter().zip(rec.iter()).all(|(&a, &b)| a as u64 == b);
        let status = if match_all && brute.len() == rec.len() { "OK" } else { "MISMATCH" };

        println!("{}: brute={:?}", label, brute);
        println!("    rec  ={:?}", rec);
        println!("    Status: {}", status);
        println!();
    }
}
