// Test if no_improve = 50 is sufficient.
// Print some steps where the best candidate changes late.

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    let limit = 200;
    let mut rn: u64 = 1;
    let mut rd: u64 = 1;

    for ti in 1..limit {
        let mut p: Vec<u64> = Vec::with_capacity(512);
        p.push(0);
        p.push(1);
        let mut j_lo = 1usize;

        let mut best_n: u64 = 0;
        let mut best_d: u64 = 0;
        let mut best_set = false;
        let mut last_improve_step = 0usize;
        let mut step_count = 0usize;

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

            step_count += 1;

            if m >= 2 {
                let cand_d = p[m - 1];
                if (pk as u128) * (rd as u128) > (rn as u128) * (cand_d as u128) {
                    if !best_set || (pk as u128) * (best_d as u128) < (best_n as u128) * (cand_d as u128) {
                        let g = gcd(pk, cand_d);
                        best_n = pk / g;
                        best_d = cand_d / g;
                        best_set = true;
                        last_improve_step = step_count;
                    }
                }
            }

            match pk.checked_add(p[m]) {
                Some(v) => p.push(v),
                None => break,
            }
            j_lo = m;

            // Run until no improvement for 200 steps
            if best_set && step_count - last_improve_step >= 200 {
                break;
            }
        }

        if !best_set {
            eprintln!("ERROR at step {}", ti);
            return;
        }

        rn = best_n;
        rd = best_d;

        if last_improve_step > 30 || ti <= 30 {
            eprintln!("T({}) = {}/{} = {:.10}  (last_improve at step {}, total steps {})",
                ti + 1, rn, rd, rn as f64 / rd as f64, last_improve_step, step_count);
        }
    }
}
