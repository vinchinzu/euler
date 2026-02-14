// Project Euler 021: Amicable Numbers
// Sum of all amicable numbers under 10000.

fn main() {
    const LIMIT: usize = 10_000;
    const EXT: usize = 20_000;

    // Sieve proper divisor sums
    let mut d = vec![0u32; EXT + 1];
    for i in 1..=EXT {
        for j in (2 * i..=EXT).step_by(i) {
            d[j] += i as u32;
        }
    }

    let mut sum: u64 = 0;
    for a in 1..LIMIT {
        let b = d[a] as usize;
        if b != a && b <= EXT && d[b] as usize == a {
            sum += a as u64;
        }
    }

    println!("{sum}");
}
