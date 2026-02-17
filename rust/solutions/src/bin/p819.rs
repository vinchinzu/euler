// Project Euler Problem 819: Iterative Sampling
// Compute expected number of steps until all entries are equal

fn expected_steps(n: usize) -> f64 {
    if n <= 1 {
        return 0.0;
    }

    // T[m] = expected remaining steps starting from m distinct labels
    let mut t = vec![0.0; n + 1];

    // p[k] = probability of k occupied bins for current m
    let mut p = vec![0.0; n + 1];
    p[0] = 1.0;
    let inv_n = 1.0 / n as f64;

    for m in 1..=n {
        // Update occupancy distribution
        for k in (1..=m).rev() {
            p[k] = p[k] * (k as f64 * inv_n) + p[k - 1] * ((n - k + 1) as f64 * inv_n);
        }
        p[0] = 0.0;

        if m == 1 {
            continue;
        }

        let stay = p[m];
        let acc: f64 = (1..m).map(|k| p[k] * t[k]).sum();

        // T[m] = (1 + acc) / (1 - stay)
        t[m] = (1.0 + acc) / (1.0 - stay);
    }

    t[n]
}

fn main() {
    // Test: E(3) = 27/7 ≈ 3.857142857
    let e3 = expected_steps(3);
    assert!((e3 - 27.0 / 7.0).abs() < 1e-12);

    // Test: E(5) = 468125/60701 ≈ 7.711982
    let e5 = expected_steps(5);
    assert!((e5 - 468125.0 / 60701.0).abs() < 1e-12);

    let result = expected_steps(1000);
    println!("{:.6}", result);
}
