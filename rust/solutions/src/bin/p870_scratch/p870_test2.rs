// Test: find exact transition values by binary search
// A transition occurs when floor(r * P_j) changes for some P_j in the losing sequence,
// causing the recurrence to change.
// Transition values are rationals of the form a/b where a, b are losing positions.

use std::collections::BTreeSet;

fn compute_l(r_num: u64, r_den: u64, max_n: usize) -> Vec<usize> {
    // Use exact rational arithmetic: r = r_num / r_den
    // floor(r * m) = floor(r_num * m / r_den)
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

    let mut result = vec![1usize]; // n=1 always in L(r)
    for n in 2..=max_n {
        let all_p2_wins = (1..n).all(|k| is_winning[n - k][k]);
        if all_p2_wins {
            result.push(n);
        }
    }
    result
}

fn compute_l_via_recurrence(r_num: u64, r_den: u64, count: usize) -> Vec<u64> {
    // Compute losing positions using the recurrence:
    // P_1 = 1
    // P_{n+1} = P_n + P_m where m = min{j : floor(r * P_j) >= P_n}
    // floor(r * P_j) = floor(r_num * P_j / r_den)
    let mut p = vec![1u64];
    for _ in 1..count {
        let pn = *p.last().unwrap();
        // Find m = min j such that floor(r_num * p[j] / r_den) >= pn
        // i.e., r_num * p[j] / r_den >= pn
        // i.e., r_num * p[j] >= pn * r_den
        let target = pn * r_den;
        let mut m = 0;
        while m < p.len() {
            if r_num * p[m] >= target {
                // Check floor: r_num * p[m] / r_den >= pn
                if (r_num * p[m]) / r_den >= pn {
                    break;
                }
            }
            m += 1;
        }
        if m >= p.len() {
            // Fallback: shouldn't happen
            panic!("Could not find m for pn={}", pn);
        }
        p.push(pn + p[m]);
    }
    p
}

fn main() {
    let max_n = 40;

    // Verify recurrence matches brute force for r=2 (Fibonacci)
    println!("Brute force L(2): {:?}", compute_l(2, 1, max_n));
    println!("Recurrence L(2): {:?}", compute_l_via_recurrence(2, 1, 10));

    // For r=1 (powers of 2)
    println!("Brute force L(1): {:?}", compute_l(1, 1, max_n));
    println!("Recurrence L(1): {:?}", compute_l_via_recurrence(1, 1, 10));

    // For r=5/2 = 2.5
    println!("Brute force L(5/2): {:?}", compute_l(5, 2, max_n));
    println!("Recurrence L(5/2): {:?}", compute_l_via_recurrence(5, 2, 10));

    // Now find exact transition values
    // A transition at r=a/b means that for r slightly below a/b, the losing positions
    // differ from those slightly above a/b.
    // This happens when floor(r * P_j) = floor(a/b * P_j) changes.
    // More precisely, floor((a/b - eps) * P_j) < floor(a/b * P_j) for some P_j.
    // This means a*P_j/b is an integer, i.e., b divides a*P_j.
    //
    // The transition values are rationals P_n / P_j where the losing sequence changes.

    // Let me enumerate ALL candidate transition values.
    // For a given recurrence sequence P, transition values are P_i / P_j.
    // But P depends on r...
    //
    // Actually, let me think about this differently. Between transitions, the recurrence
    // is fixed (same m for each step). A transition happens when r crosses P_n/P_j,
    // causing the index m to shift by 1 in the recurrence.

    // Let me just enumerate transitions by checking L(r) for many rationals
    let max_n = 50;
    let mut transitions: Vec<(u64, u64)> = Vec::new();
    let mut seen = BTreeSet::new();

    // Check all rationals a/b with small denominators
    for b in 1..=50u64 {
        for a in 1..=500u64 {
            // Check if a/b is a transition: compare L(r-eps) vs L(r+eps)
            // Use L(a*2-1, b*2) vs L(a*2+1, b*2)
            let l_below = compute_l(a * 2 - 1, b * 2, max_n);
            let l_above = compute_l(a * 2 + 1, b * 2, max_n);
            if l_below != l_above {
                // Reduce fraction
                let g = gcd(a, b);
                let (ra, rb) = (a / g, b / g);
                if !seen.contains(&(ra, rb)) {
                    seen.insert((ra, rb));
                    transitions.push((ra, rb));
                }
            }
        }
    }

    // Sort transitions by value
    transitions.sort_by(|a, b| (a.0 as f64 / a.1 as f64).partial_cmp(&(b.0 as f64 / b.1 as f64)).unwrap());

    println!("\nFirst 30 transitions:");
    for (i, &(a, b)) in transitions.iter().take(30).enumerate() {
        println!("T({}) = {}/{} = {:.10}", i + 1, a, b, a as f64 / b as f64);
    }

    // Check T(22)
    if transitions.len() >= 22 {
        let (a, b) = transitions[21];
        println!("\nT(22) = {}/{} = {:.10}", a, b, a as f64 / b as f64);
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}
