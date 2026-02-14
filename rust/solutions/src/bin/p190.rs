fn main() {
    let mut total: i64 = 0;
    for m in 2..=15 {
        let mut log_val: f64 = 0.0;
        for k in 1..=m {
            log_val += k as f64 * (2.0 * k as f64 / (m as f64 + 1.0)).ln();
        }
        let val = log_val.exp();
        let mut floored = val as i64;
        if val - floored as f64 > 0.9999999 {
            floored += 1;
        }
        total += floored;
    }
    println!("{}", total);
}
