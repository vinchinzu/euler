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
    const LIMIT: u32 = 10_000_000;

    // Precompute: max digit square sum for 7-digit number is 7*81 = 567
    let mut memo = [false; 568];
    for i in 1..568u32 {
        let mut n = i;
        while n != 1 && n != 89 {
            n = next_number(n);
        }
        memo[i as usize] = n == 89;
    }

    let mut count = 0u32;
    for i in 1..LIMIT {
        let n = next_number(i);
        if memo[n as usize] {
            count += 1;
        }
    }

    println!("{count}");
}
