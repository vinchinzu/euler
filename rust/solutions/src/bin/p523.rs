// Project Euler 523 - First Sort I
//
// Expected number of moves: sum over n=2..N of sum over i=0..n-2 of 2^i / n.

fn main() {
    let n_max = 30;
    let mut ans = 0.0f64;

    for n in 2..=n_max {
        let mut pow2 = 1.0f64;
        for _ in 0..n - 1 {
            ans += pow2 / n as f64;
            pow2 *= 2.0;
        }
    }

    println!("{:.2}", ans);
}
