// Compute the losing sequence at r = 92424/402 + eps ≈ 229.91 - eps
// to check if 92425 and 402 appear as elements.

fn main() {
    // r slightly below 92425/402
    let rn = 92424u64;
    let rd = 402u64;

    let mut p: Vec<u64> = vec![0, 1]; // 1-indexed
    let mut j_lo = 1usize;

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

        match pk.checked_add(p[m]) {
            Some(v) => p.push(v),
            None => break,
        }
        j_lo = m;
    }

    println!("Sequence at r = {}/{} = {:.6}:", rn, rd, rn as f64 / rd as f64);
    println!("Length: {}", p.len() - 1);

    // Find 402 and 92425 in the sequence
    for i in 1..p.len() {
        if p[i] == 402 {
            println!("  p[{}] = 402", i);
        }
        if p[i] == 92425 {
            println!("  p[{}] = 92425", i);
        }
    }

    // Print elements around index 230 and 400
    println!("\nAround index 230:");
    for i in 225..p.len().min(240) {
        println!("  p[{}] = {}", i, p[i]);
    }

    println!("\nAround index 400:");
    for i in 395..p.len().min(410) {
        println!("  p[{}] = {}", i, p[i]);
    }

    // Print the first 250 elements
    println!("\nFirst 250 elements:");
    for i in 1..p.len().min(251) {
        if p[i] < 300 {
            print!("{} ", p[i]);
        }
    }
    println!();
}
