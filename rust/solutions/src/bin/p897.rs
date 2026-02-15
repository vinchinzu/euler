// Project Euler 897
// Area optimization via coordinate descent with cubic root.

fn calculate_area(x: &[f64]) -> f64 {
    let mut total_trap = 0.0;
    for i in 0..x.len() - 1 {
        let u = x[i];
        let v = x[i + 1];
        let h = v - u;
        let u4 = u * u * u * u;
        let v4 = v * v * v * v;
        let avg_height = (u4 + v4) / 2.0;
        total_trap += h * avg_height;
    }
    2.0 - total_trap
}

fn cbrt_signed(y: f64) -> f64 {
    if y >= 0.0 { y.cbrt() } else { -(-y).cbrt() }
}

fn solve_for_n(n: usize) -> f64 {
    let m = n - 1;
    let mut x = vec![0.0f64; m + 1];
    for i in 0..=m {
        x[i] = -1.0 + 2.0 * i as f64 / m as f64;
    }

    for _iter in 0..100_000 {
        let mut max_diff = 0.0f64;
        for k in 1..m {
            let xp = x[k + 1];
            let xm = x[k - 1];
            let rhs = (xp * xp + xm * xm) * (xp + xm);
            let new_xk = cbrt_signed(rhs / 4.0);
            let diff = (new_xk - x[k]).abs();
            if diff > max_diff { max_diff = diff; }
            x[k] = new_xk;
        }
        if max_diff < 1e-13 { break; }
    }

    calculate_area(&x)
}

fn main() {
    let result = solve_for_n(101);
    println!("{:.9}", result);
}
