// Project Euler 461: Almost Pi
fn main() {
    const N: usize = 10000;
    let pi = std::f64::consts::PI;

    let mut f = vec![0.0f64; 2 * N + 1];
    for i in 0..=2 * N {
        f[i] = (i as f64 / N as f64).exp() - 1.0;
    }

    // Count pairs
    let mut count = 0usize;
    let mut k1 = 1;
    while f[k1] < pi {
        let mut k2 = k1;
        while f[k1] + f[k2] < pi {
            count += 1;
            k2 += 1;
        }
        k1 += 1;
    }

    let mut pairs: Vec<(f64, i32, i32)> = Vec::with_capacity(count);
    k1 = 1;
    while f[k1] < pi {
        let mut k2 = k1;
        while f[k1] + f[k2] < pi {
            pairs.push((f[k1] + f[k2], k1 as i32, k2 as i32));
            k2 += 1;
        }
        k1 += 1;
    }

    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut left = 0usize;
    let mut right = pairs.len() - 1;
    let mut min_error = 1e100f64;
    let mut min_left = 0usize;
    let mut min_right = 0usize;

    while left <= right {
        let error = pairs[left].0 + pairs[right].0 - pi;
        if error.abs() < min_error {
            min_error = error.abs();
            min_left = left;
            min_right = right;
        }
        if error < 0.0 {
            left += 1;
        } else {
            if right == 0 { break; }
            right -= 1;
        }
    }

    let target_left = pairs[min_left].0;
    let target_right = pairs[min_right].0;

    let mut ans: u64 = 0;
    k1 = 1;
    while f[k1] < pi {
        let mut k2 = k1;
        while f[k1] + f[k2] < pi {
            let s = f[k1] + f[k2];
            if s == target_left || s == target_right {
                ans += k1 as u64 * k1 as u64 + k2 as u64 * k2 as u64;
            }
            k2 += 1;
        }
        k1 += 1;
    }

    println!("{}", ans);
}
