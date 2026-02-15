// Project Euler 872 - Rooted Trees
// f(n,k) = sum of nodes on path from root to k in T_n

fn main() {
    let mut n: u128 = 1;
    for _ in 0..17 { n *= 10; }

    let mut k: u128 = 1;
    for _ in 0..17 { k *= 9; }

    let diff = n - k;

    let mut total: u128 = k;
    let mut current: u128 = k;

    // Find highest power of 2 <= diff
    let mut power: u128 = 1;
    while power <= diff { power <<= 1; }
    power >>= 1;

    while power > 0 {
        if diff & power != 0 {
            current += power;
            total += current;
        }
        power >>= 1;
    }

    println!("{}", total);
}
