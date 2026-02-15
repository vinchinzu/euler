// Project Euler 573 - Unfair Race
//
// E_N = sum_{k=1}^{N} C(N,k) * k^k * (N-k)^(N-k) / N^N
// Compute using log-space to avoid overflow.
// N = 1000000.
//
// Key precision fix: use lgamma (from libm) for log-factorial instead of
// accumulating sum of ln(i), which loses precision over 10^6 terms.

unsafe extern "C" {
    fn lgamma(x: f64) -> f64;
}

fn log_gamma(x: f64) -> f64 {
    // SAFETY: lgamma is a pure mathematical function from libm
    unsafe { lgamma(x) }
}

fn main() {
    let n: usize = 1_000_000;
    let nf = n as f64;

    // log(n!) = lgamma(n+1)
    // log(C(n,k)) = lgamma(n+1) - lgamma(k+1) - lgamma(n-k+1)
    let lg_n1 = log_gamma((n + 1) as f64);
    let ln_n = nf.ln();

    let mut ans: f64 = 1.0; // k = N case: C(N,N)*N^N*0^0/N^N = 1

    for k in 1..n {
        let kf = k as f64;
        let nkf = (n - k) as f64;
        let log_binom = lg_n1 - log_gamma((k + 1) as f64) - log_gamma((n - k + 1) as f64);
        let log_term = log_binom + kf * kf.ln() + nkf * nkf.ln() - nf * ln_n;
        ans += log_term.exp();
    }

    println!("{:.4}", ans);
}
