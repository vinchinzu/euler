// Proper algorithm to compute transition values.
//
// At each step:
// 1. Compute the losing sequence P for the current r = rn/rd + eps.
// 2. For each pair (a, b) with b < m(a) and P_a/P_b > r, find the minimum ratio.
// 3. That minimum ratio is the next transition T(i+1).
//
// Optimization: for fixed b, the minimum P_a > r*P_b is the first P_a exceeding r*P_b.
// Since the sequence is increasing, we can binary search.
// Also, the minimum over b decreases as we search more, allowing early termination.

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    let mut rn: u64 = 1;
    let mut rd: u64 = 1;
    let limit = 30;
    let max_seq = 500; // Enough terms to find the minimum ratio

    for ti in 1..=limit {
        println!("T({}) = {}/{} = {:.10}", ti, rn, rd, rn as f64 / rd as f64);

        if ti == limit { break; }

        // Compute the losing sequence for r = rn/rd + eps
        // P_1 = 1
        // P_{k+1} = P_k + P_{m(k)} where m(k) = min{j : rn * P_j >= P_k * rd}
        // (using rn*P_j >= P_k * rd as the condition for floor(r*P_j) >= P_k when r=rn/rd+eps)
        // Actually: floor((rn/rd + eps) * P_j) >= P_k
        // For eps tiny: this is floor(rn*P_j/rd + eps*P_j) >= P_k
        // If rn*P_j mod rd = 0: floor = rn*P_j/rd. Check rn*P_j/rd >= P_k, i.e., rn*P_j >= P_k*rd.
        // If rn*P_j mod rd != 0: floor = (rn*P_j)/rd (integer div). Check (rn*P_j)/rd >= P_k.
        //   Since rn*P_j/rd is not integer, floor = (rn*P_j)/rd. This is >= P_k iff rn*P_j >= P_k*rd.
        //   (If rn*P_j = P_k*rd - 1, then floor = P_k - 1 and with eps, still P_k - 1 for tiny eps.)
        //   (If rn*P_j = P_k*rd, floor = P_k exactly, with eps it's P_k, condition satisfied.)
        // So the condition is: rn * P_j >= P_k * rd.

        let mut p: Vec<u64> = vec![0, 1]; // 1-indexed: p[1] = 1
        let mut m_of: Vec<usize> = vec![0, 0]; // m_of[k] for each k

        let mut j_lo = 1usize; // m(k) is non-decreasing

        for _step in 1..max_seq {
            let k = p.len() - 1; // last index
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

            m_of.push(m);
            let next = pk + p[m];
            if next < pk { break; } // overflow check
            p.push(next);
            j_lo = m;
        }

        // Now find the minimum P_a / P_b > rn/rd where b < m_of[a]
        // For each b from 1 to max, find the smallest a such that:
        //   P_a / P_b > rn/rd, i.e., P_a * rd > rn * P_b
        //   AND b < m_of[a]
        //
        // Since m_of[a] is non-decreasing, for larger a, b < m_of[a] is more likely.
        // For small a, m_of[a] might be very small.

        let mut best_n: u64 = 0;
        let mut best_d: u64 = 0;
        let mut best_set = false;

        for b in 1..p.len() {
            // Find smallest a such that P_a * rd > rn * P_b AND b < m_of[a]
            let threshold = rn as u128 * p[b] as u128; // P_a * rd > threshold => P_a > threshold/rd

            // Binary search for smallest a with P_a * rd > threshold
            let min_pa = threshold / rd as u128 + 1; // P_a >= min_pa

            // Find first index a with P_a >= min_pa using binary search
            let a_start = match p.binary_search_by(|x| (*x as u128).cmp(&min_pa)) {
                Ok(i) => i,
                Err(i) => i,
            };

            // Check candidates starting from a_start
            for a in a_start..p.len() {
                if a <= b { continue; } // need a > b for P_a > P_b
                if b >= m_of.len() || a >= m_of.len() { break; }
                if b >= m_of[a] { continue; } // need b < m_of[a]

                // P_a / P_b is a candidate
                let cand_n = p[a];
                let cand_d = p[b];

                // Check if it's better than current best
                if !best_set || (cand_n as u128) * (best_d as u128) < (best_n as u128) * (cand_d as u128) {
                    let g = gcd(cand_n, cand_d);
                    best_n = cand_n / g;
                    best_d = cand_d / g;
                    best_set = true;
                }
                break; // The first valid a for this b gives the smallest ratio for this b
            }

            // Early termination: if the minimum possible ratio for remaining b's
            // can't beat the current best, stop.
            // For b+1, the minimum ratio is at least P_{b+2} / P_{b+1}, which is > 1.
            // But we can't easily bound it. Let's just check all b's.
        }

        if !best_set {
            println!("Could not find next transition at step {}", ti);
            break;
        }

        rn = best_n;
        rd = best_d;
    }
}
