// Fixed version: proper indexing for transitions.
//
// Sequence: p[1], p[2], p[3], ... (1-indexed, p[0] = dummy 0)
// Recurrence: p[k+1] = p[k] + p[m_of[k+1]] where m_of[k+1] = min{j : rn * p[j] >= p[k] * rd}
//
// Transition at r = p[n] / p[b] affects step n: computing p[n+1].
// m for step n is m_of[n+1].
// Condition: b < m_of[n+1].

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    let mut rn: u64 = 1;
    let mut rd: u64 = 1;
    let limit = 30;
    let max_seq = 500;

    for ti in 1..=limit {
        println!("T({}) = {}/{} = {:.10}", ti, rn, rd, rn as f64 / rd as f64);

        if ti == limit { break; }

        // Compute losing sequence
        let mut p: Vec<u64> = vec![0, 1]; // p[1] = 1
        let mut m_of: Vec<usize> = vec![0, 0]; // m_of[0], m_of[1] unused/dummy

        let mut j_lo = 1usize;

        for _step in 1..max_seq {
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

            m_of.push(m); // m_of[k+1] = m for step computing p[k+1]
            let next = pk + p[m];
            if next < pk { break; } // overflow
            p.push(next);
            j_lo = m;
        }

        // Find minimum p[n] / p[b] > rn/rd where b < m_of[n+1] and n+1 < p.len()
        let mut best_n: u64 = 0;
        let mut best_d: u64 = 0;
        let mut best_set = false;

        for b in 1..p.len() {
            let pb = p[b];
            if pb == 0 { continue; }

            // We want p[n] > (rn/rd) * p[b], i.e., p[n] * rd > rn * pb
            let threshold = rn as u128 * pb as u128;

            // Find smallest n such that p[n] * rd > threshold, i.e., p[n] > threshold / rd
            let min_pn = threshold / rd as u128 + 1;

            // Binary search for the first index n with p[n] >= min_pn
            let n_start = match p[1..].binary_search_by(|x| (*x as u128).cmp(&min_pn)) {
                Ok(i) => i + 1, // found exact match at p[i+1]
                Err(i) => i + 1, // insertion point, p[i+1] is the first >= min_pn
            };

            for n in n_start..p.len() {
                if n <= b { continue; } // need p[n] > p[b]
                // Check b < m_of[n+1]. We need n+1 to exist in m_of.
                if n + 1 >= m_of.len() { break; }
                if b >= m_of[n + 1] { continue; } // need b < m_of[n+1]

                // p[n] / p[b] is a valid candidate
                let cand_n = p[n];
                let cand_d = pb;

                if !best_set || (cand_n as u128) * (best_d as u128) < (best_n as u128) * (cand_d as u128) {
                    let g = gcd(cand_n, cand_d);
                    best_n = cand_n / g;
                    best_d = cand_d / g;
                    best_set = true;
                }
                break; // First valid n for this b is the best for this b
            }
        }

        if !best_set {
            println!("Could not find next transition at step {}", ti);
            break;
        }

        rn = best_n;
        rd = best_d;
    }
}
