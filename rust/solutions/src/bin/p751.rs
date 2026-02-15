// Project Euler 751 - Concatenation Coincidence
// Binary search on theta using 128-bit fixed-point arithmetic (30 decimal digits).

fn main() {
    let prec: u128 = 30;
    let scale: u128 = 10u128.pow(prec as u32);
    let k = 24usize;

    let mut low: i128 = 2 * scale as i128;
    let mut high: i128 = 3 * scale as i128;

    let mut prev_ans = String::new();
    let mut ans = String::new();

    for _ in 0..200 {
        let theta = (low + high) / 2;
        let mut b = theta;
        let mut digits = Vec::new();

        while digits.len() < k {
            let int_b = if b >= 0 {
                b / scale as i128
            } else if b % scale as i128 != 0 {
                b / scale as i128 - 1
            } else {
                b / scale as i128
            };

            let frac = b - int_b * scale as i128;

            // new_b = int_b * frac + int_b * SCALE
            b = int_b * frac + int_b * scale as i128;

            // floor(new_b / SCALE)
            let new_int_b = if b >= 0 {
                b / scale as i128
            } else if b % scale as i128 != 0 {
                b / scale as i128 - 1
            } else {
                b / scale as i128
            };

            let s = new_int_b.to_string();
            for ch in s.chars() {
                if digits.len() < k {
                    digits.push(ch);
                }
            }
        }

        let int_theta = theta / scale as i128;
        let mut tau = String::new();
        tau.push_str(&int_theta.to_string());
        tau.push('.');
        for &ch in digits.iter().take(k) {
            tau.push(ch);
        }

        // Parse tau back to fixed-point
        let tau_bytes: Vec<char> = tau.chars().collect();
        let mut tau_val: i128 = (tau_bytes[0] as i128 - '0' as i128) * scale as i128;
        let mut frac_val: i128 = 0;
        for i in 0..k.min(prec as usize) {
            frac_val = frac_val * 10 + (tau_bytes[2 + i] as i128 - '0' as i128);
        }
        for _ in k..prec as usize {
            frac_val *= 10;
        }
        tau_val += frac_val;

        if tau_val > theta {
            low = theta;
        } else {
            high = theta;
        }

        prev_ans = std::mem::replace(&mut ans, tau);
        if prev_ans == ans {
            break;
        }
    }

    println!("{}", ans);
}
