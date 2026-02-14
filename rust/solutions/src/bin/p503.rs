// Project Euler 503 - Alice's Game
// Optimal stopping problem. Expected score with N=10^6.

fn main() {
    let n: usize = 1_000_000;
    let mut ans = n as f64;

    for nn in (1..=n).rev() {
        let d = (n + 1) as f64 / (nn + 1) as f64;
        let k = (ans / d) as usize;
        let tr_k = k as f64 * (k + 1) as f64 / 2.0;
        ans = (tr_k * d + (nn - k) as f64 * ans) / nn as f64;
    }

    println!("{:.10}", ans);
}
