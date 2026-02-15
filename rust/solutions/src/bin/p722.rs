// Project Euler 722 - Slowly Converging Series
//
// Polynomial polylogarithm approach with floating-point polynomials.

const MAX_POLY: usize = 50;

fn poly_trim(c: &mut Vec<f64>) {
    while c.len() > 1 && c.last().map_or(false, |&v| v.abs() < 1e-15) {
        c.pop();
    }
}

fn poly_derivative(p: &[f64]) -> Vec<f64> {
    if p.len() <= 1 {
        return vec![0.0];
    }
    let mut r: Vec<f64> = (1..p.len()).map(|i| p[i] * i as f64).collect();
    poly_trim(&mut r);
    r
}

fn poly_multiply(a: &[f64], b: &[f64]) -> Vec<f64> {
    let mut newlen = a.len() + b.len() - 1;
    if newlen > MAX_POLY {
        newlen = MAX_POLY;
    }
    let mut r = vec![0.0; newlen];
    for i in 0..a.len() {
        for j in 0..b.len() {
            if i + j < MAX_POLY {
                r[i + j] += a[i] * b[j];
            }
        }
    }
    poly_trim(&mut r);
    r
}

fn poly_add(a: &[f64], b: &[f64]) -> Vec<f64> {
    let maxlen = a.len().max(b.len());
    let mut r = vec![0.0; maxlen];
    for i in 0..a.len() {
        r[i] += a[i];
    }
    for i in 0..b.len() {
        r[i] += b[i];
    }
    poly_trim(&mut r);
    r
}

fn poly_shift_up(p: &[f64], n: usize) -> Vec<f64> {
    let newlen = (p.len() + n).min(MAX_POLY);
    let mut r = vec![0.0; newlen];
    for i in 0..p.len() {
        if i + n < MAX_POLY {
            r[i + n] = p[i];
        }
    }
    r
}

fn poly_scale(p: &[f64], s: f64) -> Vec<f64> {
    p.iter().map(|&c| c * s).collect()
}

fn poly_eval(p: &[f64], x: f64) -> f64 {
    let mut result = 0.0;
    let mut power = 1.0;
    for &c in p {
        result += c * power;
        power *= x;
    }
    result
}

fn main() {
    let q = 1.0 - (0.5f64).powi(25);
    let k = 15;

    let mut num = vec![0.0, 1.0]; // x
    let den = vec![1.0, -1.0]; // 1 - x

    // Apply f(z) -> zf'(z) K times
    for i in 1..=k {
        let num_deriv = poly_derivative(&num);
        let nd_times_den = poly_multiply(&num_deriv, &den);
        let num_times_i = poly_scale(&num, i as f64);
        let sum = poly_add(&nd_times_den, &num_times_i);
        num = poly_shift_up(&sum, 1);
    }

    let mut ans = 0.0;
    let mut prev_ans = -1.0;
    for i in 1..=1000 {
        let z = q.powi(i);
        let num_val = poly_eval(&num, z);
        let den_val = (1.0 - z).powi(k + 1);
        ans += num_val / den_val;
        if (ans - prev_ans).abs() < 1e-10 {
            break;
        }
        prev_ans = ans;
    }

    // Format output
    let buf = format!("{:.12e}", ans);
    let out: String = buf.replace("e+", "e");
    println!("{}", out);
}
