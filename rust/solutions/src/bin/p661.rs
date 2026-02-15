// Project Euler 661 - A Long Chess Match
// Tridiagonal system (Thomas algorithm) for expected leads.

fn tridiagonal_solve(a: &[f64], b: &[f64], c: &[f64], d: &[f64]) -> Vec<f64> {
    let n = a.len();
    let mut cp = vec![0.0; n];
    let mut dp = vec![0.0; n];
    cp[0] = c[0] / b[0];
    dp[0] = d[0] / b[0];
    for i in 1..n {
        let denom = b[i] - a[i] * cp[i - 1];
        cp[i] = c[i] / denom;
        dp[i] = (d[i] - a[i] * dp[i - 1]) / denom;
    }
    let mut x = vec![0.0; n];
    x[n - 1] = dp[n - 1];
    for i in (0..n-1).rev() { x[i] = dp[i] - cp[i] * x[i + 1]; }
    x
}

fn e_func(pa: f64, pb: f64, p: f64) -> f64 {
    let mut prev_guess = -1e300;
    let mut l = 1;
    loop {
        let sz = 2 * l + 1;
        let mut a = vec![0.0; sz];
        let mut b = vec![0.0; sz];
        let mut c = vec![0.0; sz];
        let mut d = vec![0.0; sz];
        for diff in -(l as i32)..=(l as i32) {
            let idx = (diff + l as i32) as usize;
            a[idx] = -(1.0 - p) * pb;
            b[idx] = 1.0 - (1.0 - p) * (1.0 - pa - pb);
            c[idx] = -(1.0 - p) * pa;
            if diff >= 0 { d[idx] += pa; }
            if diff >= 2 { d[idx] += pb; }
            if diff >= 1 { d[idx] += 1.0 - pa - pb; }
        }
        let x = tridiagonal_solve(&a, &b, &c, &d);
        let guess = x[l];
        if (prev_guess - guess).abs() < 1e-10 { return guess; }
        prev_guess = guess;
        l *= 2;
    }
}

fn main() {
    let mut ans = 0.0;
    for k in 3..=50 {
        let pa = 1.0 / (k as f64 + 3.0).sqrt();
        let pb = pa + 1.0 / ((k * k) as f64);
        let p = 1.0 / ((k * k * k) as f64);
        ans += e_func(pa, pb, p);
    }
    println!("{:.4}", ans);
}
