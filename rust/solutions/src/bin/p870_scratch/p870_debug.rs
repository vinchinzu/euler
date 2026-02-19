// Debug: what is the sequence at r = 34/5 + eps and where does the transition at 173/25 happen?

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    // r = 34/5 = 6.8
    let rn = 34u64;
    let rd = 5u64;

    // Compute the losing sequence for r = rn/rd + eps
    let mut p: Vec<u64> = vec![0, 1]; // 1-indexed
    let mut j_lo = 1usize;

    println!("Sequence at r = {}/{}:", rn, rd);

    for step in 1..200 {
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

        let next = pk + p[m];
        if next < pk { break; } // overflow

        // Check if this step generates the transition candidate
        if m >= 2 {
            let cand_n = pk;
            let cand_d = p[m - 1];
            let ratio = cand_n as f64 / cand_d as f64;
            if cand_n as u128 * rd as u128 > rn as u128 * cand_d as u128 {
                if ratio < 8.0 {
                    let g = gcd(cand_n, cand_d);
                    println!("  Step {}: p[{}]={}, m={}, p[m-1]={}, candidate={}/{} = {:.6}",
                        step, k, pk, m, p[m-1], cand_n/g, cand_d/g, ratio);
                }
            }
        }

        p.push(next);
        j_lo = m;
    }

    // Also print the first 30 elements
    println!("\nFirst 30 elements:");
    for i in 1..p.len().min(31) {
        println!("  p[{}] = {}", i, p[i]);
    }

    // Now check: at r = 173/25 + eps, does the sequence change?
    println!("\n\nSequence at r = 173/25 + eps (3461/500):");
    let rn2 = 3461u64;
    let rd2 = 500u64;
    let mut p2: Vec<u64> = vec![0, 1];
    let mut j_lo2 = 1usize;

    for _step in 1..200 {
        let k = p2.len() - 1;
        let pk = p2[k];
        let target = pk as u128 * rd2 as u128;

        let mut m = j_lo2;
        while m < p2.len() {
            if rn2 as u128 * p2[m] as u128 >= target {
                break;
            }
            m += 1;
        }
        if m >= p2.len() { break; }

        let next = pk + p2[m];
        if next < pk { break; }

        p2.push(next);
        j_lo2 = m;
    }

    println!("First 30 elements:");
    for i in 1..p2.len().min(31) {
        print!("  p[{}] = {}", i, p2[i]);
        if i < p.len() && p[i] != p2[i] {
            print!("  *** DIFFERS (was {})", p[i]);
        }
        println!();
    }

    // Check where they diverge
    let min_len = p.len().min(p2.len());
    for i in 1..min_len {
        if p[i] != p2[i] {
            println!("\nFirst divergence at index {}: {} vs {}", i, p[i], p2[i]);
            break;
        }
    }
}
