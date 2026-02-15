// Project Euler 392: Enmeshed Unit Circle

fn main() {
    let n = 400;
    let half = n / 2 + 1;

    let mut lo = 1e-15f64;
    let mut hi = 1.0 - 1e-15;

    for _ in 0..200 {
        let mid = (lo + hi) / 2.0;
        let mut x_prev2 = 0.0f64;
        let mut x_prev1 = mid;
        let mut ok = true;

        for _ in 2..=half {
            let s1 = (1.0 - x_prev1 * x_prev1).sqrt();
            let s2 = (1.0 - x_prev2 * x_prev2).sqrt();
            let x_new = x_prev1 - (s1 - s2) * s1 / x_prev1;
            if x_new > 1.0 { ok = false; break; }
            x_prev2 = x_prev1;
            x_prev1 = x_new;
        }

        if !ok || x_prev1 > 1.0 { hi = mid; } else { lo = mid; }
    }

    let mid = (lo + hi) / 2.0;
    let mut x_prev2 = 0.0f64;
    let mut x_prev1 = mid;
    let mut area = x_prev1;

    for _ in 2..=half {
        let s1 = (1.0 - x_prev1 * x_prev1).sqrt();
        let s2 = (1.0 - x_prev2 * x_prev2).sqrt();
        let x_new = x_prev1 - (s1 - s2) * s1 / x_prev1;
        area += (x_new - x_prev1) * s1;
        x_prev2 = x_prev1;
        x_prev1 = x_new;
    }

    println!("{:.10}", 4.0 * area);
}
