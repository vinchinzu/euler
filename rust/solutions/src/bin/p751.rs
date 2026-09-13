// Project Euler 751 - Concatenation Coincidence
// Binary search on theta using 128-bit fixed-point arithmetic.

fn main() {
    const PREC: usize = 30;
    const K: usize = 24;
    let scale: i128 = 10i128.pow(PREC as u32);

    let mut low: i128 = 2 * scale;
    let mut high: i128 = 3 * scale;
    let mut prev_tau_val: i128 = 0;

    for _ in 0..200 {
        let theta = (low + high) / 2;
        let mut b = theta;
        let mut digit_buf = [0i128; 32];
        let mut count = 0usize;

        while count < K {
            let int_b = b / scale;
            let frac = b - int_b * scale;
            b = int_b * frac + int_b * scale;
            let d = b / scale;

            if d < 10 {
                unsafe {
                    *digit_buf.get_unchecked_mut(count) = d;
                }
                count += 1;
            } else if d < 100 {
                unsafe {
                    *digit_buf.get_unchecked_mut(count) = d / 10;
                    *digit_buf.get_unchecked_mut(count + 1) = d % 10;
                }
                count += 2;
            } else {
                let mut temp_buf = [0i128; 10];
                let mut tc = 0;
                let mut temp = d;
                while temp > 0 {
                    temp_buf[tc] = temp % 10;
                    temp /= 10;
                    tc += 1;
                }
                while tc > 0 {
                    tc -= 1;
                    unsafe {
                        *digit_buf.get_unchecked_mut(count) = temp_buf[tc];
                    }
                    count += 1;
                    if count >= K {
                        break;
                    }
                }
            }
        }

        let int_theta = theta / scale;
        let mut tau_val = int_theta * scale;
        let mut frac_val: i128 = digit_buf[0];
        for i in 1..K {
            frac_val = frac_val * 10 + digit_buf[i];
        }
        for _ in K..PREC {
            frac_val *= 10;
        }
        tau_val += frac_val;

        if tau_val > theta {
            low = theta;
        } else {
            high = theta;
        }

        if tau_val == prev_tau_val {
            break;
        }
        prev_tau_val = tau_val;
    }

    let int_part = prev_tau_val / scale;
    let mut frac_part = prev_tau_val % scale;
    print!("{}.", int_part);
    for _ in 0..K {
        frac_part *= 10;
        print!("{}", frac_part / scale);
        frac_part %= scale;
    }
    println!();
}
