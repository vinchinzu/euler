// Project Euler 887
// N = 7^10, K = 7. Binary representation analysis.

fn main() {
    let mut n: i64 = 1;
    for _ in 0..10 { n *= 7; }

    let k = 7;
    let mut ans = (n - 1) * n / 2;

    for d in 1..=k {
        let mut prev_k = 1i64;
        let mut t = 1i64;
        while prev_k < n {
            let mut kv = 1i64 << t; // 2^t
            if t > d {
                kv += t + 1 - d as i64 - (1i64 << (t - d as i64));
            }
            if kv > n { kv = n; }
            ans += (kv - prev_k) * t;
            prev_k = kv;
            t += 1;
        }
    }
    println!("{}", ans);
}
