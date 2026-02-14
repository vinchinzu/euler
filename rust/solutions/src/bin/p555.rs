// Project Euler 555 - McCarthy 91 Function
//
// Find sum_{1<=s<k<=N} SF(M, k, s) where M = N = 10^6.
// Fixed points exist when s is divisible by d = k - s.

fn triangular(n: i64) -> i64 {
    if n < 0 { return 0; }
    n * (n + 1) / 2
}

fn main() {
    let n = 1_000_000i64;
    let m_param = 1_000_000i64;

    let mut ans: i64 = 0;
    for d in 1..=n / 2 {
        let mut s = d;
        while s <= n - d {
            let t1 = triangular(m_param + d - s);
            let t2 = triangular(m_param - s);
            ans += t1 - t2;
            s += d;
        }
    }

    println!("{ans}");
}
