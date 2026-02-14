// Project Euler 174: Counting hollow square laminae
fn main() {
    const LIMIT: usize = 1_000_000;
    let mut counts = vec![0u32; LIMIT + 1];

    let max_m = ((LIMIT as f64 / 4.0).sqrt() as usize) + 1;
    for m in 1..=max_m {
        let min_k = 2 * m + 1;
        let max_k = m + LIMIT / (4 * m);
        for k in min_k..=max_k {
            let t = 4 * m * (k - m);
            if t > LIMIT { break; }
            counts[t] += 1;
        }
    }

    let result = counts[1..].iter().filter(|&&c| c >= 1 && c <= 10).count();
    println!("{result}");
}
