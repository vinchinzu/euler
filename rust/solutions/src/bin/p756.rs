// Project Euler 756 - Approximating a Sum
// Euler's totient sieve with long-double-like accumulation.

fn main() {
    let n: usize = 12_345_678;
    let k: usize = 12_345;

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

    // Prefix sums
    let mut sum_phis = vec![0i64; n + 1];
    for i in 1..=n {
        sum_phis[i] = sum_phis[i - 1] + phi[i] as i64;
    }
    drop(phi);

    // Accumulation using f64 (sufficient for 6 decimal digits)
    let mut d: f64 = k as f64 / n as f64;
    let mut ans: f64 = sum_phis[n] as f64;

    for i in 1..=n {
        let tail = sum_phis[n] - sum_phis[i - 1];
        let diff = d * tail as f64;

        if diff == 0.0 { break; }

        ans -= diff;

        let nr = n as i64 - k as i64 - i as i64 + 1;
        if nr <= 0 { break; }
        let dn = n as i64 - i as i64;
        if dn <= 0 { break; }

        d = d * nr as f64 / dn as f64;
    }

    println!("{:.6}", ans);
}
