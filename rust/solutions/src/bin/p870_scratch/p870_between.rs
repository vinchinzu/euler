// Check ALL transition candidates between r=6 and r=145/23 using the recurrence.
// The candidate at each step is p[k] / p[m-1].

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    let rn = 6u64;
    let rd = 1u64;

    let mut p: Vec<u64> = vec![0, 1];
    let mut j_lo = 1usize;

    println!("Sequence at r = {}/{}:", rn, rd);
    println!("Transition candidates:");

    let mut candidates: Vec<(u64, u64, f64, usize)> = Vec::new();

    for _step in 1..500 {
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
            if cand_n as u128 * rd as u128 > rn as u128 * cand_d as u128 {
                let g = gcd(cand_n, cand_d);
                let cn = cand_n / g;
                let cd = cand_d / g;
                let ratio = cn as f64 / cd as f64;
                if ratio < 6.31 {
                    candidates.push((cn, cd, ratio, k));
                }
            }
        }

        let next = pk + p[m];
        if next < pk { break; }
        p.push(next);
        j_lo = m;
    }

    // Sort candidates by ratio
    candidates.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    println!("Candidates with ratio in (6, 6.31):");
    for (cn, cd, ratio, step) in &candidates {
        println!("  {}/{} = {:.10}  (step {})", cn, cd, ratio, step);
    }
    println!("Total: {}", candidates.len());
}
