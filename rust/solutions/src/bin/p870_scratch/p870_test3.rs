// Investigate the structure of transition values more carefully
// Use larger max_n and look for patterns

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

fn compute_losing_recurrence(r_num: u64, r_den: u64, count: usize) -> Vec<u64> {
    let mut p = vec![1u64];
    for _ in 1..count {
        let pn = *p.last().unwrap();
        let target = pn * r_den; // want r_num * p[m] >= target
        // But we need floor(r_num * p[m] / r_den) >= pn
        // Which means r_num * p[m] >= pn * r_den (since if r_num*p[m]/r_den >= pn and
        // pn is integer, floor >= pn too)
        // Actually: floor(r_num * p[m] / r_den) >= pn
        // <=> r_num * p[m] / r_den >= pn  (since pn is integer)
        // <=> r_num * p[m] >= pn * r_den
        let mut m = 0;
        while m < p.len() {
            if r_num * p[m] >= target {
                break;
            }
            m += 1;
        }
        if m >= p.len() {
            panic!("Could not find m for pn={}", pn);
        }
        p.push(pn + p[m]);
    }
    p
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    // For r slightly above 2: L = Fibonacci = 1, 2, 3, 5, 8, 13, 21, 34, 55, ...
    // The recurrence is P_{n+1} = P_n + P_{n-1} (standard Fibonacci)
    // because m = n-1 always works: floor(2 * P_{n-1}) >= P_n holds for Fibonacci.

    // For r slightly below 5/2: L = 1, 2, 3, 5, 7, 10, 15, 22, 32, 47, ...
    // The recurrence: P_{n+1} = P_n + P_{m} where floor(r * P_m) >= P_n.
    // For the 5/2 sequence, what's the pattern?
    println!("Sequence at r=5/2-eps (using 49/20):");
    let seq = compute_losing_recurrence(49, 20, 20);
    println!("{:?}", seq);

    println!("Sequence at r=5/2+eps (using 51/20):");
    let seq = compute_losing_recurrence(51, 20, 20);
    println!("{:?}", seq);

    println!("Sequence at r=2+eps (using 41/20):");
    let seq_fib = compute_losing_recurrence(41, 20, 20);
    println!("{:?}", seq_fib);

    // Now let's look at transitions more carefully.
    // Between r=2 and r=5/2, the transitions happen at P_n/P_m values.
    // But which P_n, P_m?
    //
    // A transition at ratio q = P_n/P_m means that as r increases through q,
    // floor(r * P_m) increases from P_n - 1 to P_n, which changes the recurrence.
    //
    // So I need to find ALL ratios P_n/P_m from the sequence, sorted, and those
    // that actually cause a change in L(r).

    // Let's compute the Fibonacci sequence and look at ratios
    let fib: Vec<u64> = compute_losing_recurrence(201, 100, 30);
    println!("\nFibonacci-ish: {:?}", fib);

    // All ratios P_i/P_j for i > j, sorted
    let mut ratios: Vec<(u64, u64, usize, usize)> = Vec::new();
    for i in 0..fib.len() {
        for j in 0..i {
            let g = gcd(fib[i], fib[j]);
            ratios.push((fib[i]/g, fib[j]/g, i, j));
        }
    }
    ratios.sort_by(|a, b| (a.0 as f64 / a.1 as f64).partial_cmp(&(b.0 as f64 / b.1 as f64)).unwrap());
    ratios.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);

    // Show ratios near 2
    println!("\nRatios near 2 from Fibonacci:");
    for &(n, d, i, j) in ratios.iter().filter(|r| {
        let v = r.0 as f64 / r.1 as f64;
        v >= 1.9 && v <= 3.1
    }).take(40) {
        println!("  {}/{} = {:.10} (P[{}]/P[{}])", n, d, n as f64 / d as f64, i, j);
    }

    // Now let's check: is the T(3) = 7/3 a ratio from the Fibonacci sequence?
    // Fibonacci: 1, 2, 3, 5, 8, 13, 21, 34, 55, 89
    // 7 is not in Fibonacci. So 7/3 is not a ratio of Fibonacci numbers.
    // But 7 = 5 + 2 = F(5) + F(3). Hmm.
    //
    // Let's check what the losing sequence looks like for r just above 7/3.
    println!("\nSequence at r=7/3+eps (using 141/60):");
    let seq = compute_losing_recurrence(141, 60, 15);
    println!("{:?}", seq);

    println!("Sequence at r=7/3-eps (using 139/60):");
    let seq = compute_losing_recurrence(139, 60, 15);
    println!("{:?}", seq);

    // And for r just above 2:
    println!("\nSequence at r=2+tiny (using 2001/1000):");
    let seq = compute_losing_recurrence(2001, 1000, 15);
    println!("{:?}", seq);

    // Let's verify these by brute force with larger max_n
    println!("\nBrute force at r=2001/1000 (max_n=100):");
    let bf = compute_l_brute(2001, 1000, 100);
    println!("{:?}", bf);

    println!("\nRecurrence at r=2001/1000:");
    let rec = compute_losing_recurrence(2001, 1000, 15);
    println!("{:?}", rec);
}
