// Scan for transitions by systematically checking rationals.
// For each step, instead of finding the minimum ratio from the sequence,
// let's check ALL rationals a/b > r in increasing order.

// Actually, let me first understand: what rationals a/b correspond to transitions?
// A transition at r = a/b means floor(r*b) changes: specifically, floor((a/b)*b) = a,
// while floor((a/b - eps)*b) = a - 1.
// So crossing a/b changes floor(r*b) from a-1 to a.
// This could affect the game at pile sizes where someone's move of b stones is relevant.
//
// More generally, crossing any a/b changes floor(r*m) for m=b (and possibly for other m
// that are multiples of b).
//
// For the losing sequence recurrence: m(k) = min{j: floor(r*P_j) >= P_k}.
// When r crosses some q, floor(r*P_j) might change for one or more j values.
// Specifically, floor(r*P_j) changes when r*P_j crosses an integer, i.e., r = c/P_j
// for some integer c.
//
// So transitions happen at r = c/P_j for some integer c and some element P_j of the
// current sequence. AND c must equal some P_k (so that floor changes from P_k-1 to P_k).
//
// Wait, floor(r*P_j) changes from c-1 to c when r crosses c/P_j. This affects the
// condition floor(r*P_j) >= P_k for P_k = c (but not for P_k < c, since floor was
// already >= P_k).
//
// So the transition at c/P_j affects step k where P_k = c. But c need not be an element
// of the sequence! c is just any positive integer.
//
// OH WAIT. floor(r*P_j) changes from c-1 to c at r = c/P_j, for ANY integer c.
// The condition floor(r*P_j) >= P_k is affected when c = P_k (specifically, P_j becomes
// valid for step k).
//
// But also when c = P_k + 1 or higher: floor(r*P_j) >= c >= P_k + 1 > P_k, but this
// doesn't matter since P_j was already valid (floor was already >= P_k + 1 > P_k).
//
// And when c < P_k: floor changes from c-1 to c, but c < P_k, so still < P_k. Not relevant.
//
// So the only relevant transitions are at c = P_k, i.e., r = P_k / P_j. This is what I had.
//
// So the transitions ARE at ratios P_k / P_j from the sequence. My algorithm is correct.
//
// BUT: maybe I need to consider ALL j values, not just j = m(k) - 1?

// Let me reconsider: for a given step k, the transition candidates are P_k/P_j for all
// j < m(k). The minimum is P_k / P_{m(k)-1}.
//
// But what about j from a DIFFERENT step? For step k', the candidates are P_{k'}/P_j for
// j < m(k'). A candidate from step k' might be smaller than the candidate from step k.
//
// My algorithm already considers all steps k (iterating over the sequence). For each k,
// it checks P_k / P_{m(k)-1}. The global minimum over all k gives the next transition.
//
// But HOLD ON: what if for some step k, there's a j << m(k)-1 that gives a smaller ratio?
// P_k / P_j for j < m(k)-1 would give a LARGER ratio (since P_j < P_{m(k)-1}).
// So P_k / P_{m(k)-1} IS the minimum for step k.
//
// Wait, LARGER denominator means SMALLER ratio! P_{m(k)-1} is the LARGEST denominator
// among j < m(k), giving the SMALLEST ratio for step k. This is correct.
//
// So my algorithm IS considering all relevant transitions. The issue must be elsewhere.

// Let me try something: compute the sequence at r = 5 + eps and find ALL transitions
// between 5 and 6, then compare with the brute force.

fn compute_l_recurrence(rn: u64, rd: u64, max_terms: usize) -> Vec<u64> {
    let mut p: Vec<u64> = vec![0, 1]; // 1-indexed
    let mut j_lo = 1usize;

    for _ in 1..max_terms {
        let k = p.len() - 1;
        let pk = p[k];
        let target = pk as u128 * rd as u128;

        let mut m = j_lo;
        while m < p.len() {
            if rn as u128 * p[m] as u128 >= target {
                break;
            }
            m += 1;
        }
        if m >= p.len() { break; }

        match pk.checked_add(p[m]) {
            Some(v) => p.push(v),
            None => break,
        }
        j_lo = m;
    }
    p
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    // Compute the sequence at r = 5 + eps (use 501/100)
    let rn = 501u64;
    let rd = 100u64;
    let p = compute_l_recurrence(rn, rd, 300);

    println!("Sequence at r=5+eps (first 50 elements):");
    for i in 1..p.len().min(51) {
        print!("{} ", p[i]);
    }
    println!();

    // Find ALL ratios P_a / P_b > 5 from this sequence, up to ratio < 6
    let mut ratios: Vec<(u64, u64, usize, usize)> = Vec::new();
    for a in 1..p.len() {
        for b in 1..a {
            if p[b] == 0 { continue; }
            // P_a / P_b
            let ratio_check = p[a] as u128 * 100 > 501u128 * p[b] as u128; // > 501/100
            if !ratio_check { continue; }
            let ratio_below_6 = p[a] as u128 * 1 < 6u128 * p[b] as u128; // < 6/1
            if !ratio_below_6 { continue; }
            let g = gcd(p[a], p[b]);
            ratios.push((p[a]/g, p[b]/g, a, b));
        }
    }

    // Remove duplicates and sort
    ratios.sort_by(|a, b| (a.0 as u128 * b.1 as u128).cmp(&(b.0 as u128 * a.1 as u128)));
    ratios.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);

    println!("\nRatios P_a/P_b in (5, 6) from the sequence:");
    for (n, d, a, b) in &ratios {
        println!("  {}/{} = {:.10}  (P[{}]={}/P[{}]={})", n, d, *n as f64 / *d as f64, a, p[*a], b, p[*b]);
    }
    println!("Total: {}", ratios.len());

    // Now check which of these are actually transitions (i.e., they affect m(k) for some k)
    // This means b < m(k) where k is such that P_k = P_a (i.e., k = a)
    // But we need m(k) computed at the current r.
    //
    // Actually, let me just check: how many of these ratios are between 5 and 6?
    // And does this match my transition algorithm?
}
