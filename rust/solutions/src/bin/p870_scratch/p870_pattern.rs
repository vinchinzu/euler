// Print the first 200 transition values to look for patterns.

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    let mut rn: u64 = 1;
    let mut rd: u64 = 1;
    let limit = 200;
    let max_seq = 500;

    for ti in 1..=limit {
        println!("T({}) = {}/{} = {:.10}", ti, rn, rd, rn as f64 / rd as f64);

        if ti == limit { break; }

        let mut p: Vec<u64> = vec![0, 1];
        let mut j_lo = 1usize;

        let mut best_n: u64 = 0;
        let mut best_d: u64 = 0;
        let mut best_set = false;
        let mut no_improve = 0usize;
        let max_no_improve = 50;

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
                let cand_n = pk;
                let cand_d = p[m - 1];
                if (cand_n as u128) * (rd as u128) > (rn as u128) * (cand_d as u128) {
                    if !best_set || (cand_n as u128) * (best_d as u128) < (best_n as u128) * (cand_d as u128) {
                        let g = gcd(cand_n, cand_d);
                        best_n = cand_n / g;
                        best_d = cand_d / g;
                        best_set = true;
                        no_improve = 0;
                    } else {
                        no_improve += 1;
                        if no_improve >= max_no_improve {
                            break;
                        }
                    }
                }
            }

            let next = pk + p[m];
            if next < pk { break; }
            p.push(next);
            j_lo = m;
        }

        if !best_set {
            eprintln!("ERROR at step {}", ti);
            break;
        }
        rn = best_n;
        rd = best_d;
    }
}
