// Project Euler 645 - Every Day is a Holiday
// Expected number of emperors until every day is a holiday

fn n_choose_r(n: i32, k: i32) -> f64 {
    if k < 0 || k > n { return 0.0; }
    let mut result = 1.0f64;
    for i in 0..k {
        result = result * (n - i) as f64 / (i + 1) as f64;
    }
    result
}

fn main() {
    let n = 10000i32;

    let mut p = 1.0f64;
    let mut ans = 1.0f64;
    for k in (1..n).rev() {
        ans += (1.0 - p) * n as f64 / (n - k) as f64;
        if k >= 2 && 2 * k - n >= 2 {
            p *= n_choose_r(2 * k - n, 2) / n_choose_r(k, 2);
        } else {
            p = 0.0;
        }
    }

    println!("{:.4}", ans);
}
