// Project Euler 567 - Reciprocal Games I
//
// Sum_{n=1}^N (J_A(n) + J_B(n)) = 4*H_{N-1} - ln(4)
// where H_n = harmonic number and N = 123456789.

fn main() {
    let n: i64 = 123_456_789;

    // Compute harmonic number H_{N-1} using Kahan summation for precision
    let mut sum: f64 = 0.0;
    let mut c: f64 = 0.0;
    for i in 1..n {
        let y = 1.0 / i as f64 - c;
        let t = sum + y;
        c = (t - sum) - y;
        sum = t;
    }

    let ans = 4.0 * sum - (4.0_f64).ln();
    println!("{:.8}", ans);
}
