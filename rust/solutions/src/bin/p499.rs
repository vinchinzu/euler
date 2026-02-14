// Project Euler 499: St. Petersburg Lottery

fn main() {
    let n_val: i64 = 1_000_000_000;
    let k: i64 = 15;
    let l = 50;

    let mut low = 0.0_f64;
    let mut high = 1.0_f64;
    for _ in 0..200 {
        let mid = (low + high) / 2.0;
        let mut res = 0.0_f64;
        for i in 0..l {
            let exponent = ((1i64 << i) - k) as f64;
            res += mid.powf(exponent) / (2i64 << i) as f64;
        }
        if res < 1.0 {
            high = mid;
        } else {
            low = mid;
        }
    }

    let ans = 1.0 - low.powf(n_val as f64);
    println!("{:.7}", ans);
}
