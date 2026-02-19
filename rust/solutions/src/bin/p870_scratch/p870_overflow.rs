// Check for potential overflow issues in the transition computation.

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    let limit: usize = 1000;
    let mut rn: u64 = 1;
    let mut rd: u64 = 1;

    let mut max_rn: u64 = 0;
    let mut max_rd: u64 = 0;
    let mut max_seq_len: usize = 0;
    let mut max_p_val: u64 = 0;

    for ti in 1..limit {
        let mut p: Vec<u64> = Vec::with_capacity(512);
        p.push(0);
        p.push(1);
        let mut j_lo = 1usize;

        let mut best_n: u64 = 0;
        let mut best_d: u64 = 0;
        let mut best_set = false;
        let mut no_improve = 0u32;

        loop {
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

            if m >= 2 {
                let cand_d = p[m - 1];
                if (pk as u128) * (rd as u128) > (rn as u128) * (cand_d as u128) {
                    if !best_set || (pk as u128) * (best_d as u128) < (best_n as u128) * (cand_d as u128) {
                        let g = gcd(pk, cand_d);
                        best_n = pk / g;
                        best_d = cand_d / g;
                        best_set = true;
                        no_improve = 0;
                    } else {
                        no_improve += 1;
                        if no_improve >= 50 {
                            break;
                        }
                    }
                }
            }

            let pm = p[m];
            match pk.checked_add(pm) {
                Some(v) => {
                    if v > max_p_val { max_p_val = v; }
                    p.push(v);
                }
                None => {
                    eprintln!("OVERFLOW at step {}, seq len {}, pk={}, pm={}", ti, p.len(), pk, pm);
                    break;
                }
            }
            j_lo = m;
        }

        if !best_set {
            eprintln!("ERROR: No transition found at step {}", ti);
            return;
        }

        rn = best_n;
        rd = best_d;

        if rn > max_rn { max_rn = rn; }
        if rd > max_rd { max_rd = rd; }
        if p.len() > max_seq_len { max_seq_len = p.len(); }

        if ti <= 10 || ti % 100 == 0 {
            eprintln!("T({}) = {}/{} = {:.6}, seq_len={}", ti, rn, rd, rn as f64 / rd as f64, p.len());
        }
    }

    eprintln!("\nMax rn: {}", max_rn);
    eprintln!("Max rd: {}", max_rd);
    eprintln!("Max seq len: {}", max_seq_len);
    eprintln!("Max p value: {}", max_p_val);
    eprintln!("u64 MAX: {}", u64::MAX);
}
