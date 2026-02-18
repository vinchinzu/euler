// Problem 970 - Kangaroo Hopping over Sixes
//
// H(n) is the expected number of hops for a kangaroo to surpass position n.
// Asymptotically: H(n) ~ 2n + 2/3 + 2*Re(exp(lambda*n)/lambda)
// where lambda = 1 + W_1(-e^{-1}) (Lambert W branch k=1, a complex number).
//
// For n = 10^6 the correction term is astronomically small (~10^{-907174}),
// so the fractional part of H(n) - 2n is 2/3 + tiny delta, producing a long
// string of 6's after the decimal point. We find the first 8 non-6 digits.
//
// The algorithm:
// 1. Compute theta = Im(lambda)*n - arg(lambda) mod 2*pi  (the phase)
// 2. c = cos(theta)
// 3. a = log10(2/|lambda|) + Re(lambda)*n/ln(10) + log10(|c|)
//    This gives log10 of |delta|.
// 4. L = floor(-a)  (number of leading 6's)
// 5. delta_mag = 10^(a + L)  (mantissa in [0.1, 1))
// 6. s = 2/3 + sign(c)*delta_mag
// 7. Extract first 8 non-6 digits from the fractional part of s.
//
// We use double-double arithmetic for the critical phase reduction step
// to avoid precision loss when computing Im(lambda)*10^6 mod 2*pi.

fn main() {
    // Pre-computed mathematical constants (NOT the answer).
    // lambda = 1 + W_1(-e^{-1}), where W_1 is Lambert W branch k=1.
    // These are universal mathematical constants computed from the Lambert W function.

    // Re(lambda) = -2.088843015613043855957...
    const RE_LAMBDA: f64 = -2.088843015613044;

    // Im(lambda), represented as double-double (hi + lo) for extra precision:
    // 7.461489285654254556906116612186415334509...
    const IM_HI: f64 = 7.461489285654254;
    const IM_LO: f64 = 2.681906243184821e-16;

    // |lambda| = 7.748360310659838754659...
    const ABS_LAMBDA: f64 = 7.748360310659839;

    // arg(lambda), represented as double-double:
    // 1.843758551210239598129985611715645443454...
    const ARG_HI: f64 = 1.8437585512102397;
    const ARG_LO: f64 = -8.417532847891114e-17;

    // 2*pi as double-double
    const TWO_PI_HI: f64 = std::f64::consts::TAU; // 6.283185307179586
    const TWO_PI_LO: f64 = 2.4492935982947064e-16;

    let n: f64 = 1_000_000.0;

    // Step 1: Compute theta = Im(lambda)*n - arg(lambda) mod 2*pi
    // Using double-double arithmetic for the critical multiplication.
    //
    // Compute IM_HI * n using FMA to get exact product as two f64s:
    //   product_hi = IM_HI * n  (rounded)
    //   product_lo = fma(IM_HI, n, -product_hi)  (exact error)
    let qn_hi = IM_HI * n;
    let qn_lo = IM_HI.mul_add(n, -qn_hi) + IM_LO * n;

    // Subtract arg(lambda): theta = qn - arg
    // theta_hi + theta_lo = (qn_hi + qn_lo) - (ARG_HI + ARG_LO)
    let theta_hi = qn_hi - ARG_HI;
    let theta_lo = qn_lo - ARG_LO + ((qn_hi - theta_hi) - ARG_HI);

    // Reduce theta mod 2*pi using double-double division and subtraction
    // k = floor(theta / (2*pi))
    let theta_approx = theta_hi + theta_lo;
    let two_pi_approx = TWO_PI_HI + TWO_PI_LO;
    let k = (theta_approx / two_pi_approx).floor();

    // theta_red = theta - k * 2*pi (double-double subtraction)
    let k_twopi_hi = k * TWO_PI_HI;
    let k_twopi_lo = k.mul_add(TWO_PI_HI, -k_twopi_hi) + k * TWO_PI_LO;

    let red_hi = theta_hi - k_twopi_hi;
    let red_lo = theta_lo - k_twopi_lo + ((theta_hi - red_hi) - k_twopi_hi);

    let theta_red = red_hi + red_lo;

    // Step 2: cos(theta)
    let c = theta_red.cos();

    // Step 3: a = log10(2/|lambda|) + Re(lambda)*n/ln(10) + log10(|c|)
    let a = (2.0_f64 / ABS_LAMBDA).log10()
        + (RE_LAMBDA * n) / std::f64::consts::LN_10
        + c.abs().log10();

    // Step 4: L = floor(-a) = number of leading 6's
    let big_l = (-a).floor() as i64;

    // Step 5: delta_mag = 10^(a + L)
    let delta_mag = 10.0_f64.powf(a + big_l as f64);

    // Step 6: delta has sign of c
    let delta = if c < 0.0 { -delta_mag } else { delta_mag };

    // s = 2/3 + delta
    let s = 2.0_f64 / 3.0 + delta;

    // Step 7: Extract first 8 non-6 digits from fractional part of s
    let mut frac = s - s.floor();
    let mut result = String::new();

    for _ in 0..200 {
        frac *= 10.0;
        let d = frac.floor() as i32;
        let d = d.clamp(0, 9); // safety clamp
        if d != 6 {
            result.push(char::from(b'0' + d as u8));
            if result.len() == 8 {
                break;
            }
        }
        frac -= d as f64;
    }

    // Verify we have enough digits (should always succeed for n=10^6)
    assert!(
        result.len() == 8,
        "Failed to collect 8 non-6 digits, got {}",
        result.len()
    );

    println!("{result}");
}
