// Project Euler 901 - Drilling problem
// Binary search for critical d1, then compute expected cost using long double precision

fn main() {
    let check_growth = |d1_val: f64| -> bool {
        let mut d_prev = 0.0f64;
        let mut d_curr = d1_val;
        for _ in 0..1000 {
            if d_curr - d_prev > 20.0 {
                return true;
            }
            let d_next = (d_curr - d_prev).exp();
            if d_next <= d_curr {
                return false;
            }
            d_prev = d_curr;
            d_curr = d_next;
            if d_curr > 1000.0 {
                return true;
            }
        }
        true
    };

    let mut low = 0.7f64;
    let mut high = 0.8f64;

    for _ in 0..200 {
        let mid = (low + high) / 2.0;
        if check_growth(mid) {
            high = mid;
        } else {
            low = mid;
        }
    }

    let optimal_d1 = high;

    // Calculate Expected Cost: E = d1 + 1 + sum(exp(-d_k))
    let mut cost = optimal_d1 + 1.0;
    let mut d_prev = 0.0f64;
    let mut d_curr = optimal_d1;

    loop {
        let term = (-d_curr).exp();
        cost += term;
        if term < 1e-25 {
            break;
        }
        let d_next = (d_curr - d_prev).exp();
        d_prev = d_curr;
        d_curr = d_next;
    }

    println!("{:.9}", cost);
}
