// Project Euler 568 - Reciprocal Games II
//
// D(n) = H_n / 2^n. We need 7 most significant digits of D(123456789).
// log10(D(n)) = log10(H_n) - n*log10(2).
// H_n ~ ln(n) + gamma + 1/(2n) - 1/(12n^2) + ... (Euler-Maclaurin)

fn main() {
    let n: i64 = 123_456_789;
    let nf = n as f64;

    // Euler-Mascheroni constant
    let gamma: f64 = 0.5772156649015328606065120900824024310421;

    // H_n via Euler-Maclaurin
    let inv_n = 1.0 / nf;
    let inv_n2 = inv_n * inv_n;
    let h_n = nf.ln() + gamma
        + inv_n / 2.0
        - inv_n2 / 12.0
        + inv_n2 * inv_n2 / 120.0
        - inv_n2 * inv_n2 * inv_n2 / 252.0
        + inv_n2 * inv_n2 * inv_n2 * inv_n2 / 240.0;

    // log10(D(N)) = log10(H_N) - N * log10(2)
    let log10_h = h_n.log10();
    let log10_d = log10_h - nf * 2.0_f64.log10();

    // Extract fractional part
    let frac = log10_d - log10_d.floor();
    let significant = 10.0_f64.powf(frac);

    // Get 7 digits
    let digits = (significant * 1_000_000.0) as i64;
    println!("{}", digits);
}
