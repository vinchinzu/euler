// Project Euler 92: Square digit chains
// Count numbers below 10,000,000 that reach 89 in the chain.

fn next_number(mut n: u32) -> u32 {
    let mut sum = 0;
    while n > 0 {
        let d = n % 10;
        sum += d * d;
        n /= 10;
    }
    sum
}

fn main() {
    const MAX: usize = 7 * 81; // 567

    let mut to89 = [false; MAX + 1];
    for i in 1..=MAX {
        let mut n = i as u32;
        while n != 1 && n != 89 {
            n = next_number(n);
        }
        to89[i] = n == 89;
    }

    // 7 digits with leading zeros cover 0..9_999_999; exclude 0 (sum 0).
    let mut dp = [0u32; MAX + 1];
    dp[0] = 1;
    for _ in 0..7 {
        let mut ndp = [0u32; MAX + 1];
        for s in 0..=MAX {
            let c = dp[s];
            if c == 0 {
                continue;
            }
            for d in 0..10 {
                let ns = s + d * d;
                if ns <= MAX {
                    ndp[ns] += c;
                }
            }
        }
        dp = ndp;
    }

    let mut count = 0u32;
    for s in 1..=MAX {
        if to89[s] {
            count += dp[s];
        }
    }
    println!("{count}");
}
