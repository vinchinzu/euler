// Large brute-force game analysis at r = 3.0 with max_n = 500
// to verify the recurrence for a larger range.
//
// Optimize: use bit flags for winning/losing and break early.

fn compute_l_brute(r_num: u64, r_den: u64, max_n: usize) -> Vec<usize> {
    // is_winning[remaining][last] packed into a flat array
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
    let max_n = 500;

    for &(rn, rd, label) in &[
        (3u64, 1u64, "r=3"),
        (5u64, 1u64, "r=5"),
        (10u64, 1u64, "r=10"),
        (20u64, 1u64, "r=20"),
    ] {
        eprintln!("Computing L({}) with max_n={} by brute force...", label, max_n);
        let brute = compute_l_brute(rn, rd, max_n);
        let rec = compute_l_recurrence(rn, rd, brute.len());

        let match_all = brute.iter().zip(rec.iter()).all(|(&a, &b)| a as u64 == b);

        if match_all && brute.len() == rec.len() {
            println!("{}: OK (len={})", label, brute.len());
        } else {
            println!("{}: MISMATCH!", label);
            println!("  brute: {:?}", &brute[..brute.len().min(30)]);
            println!("  rec:   {:?}", &rec[..rec.len().min(30)]);
            // Find first mismatch
            for i in 0..brute.len().min(rec.len()) {
                if brute[i] as u64 != rec[i] {
                    println!("  First mismatch at index {}: brute={}, rec={}", i, brute[i], rec[i]);
                    break;
                }
            }
        }
    }
}
