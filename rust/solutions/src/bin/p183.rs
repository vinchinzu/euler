// Project Euler 183: Maximum product of parts
use euler_utils::gcd;

fn main() {
    let (start_n, end_n) = (5u64, 10000u64);
    let mut sum_val: i64 = 0;

    for n in start_n..=end_n {
        let k0 = (n as f64 / std::f64::consts::E) as i64;
        let mut candidates = vec![1i64, n as i64];
        for delta in -1..=2i64 {
            let k = k0 + delta;
            if k >= 1 && k <= n as i64 {
                candidates.push(k);
            }
        }

        let mut best_k = 1i64;
        let mut best_val = f64::NEG_INFINITY;
        for &k in &candidates {
            let val = k as f64 * (n as f64 / k as f64).ln();
            if val > best_val {
                best_val = val;
                best_k = k;
            }
        }

        let gcd_val = gcd(n, best_k as u64);
        let mut reduced_k = best_k as u64 / gcd_val;
        while reduced_k % 2 == 0 { reduced_k /= 2; }
        while reduced_k % 5 == 0 { reduced_k /= 5; }

        sum_val += if reduced_k == 1 { -(n as i64) } else { n as i64 };
    }

    println!("{}", sum_val);
}
