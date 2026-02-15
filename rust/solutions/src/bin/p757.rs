// Project Euler 757 - Stealthy Numbers
// A stealthy number is n = x(x+1)*g(g+1) for some x,g >= 1. Count distinct <= N.

fn main() {
    const N: i64 = 100_000_000_000_000; // 10^14

    let mut stealthies: Vec<i64> = Vec::with_capacity(50_000_000);

    let mut x: i64 = 1;
    while x * (x + 1) * x * (x + 1) <= N {
        let mut g = x;
        while x * (x + 1) * g * (g + 1) <= N {
            stealthies.push(x * (x + 1) * g * (g + 1));
            g += 1;
        }
        x += 1;
    }

    stealthies.sort_unstable();

    let mut count = 0u64;
    for i in 0..stealthies.len() {
        if i == 0 || stealthies[i] != stealthies[i - 1] {
            count += 1;
        }
    }

    println!("{}", count);
}
