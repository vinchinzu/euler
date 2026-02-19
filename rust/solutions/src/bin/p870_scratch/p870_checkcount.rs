// Verify the count: T(1)=1, T(2)=2, T(22)=145/23=6.3043478261

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn compute_next_transition(rn: u64, rd: u64) -> (u64, u64) {
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
                    if no_improve >= 200 { break; }
                }
            }
        }

        match pk.checked_add(p[m]) {
            Some(v) => p.push(v),
            None => break,
        }
        j_lo = m;
    }

    (best_n, best_d)
}

fn main() {
    let mut rn: u64 = 1;
    let mut rd: u64 = 1;

    for ti in 1..=30 {
        println!("T({}) = {}/{} = {:.10}", ti, rn, rd, rn as f64 / rd as f64);
        let (nn, nd) = compute_next_transition(rn, rd);
        rn = nn;
        rd = nd;
    }
}
