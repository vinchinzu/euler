// Project Euler 666 - Polymorphic Bacteria
// Fixed point iteration for extinction probability.

fn main() {
    let n = 500usize;
    let k = 10usize;
    let mut r_seq = vec![0i32; n * k];
    let mut rv = 306i32;
    for i in 0..n*k {
        r_seq[i] = rv;
        rv = (rv * rv) % 10007;
    }
    let mut probs = vec![0.5f64; n];
    loop {
        let mut new_probs = vec![0.0f64; n];
        for i in 0..n {
            for j in 0..k {
                let q = r_seq[i * k + j] % 5;
                match q {
                    0 => new_probs[i] += 1.0 / k as f64,
                    1 => new_probs[i] += probs[i] * probs[i] / k as f64,
                    2 => new_probs[i] += probs[(2 * i) % n] / k as f64,
                    3 => { let idx = (i * i + 1) % n; new_probs[i] += probs[idx].powi(3) / k as f64; },
                    4 => new_probs[i] += probs[i] * probs[(i + 1) % n] / k as f64,
                    _ => {}
                }
            }
        }
        if (probs[0] - new_probs[0]).abs() < 1e-10 {
            println!("{:.8}", new_probs[0]);
            return;
        }
        probs = new_probs;
    }
}
