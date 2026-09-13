// Project Euler 901 - Drilling problem
// Binary search for critical d1, then compute expected cost

#[inline(always)]
fn check_growth(d1_val: f64) -> bool {
    let mut d_prev = 0.0;
    let mut d_curr = d1_val;
    
    for _ in 0..220 {
        let diff = d_curr - d_prev;
        if diff > 14.0 || d_curr > 360.0 {
            return true;
        }
        let d_next = diff.exp();
        if d_next <= d_curr {
            return false;
        }
        d_prev = d_curr;
        d_curr = d_next;
    }
    true
}

fn main() {
    let mut low = 0.7;
    let mut high = 0.8;

    for _ in 0..55 {
        let mid = (low + high) * 0.5;
        if check_growth(mid) {
            high = mid;
        } else {
            low = mid;
        }
    }

    let optimal_d1 = high;
    let mut cost = optimal_d1 + 1.0;
    let mut d_prev = 0.0;
    let mut d_curr = optimal_d1;

    loop {
        let term = (-d_curr).exp();
        if term < 4e-18 {
            break;
        }
        cost += term;
        let diff = d_curr - d_prev;
        d_prev = d_curr;
        d_curr = diff.exp();
    }

    println!("{:.9}", cost);
}
