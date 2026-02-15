// Project Euler 756 - Approximating a Sum
// E(Delta | phi(k), n, m) = sum_{j=1}^{n-m} phi(j) * C(n-j, m) / C(n, m)
//
// where C(n-j,m)/C(n,m) = prod_{t=0}^{j-1} (n-t-m)/(n-t)
//
// Compute directly to avoid catastrophic cancellation from S - (large sum).
// Use Kahan compensated summation for precision.

fn main() {
    let n: usize = 12_345_678;
    let m: usize = 12_345;

    // Sieve for Euler's totient
    let mut phi = vec![0u32; n + 1];
    for i in 0..=n {
        phi[i] = i as u32;
    }
    for i in 2..=n {
        if phi[i] == i as u32 {
            // i is prime
            for j in (i..=n).step_by(i) {
                phi[j] = phi[j] / i as u32 * (i as u32 - 1);
            }
        }
    }

    // Compute E[Delta] = sum_{j=1}^{n-m} phi(j) * w_j
    // where w_j = prod_{t=0}^{j-1} (n-t-m)/(n-t)
    // w_0 = 1, w_j = w_{j-1} * (n - j + 1 - m) / (n - j + 1)
    //
    // Use Kahan compensated summation for precision.
    let mut ans: f64 = 0.0;
    let mut comp: f64 = 0.0; // Kahan compensation
    let mut w: f64 = 1.0;

    let max_j = n - m; // beyond this, C(n-j, m) = 0

    for j in 1..=max_j {
        // Update weight: w_j = w_{j-1} * (n - j + 1 - m) / (n - j + 1)
        let numerator = (n - j + 1 - m) as f64;
        let denominator = (n - j + 1) as f64;
        w *= numerator / denominator;

        if w < 1e-18 {
            break; // negligible contribution
        }

        let term = phi[j] as f64 * w;

        // Kahan summation
        let y = term - comp;
        let t = ans + y;
        comp = (t - ans) - y;
        ans = t;
    }

    println!("{:.6}", ans);
}
