// Project Euler 713 - Turan Graphs
//
// T(N, m) sum over m = 2..N using floor quotient grouping.

fn main() {
    let n: i64 = 10_000_000;
    let mut ans: i64 = 0;

    for k in 1..n {
        let gs = n / k;
        let rem = n % k;
        let c1 = gs * (gs + 1) / 2;
        let c2 = gs * (gs - 1) / 2;
        ans += rem * c1 + (k - rem) * c2;
    }

    println!("{}", ans);
}
