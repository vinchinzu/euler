// Project Euler 938 - Exhausting a Colour
//
// Red cards are discarded in pairs, so odd R cannot reach all-black (P=0).
// For even R=2a the DP
//   F(a,b) = ((2a-1) F(a-1,b) + 2b F(a,b-1)) / ((2a-1)+2b)
// transforms (via C(a,b) = Γ(a+b+1/2)/(Γ(a+1/2) Γ(b+1))) into a Pascal
// recurrence whose solution is
//   F(a,b) = U(a,b) / C(a,b),
//   U(a,b) = sum_{k=1}^b (C(2k,k)/4^k) * C(a+b-k-1, a-1).
// Terms decay geometrically; evaluate the sum by successive ratios and
// the prefactor in log-space.

unsafe extern "C" {
    fn lgamma(x: f64) -> f64;
}

fn ln_gamma(x: f64) -> f64 {
    // SAFETY: libm lgamma is a pure math function (signgam unused).
    unsafe { lgamma(x) }
}

fn p_black(r: usize, b: usize) -> f64 {
    if b == 0 {
        return 0.0;
    }
    if r == 0 {
        return 1.0;
    }
    if r & 1 == 1 {
        return 0.0;
    }
    let a = r / 2;
    if a == 0 {
        return 1.0;
    }

    let af = a as f64;
    let bf = b as f64;

    // S = sum_k term(k)/term(1); term(k+1)/term(k) = (2k+1)/(2k+2)*(b-k)/(a+b-k-1)
    let mut s = 1.0;
    let mut t = 1.0;
    for k in 1..b {
        let kf = k as f64;
        t *= (2.0 * kf + 1.0) / (2.0 * (kf + 1.0)) * (bf - kf) / (af + bf - kf - 1.0);
        s += t;
        if t < 1e-20 {
            break;
        }
    }

    // 0.5 * C(a+b-2, a-1) / C(a,b) = 0.5 * b * Γ(a+b-1) Γ(a+1/2) / (Γ(a) Γ(a+b+1/2))
    let log_pref = (0.5_f64).ln()
        + bf.ln()
        + ln_gamma(af + bf - 1.0)
        - ln_gamma(af)
        + ln_gamma(af + 0.5)
        - ln_gamma(af + bf + 0.5);
    log_pref.exp() * s
}

fn main() {
    debug_assert!((p_black(2, 2) - 0.4666666667).abs() < 1e-10);
    debug_assert!((p_black(10, 9) - 0.4118903397).abs() < 1e-10);
    debug_assert!((p_black(34, 25) - 0.3665688069).abs() < 1e-10);

    const R: usize = 24690;
    const B: usize = 12345;
    println!("{:.10}", p_black(R, B));
}
