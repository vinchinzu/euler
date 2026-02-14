// Project Euler 44: Pentagon numbers
// Find the pair of pentagonal numbers whose sum and difference are also pentagonal,
// minimising the difference.

fn pentagonal(n: u64) -> u64 {
    n * (3 * n - 1) / 2
}

fn is_pentagonal(x: u64) -> bool {
    if x == 0 {
        return false;
    }
    // x = n*(3n-1)/2 => 3n^2 - n - 2x = 0 => n = (1 + sqrt(1+24x))/6
    let disc = 1 + 24 * x;
    let s = (disc as f64).sqrt() as u64;
    for m in s.saturating_sub(1)..=s + 1 {
        if m * m == disc && (1 + m) % 6 == 0 {
            return true;
        }
    }
    false
}

fn main() {
    let limit = 3000;
    let pents: Vec<u64> = (1..=limit).map(pentagonal).collect();

    let mut min_d = u64::MAX;
    for j in 1..pents.len() {
        for k in 0..j {
            let sum = pents[j] + pents[k];
            let diff = pents[j] - pents[k];
            if is_pentagonal(sum) && is_pentagonal(diff) && diff < min_d {
                min_d = diff;
            }
        }
    }

    println!("{min_d}");
}
