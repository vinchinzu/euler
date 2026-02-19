// Find where 92425 and 402 appear in the sequence at r just below 92425/402.

fn main() {
    // r = 92425/402 - epsilon. Use (92425*2-1)/(402*2) = 184849/804
    let rn = 184849u64;
    let rd = 804u64;

    let mut p: Vec<u64> = vec![0, 1];
    let mut j_lo = 1usize;
    let mut found_402 = false;
    let mut found_92425 = false;

    for _step in 1..100000 {
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
            Some(v) => {
                p.push(v);
                if v == 402 && !found_402 {
                    println!("Found 402 at index {}", p.len()-1);
                    found_402 = true;
                }
                if v == 92425 && !found_92425 {
                    println!("Found 92425 at index {}", p.len()-1);
                    found_92425 = true;
                }
            }
            None => break,
        }
        j_lo = m;
    }

    println!("Sequence length: {}", p.len() - 1);

    // Print around index 316 and wherever 92425 might be
    if found_402 {
        // find index of 402
        for i in 1..p.len() {
            if p[i] == 402 {
                println!("\nAround p[{}]=402:", i);
                for j in (i.saturating_sub(3))..p.len().min(i+4) {
                    println!("  p[{}] = {}", j, p[j]);
                }
                break;
            }
        }
    }

    // Show the m values around the transition
    // Recompute with m tracking
    let mut p2: Vec<u64> = vec![0, 1];
    let mut m_vals: Vec<usize> = vec![0, 0]; // dummy entries
    let mut j_lo2 = 1usize;

    for _ in 1..p.len() {
        let k = p2.len() - 1;
        let pk = p2[k];
        let target = pk as u128 * rd as u128;

        let mut m = j_lo2;
        while m < p2.len() {
            if rn as u128 * p2[m] as u128 >= target {
                break;
            }
            m += 1;
        }
        if m >= p2.len() { break; }

        m_vals.push(m);
        match pk.checked_add(p2[m]) {
            Some(v) => p2.push(v),
            None => break,
        }
        j_lo2 = m;
    }

    // Find the transition candidate for 92425/402
    for k in 2..p2.len() {
        let m = m_vals[k];
        if m >= 2 && k < p2.len() {
            let pk = p2[k-1]; // This is actually p[k-1], need to check
            // Actually m_vals[k] is the m for step computing p[k].
            // p[k] = p[k-1] + p[m_vals[k]]
            // Transition candidate: p[k-1] / p[m_vals[k] - 1]
            let cand_n = p2[k-1];
            let cand_d = p2[m - 1];
            if cand_d > 0 && cand_n > 0 {
                let ratio = cand_n as f64 / cand_d as f64;
                if (ratio - 229.9129353234).abs() < 0.001 {
                    println!("\nCandidate near 229.91: p[{}]={} / p[{}]={} = {:.10}",
                        k-1, cand_n, m-1, cand_d, ratio);
                }
            }
        }
    }
}
