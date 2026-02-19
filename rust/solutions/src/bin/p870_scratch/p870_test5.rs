// Compute transition values using the correct algorithm:
// 1. Start at T(i), compute the losing sequence P for r slightly above T(i).
// 2. For each step n, compute m(n) = min{j : floor(r * P_j) >= P_n}.
// 3. T(i+1) = min over n of P_n / P_{m(n)-1} where m(n) >= 2 and P_n / P_{m(n)-1} > T(i).
//
// Use exact rational arithmetic to avoid floating point issues.

use num::bigint::BigInt;
use num::rational::BigRational;
use num::{Zero, One, ToPrimitive};

fn gcd_u64(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd_u64(b, a % b) }
}

fn main() {
    // Represent r as a fraction (rn, rd)
    // Start at T(1) = 1
    let mut rn: u64 = 1;
    let mut rd: u64 = 1;

    let limit = 30; // Compute first 30 transitions
    let max_seq = 2000; // Maximum sequence length to compute

    for ti in 1..=limit {
        println!("T({}) = {}/{} = {:.10}", ti, rn, rd, rn as f64 / rd as f64);

        if ti == limit {
            break;
        }

        // Compute the losing sequence for r = rn/rd + epsilon
        // The recurrence: P_1 = 1, P_{n+1} = P_n + P_{m(n)}
        // where m(n) = min{j : floor(r * P_j) >= P_n}
        // Since r = rn/rd + eps, floor((rn/rd + eps) * P_j) = floor(rn*P_j/rd + eps*P_j)
        // If rn*P_j % rd == 0: floor = rn*P_j/rd (and with eps, it's still rn*P_j/rd)
        // If rn*P_j % rd != 0: floor = rn*P_j/rd (integer division, rounds down)
        // So for r = rn/rd + eps: floor(r * P_j) = (rn * P_j) / rd if rn*P_j % rd != 0
        //                                        = rn * P_j / rd   if rn*P_j % rd == 0
        // Wait, floor((rn/rd + eps) * P_j):
        // If rn*P_j / rd is exact integer: floor = rn*P_j/rd (the eps*P_j adds a tiny bit
        //   above the integer, but floor is still that integer... unless eps*P_j >= 1 which
        //   it doesn't for small enough eps).
        // Actually, no. If rn*P_j/rd is exact integer K, then (rn/rd + eps)*P_j = K + eps*P_j.
        // floor(K + eps*P_j) = K for small enough eps. So floor(r*P_j) = rn*P_j / rd (integer
        // division, truncating) for r = rn/rd + eps.
        //
        // But wait: we want the behavior just ABOVE the transition. At the transition r = rn/rd,
        // for the case that caused this transition, we have rn*P_j = P_n * rd exactly.
        // For r slightly above, floor(r*P_j) = P_n (the value increases from P_n-1 to P_n).
        //
        // So: floor(r*P_j) for r = rn/rd + eps is:
        //   rn * P_j / rd  (integer division, truncating)
        // This is correct because for r just above rn/rd, the floor is the same as at rn/rd
        // (the eps only adds a tiny amount, not enough to cross the next integer).
        //
        // The condition floor(r * P_j) >= P_n becomes: rn * P_j / rd >= P_n (integer division)
        // which is: rn * P_j >= P_n * rd (since this is exact when divides evenly, and
        // rn*P_j/rd rounds down otherwise).
        //
        // Wait: if rn * P_j = P_n * rd exactly, then floor(r * P_j) = P_n at r = rn/rd.
        // For r = rn/rd + eps: floor = P_n (since we go just above). So the condition
        // floor(r * P_j) >= P_n is satisfied. Good.
        //
        // If rn * P_j = P_n * rd - 1 (just below), then floor(rn*P_j / rd) = P_n - 1 < P_n.
        // For r = rn/rd + eps: floor((rn+eps*rd)*P_j/rd) = floor((P_n*rd - 1)/rd + eps*P_j)
        //                    = floor(P_n - 1/rd + eps*P_j)
        //                    = P_n - 1 (for small enough eps).
        // So condition not satisfied. Correct.
        //
        // Summary: use rn * P_j >= P_n * rd as the condition (equivalent to floor >= P_n
        // for r = rn/rd + eps). Actually hmm, when rn*P_j = P_n*rd exactly, rn*P_j/rd = P_n
        // (exact), so floor = P_n, condition satisfied. For rn*P_j = P_n*rd - 1, rn*P_j/rd
        // = P_n - 1/rd < P_n, floor = P_n - 1 < P_n, not satisfied.
        // For rn*P_j = P_n*rd + 1, rn*P_j/rd = P_n + 1/rd, floor = P_n >= P_n, satisfied.
        //
        // So condition is: rn * P_j >= P_n * rd. Correct.

        let mut p: Vec<u64> = vec![1];
        let mut m_vals: Vec<usize> = Vec::new(); // m(n) for each step

        let mut best_num: u64 = u64::MAX;
        let mut best_den: u64 = 1;

        let mut j_start = 0usize; // optimization: m(n) is non-decreasing

        for _step in 0..max_seq {
            let pn = *p.last().unwrap();
            let target = pn as u128 * rd as u128;

            // Find m = min{j : rn * p[j] >= pn * rd}
            let mut m = j_start;
            while m < p.len() {
                if rn as u128 * p[m] as u128 >= target {
                    break;
                }
                m += 1;
            }
            if m >= p.len() {
                // Sequence grows too fast or we can't find m.
                // This shouldn't happen for reasonable parameters.
                break;
            }

            m_vals.push(m);

            // Candidate for next transition: pn / p[m-1] if m >= 1
            if m >= 1 {
                let cand_n = pn;
                let cand_d = p[m - 1];
                // Check: cand_n / cand_d > rn / rd, i.e., cand_n * rd > rn * cand_d
                if (cand_n as u128) * (rd as u128) > (rn as u128) * (cand_d as u128) {
                    // Check if this is better than current best
                    // cand_n / cand_d < best_num / best_den?
                    if (cand_n as u128) * (best_den as u128) < (best_num as u128) * (cand_d as u128) {
                        let g = gcd_u64(cand_n, cand_d);
                        best_num = cand_n / g;
                        best_den = cand_d / g;
                    }
                }
            }

            let pm = p[m];
            let next = pn + pm;
            p.push(next);
            j_start = m; // m(n) is non-decreasing
        }

        // Set the next transition
        if best_num == u64::MAX {
            println!("Could not find next transition!");
            break;
        }
        rn = best_num;
        rd = best_den;
    }
}
