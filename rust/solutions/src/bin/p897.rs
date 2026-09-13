// Project Euler 897
// Area optimization via coordinate descent with cubic root.
// Optimized with unsafe array access, inlining, and reduced operations.

#[inline]
fn calculate_area(x: &[f64]) -> f64 {
    let mut total_trap = 0.0;
    let len = x.len();
    for i in 0..len - 1 {
        unsafe {
            let u = *x.get_unchecked(i);
            let v = *x.get_unchecked(i + 1);
            let h = v - u;
            let u2 = u * u;
            let v2 = v * v;
            let avg_height = (u2 * u2 + v2 * v2) * 0.5;
            total_trap += h * avg_height;
        }
    }
    2.0 - total_trap
}

#[inline(always)]
fn cbrt_signed(y: f64) -> f64 {
    if y >= 0.0 { y.cbrt() } else { -(-y).cbrt() }
}

fn solve_for_n(n: usize) -> f64 {
    let m = n - 1;
    let mut x = vec![0.0f64; m + 1];
    let inv_m = 1.0 / m as f64;
    for i in 0..=m {
        x[i] = -1.0 + 2.0 * i as f64 * inv_m;
    }

    const THRESHOLD: f64 = 1e-13;
    const ONE_FOURTH: f64 = 0.25;
    
    for _iter in 0..100_000 {
        let mut max_diff = 0.0f64;
        for k in 1..m {
            unsafe {
                let xp = *x.get_unchecked(k + 1);
                let xm = *x.get_unchecked(k - 1);
                let xk = *x.get_unchecked(k);
                
                let sum = xp + xm;
                let xp2 = xp * xp;
                let xm2 = xm * xm;
                let rhs = (xp2 + xm2) * sum * ONE_FOURTH;
                let new_xk = cbrt_signed(rhs);
                let diff = (new_xk - xk).abs();
                if diff > max_diff {
                    max_diff = diff;
                }
                *x.get_unchecked_mut(k) = new_xk;
            }
        }
        if max_diff < THRESHOLD {
            break;
        }
    }

    calculate_area(&x)
}

fn main() {
    let result = solve_for_n(101);
    println!("{:.9}", result);
}
