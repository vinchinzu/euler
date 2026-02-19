// More careful investigation: the recurrence might not exactly match the game.
// Let me check L(r) by brute force for various r values around the transitions.

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

    // Check brute force around 7/3
    println!("L(r) at r = 7/3 - eps = 139/60:");
    println!("{:?}", compute_l_brute(139, 60, max_n));
    println!("L(r) at r = 7/3 + eps = 141/60:");
    println!("{:?}", compute_l_brute(141, 60, max_n));

    // Check brute force around 5/2 more carefully
    println!("\nL(r) at r = 5/2 - eps = 249/100:");
    println!("{:?}", compute_l_brute(249, 100, max_n));
    println!("L(r) at r = 5/2 + eps = 251/100:");
    println!("{:?}", compute_l_brute(251, 100, max_n));

    // Check transitions between 1 and 2
    println!("\nL(r) at r = 3/2 - eps = 149/100:");
    println!("{:?}", compute_l_brute(149, 100, max_n));
    println!("L(r) at r = 3/2 + eps = 151/100:");
    println!("{:?}", compute_l_brute(151, 100, max_n));

    // Check what happens for r < 1
    println!("\nL(r) at r = 1/2:");
    println!("{:?}", compute_l_brute(1, 2, 20));
    println!("L(r) at r = 3/4:");
    println!("{:?}", compute_l_brute(3, 4, 20));
    println!("L(r) at r = 9/10:");
    println!("{:?}", compute_l_brute(9, 10, 20));
    println!("L(r) at r = 99/100:");
    println!("{:?}", compute_l_brute(99, 100, 20));

    // Try to find transitions between 0 and 1
    // Actually from problem statement, L(0.5)={1} and L(1) = {1,2,4,8,...}
    // So T(1) = 1 is the first transition.
    // For r < 1 but close to 1, e.g. r=0.99, what happens?
    println!("\nL(r) at r=0.99 (max_n=30):");
    println!("{:?}", compute_l_brute(99, 100, 30));
    println!("L(r) at r=1.01:");
    println!("{:?}", compute_l_brute(101, 100, 30));

    // Now let's carefully enumerate ALL transitions from T(1) onward
    // Strategy: the transitions must be rational numbers a/b where a, b are
    // elements of some losing position sequence.
    // But we don't know which. Let me just do a very fine scan.

    // Actually, let me try: transitions are at values where the set of losing
    // positions changes. Since L(r) is piecewise constant on intervals,
    // and transitions happen at values where floor(r*k) changes for some
    // critical k, the transitions are at rationals n/k for integers n, k.
    //
    // But not ALL rationals are transitions. Let me check which ones are.

    // Enumerate transitions between 1 and 10 by checking L for consecutive
    // rationals with denominator up to 100
    let max_n = 80;
    let max_den = 60;
    let mut transition_rats: Vec<(u64, u64)> = Vec::new();

    // Build sorted list of unique rationals a/b with 1 <= a/b <= 10
    let mut rats: Vec<(u64, u64)> = Vec::new();
    for b in 1..=max_den {
        for a in b..=10*b {
            let g = gcd(a, b);
            rats.push((a/g, b/g));
        }
    }
    rats.sort_by(|a, b| (a.0 * b.1).cmp(&(b.0 * a.1)));
    rats.dedup();

    // For each consecutive pair, check if L changes
    let mut prev = compute_l_brute(rats[0].0, rats[0].1, max_n);
    for i in 1..rats.len() {
        let (a, b) = rats[i];
        let l = compute_l_brute(a, b, max_n);
        if l != prev {
            transition_rats.push((a, b));
        }
        prev = l;
    }

    println!("\nTransitions found (first 40):");
    for (i, &(a, b)) in transition_rats.iter().take(40).enumerate() {
        println!("T({}) = {}/{} = {:.10}", i+1, a, b, a as f64 / b as f64);
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}
