// Project Euler 78: Coin partitions
// Find the smallest n such that p(n) is divisible by 1,000,000.
// Uses the pentagonal number theorem for efficient partition computation.

fn main() {
    let modulus = 1_000_000i64;
    let max_n = 100_000usize;
    let mut p = vec![0i64; max_n + 1];
    p[0] = 1;

    for n in 1..=max_n {
        let mut current = 0i64;
        let mut k = 1i64;
        loop {
            let pent1 = (k * (3 * k - 1) / 2) as usize;
            if pent1 > n { break; }
            let sign = if k % 2 == 1 { 1 } else { -1 };
            current += sign * p[n - pent1];

            let pent2 = (k * (3 * k + 1) / 2) as usize;
            if pent2 <= n {
                current += sign * p[n - pent2];
            }
            k += 1;
        }
        p[n] = ((current % modulus) + modulus) % modulus;

        if p[n] == 0 {
            println!("{n}");
            return;
        }
    }
}
