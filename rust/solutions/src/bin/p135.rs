// Project Euler 135: Same differences
// Count n < 1,000,000 with exactly 10 solutions to x^2 - y^2 - z^2 = n
// where x, y, z are positive integers in arithmetic progression.

fn main() {
    const LIMIT: usize = 1_000_000;
    const TARGET: i32 = 10;
    let mut counts = vec![0i32; LIMIT];

    let max_d = LIMIT / 4;

    for d in 1..=max_d {
        let threshold = 4 * d * d;
        let max_k = 3 * d - 1;

        if threshold <= LIMIT {
            for k in 1..=max_k {
                let n = k * (4 * d - k);
                if n > 0 && n < LIMIT {
                    counts[n] += 1;
                }
            }
            continue;
        }

        let s = ((threshold - LIMIT) as f64).sqrt();
        let k1 = ((2.0 * d as f64 - s).ceil() as usize).max(1);
        let k2 = (2.0 * d as f64 + s).floor() as usize;

        for k in 1..k1 {
            let n = k * (4 * d - k);
            if n > 0 && n < LIMIT {
                counts[n] += 1;
            }
        }

        if k2 < max_k {
            for k in (k2 + 1)..=max_k {
                let n = k * (4 * d - k);
                if n > 0 && n < LIMIT {
                    counts[n] += 1;
                }
            }
        }
    }

    let result = counts[1..].iter().filter(|&&c| c == TARGET).count();
    println!("{}", result);
}
