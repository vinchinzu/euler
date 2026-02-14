// Project Euler 113: Non-bouncy Numbers Below 10^100
// Count = C(N+10,10) + C(N+9,9) - 10*N - 2

fn comb(n: i64, k: i64) -> i64 {
    let k = k.min(n - k);
    let mut result: i64 = 1;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

fn main() {
    let n = 100i64;
    let count = comb(n + 10, 10) + comb(n + 9, 9) - 10 * n - 2;
    println!("{count}");
}
