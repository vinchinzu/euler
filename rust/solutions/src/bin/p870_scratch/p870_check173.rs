// Check if 173/25 = 6.92 is a real transition.
// Compute L for r slightly below and above 173/25 with max_n = 500.

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

fn main() {
    let max_n = 500;

    eprintln!("Computing L(6919/1000) with max_n=500...");
    let l_below = compute_l_brute(6919, 1000, max_n);
    eprintln!("Computing L(6921/1000) with max_n=500...");
    let l_above = compute_l_brute(6921, 1000, max_n);

    if l_below == l_above {
        println!("L(6.919) == L(6.921) for max_n=500. NOT a transition (at this resolution).");
    } else {
        println!("L(6.919) != L(6.921). IS a transition.");
        let min_len = l_below.len().min(l_above.len());
        for i in 0..min_len {
            if l_below[i] != l_above[i] {
                println!("First diff at index {}: {} vs {}", i, l_below[i], l_above[i]);
                break;
            }
        }
        if l_below.len() != l_above.len() {
            println!("Different lengths: {} vs {}", l_below.len(), l_above.len());
        }
    }

    // Print the elements near the difference
    println!("\nL(6.919) last elements: {:?}", &l_below[l_below.len().saturating_sub(5)..]);
    println!("L(6.921) last elements: {:?}", &l_above[l_above.len().saturating_sub(5)..]);
}
