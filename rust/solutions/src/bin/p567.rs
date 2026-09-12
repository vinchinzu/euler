// Project Euler 567 - Reciprocal Games I
//
// Sum_{n=1}^N (J_A(n) + J_B(n)) = 4*H_{N-1} - ln(4)
// where H_n = harmonic number and N = 123456789.

fn main() {
    // H_n via Euler–Maclaurin: ln(n)+γ+1/(2n)-1/(12n²)+1/(120n⁴)-…
    let n: f64 = 123_456_788.0;
    const GAMMA: f64 = 0.5772156649015328606;
    let n2 = n * n;
    let n4 = n2 * n2;
    let h = n.ln() + GAMMA + 0.5 / n - 1.0 / (12.0 * n2) + 1.0 / (120.0 * n4);
    let ans = 4.0 * h - 4.0_f64.ln();
    println!("{:.8}", ans);
}
