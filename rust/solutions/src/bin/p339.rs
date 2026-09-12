// Project Euler 339: Peredur fab Efrawg
//
// Optimal stopping on the birth-death chain of black-sheep counts. The
// Snell envelope is linear in the binomial scale function, so the DP
// collapses to an O(n) recurrence on the midpoint values B[b] = S_{2b}(b)
// driven by central binomial probabilities.

fn expected_black(n: usize) -> f64 {
    // B[b] = S_{2b}(b). Only B[n] and B[n-1] are needed at the end.
    let mut b_cur = 0.0f64;
    let mut b_prev = 0.0f64;
    // p = C(N, N/2) / 2^N for even N, starting at N = 0.
    let mut p = 1.0f64;
    // q = C(N, n) / 2^N for odd N = 2n-1, starting at N = 1.
    let mut q = 0.5f64;

    for b in 1..=n {
        let r = (2.0 * p) / (1.0 + p);
        let m = (2 * b - 1) as f64;
        b_prev = b_cur;
        b_cur += (m - b_cur) * r;
        let bf = b as f64;
        p *= (2.0 * bf - 1.0) / (2.0 * bf);
        if b < n {
            q *= (2.0 * bf + 1.0) / (2.0 * bf + 2.0);
        }
    }

    let r2 = 2.0 * q;
    let m2 = (2 * n) as f64;
    let s_n_plus_1 = b_cur + (m2 - b_cur) * r2;
    // E(n) = (S_{2n}(n-1) + S_{2n}(n+1)) / 2, and S_{2n}(n-1) = B[n-1].
    0.5 * (b_prev + s_n_plus_1)
}

fn main() {
    debug_assert!((expected_black(5) - 6.871346).abs() < 5e-7);
    println!("{:.6}", expected_black(10_000));
}
