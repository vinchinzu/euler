// Project Euler 142: Perfect Square Collection
fn is_square(n: i64) -> bool {
    if n < 0 { return false; }
    let r = (n as f64).sqrt() as i64;
    for t in (r - 1).max(0)..=r + 1 {
        if t * t == n { return true; }
    }
    false
}

fn main() {
    let mut min_sum: i64 = -1;
    let limit_k5 = 1000;

    let mut k5 = 2;
    while k5 <= limit_k5 {
        let mut k6 = 2;
        while k6 < k5 {
            let k5_sq = k5 as i64 * k5 as i64;
            let k6_sq = k6 as i64 * k6 as i64;

            let y_val = (k5_sq + k6_sq) / 2;
            let z_val = (k5_sq - k6_sq) / 2;
            let z2_val = 2 * z_val;

            let p_limit = (z2_val as f64).sqrt() as i64;
            let mut p_factor = 2i64;
            while p_factor <= p_limit {
                if z2_val % p_factor == 0 {
                    let q_factor = z2_val / p_factor;
                    if q_factor % 2 == 0 && p_factor < q_factor {
                        let k3 = (p_factor + q_factor) / 2;
                        let k4 = (q_factor - p_factor) / 2;
                        let x_val = (k3 * k3 + k4 * k4) / 2;

                        let k1_sq_val = x_val + y_val;
                        let k2_sq_val = x_val - y_val;

                        if k2_sq_val > 0 && is_square(k1_sq_val) && is_square(k2_sq_val) {
                            let k1 = (k1_sq_val as f64).sqrt().round() as i64;
                            let k2 = (k2_sq_val as f64).sqrt().round() as i64;
                            if k1 % 2 == k2 % 2 && k1 % 2 == x_val % 2 {
                                let sum_val = x_val + y_val + z_val;
                                if min_sum < 0 || sum_val < min_sum {
                                    min_sum = sum_val;
                                }
                            }
                        }
                    }
                }
                p_factor += 2;
            }
            k6 += 2;
        }
        k5 += 2;
    }
    println!("{}", min_sum);
}
