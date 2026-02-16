// Project Euler 380 - Amazing Mazes!
//
// Count spanning trees in 100x500 grid graph.
// The number of spanning trees of P_m x P_n is:
//   tau = (1/(m*n)) * product_{(i,j)!=(0,0)} (lambda_i + mu_j)
// where lambda_i = 2 - 2*cos(pi*i/m) for i=0..m-1
// and   mu_j = 2 - 2*cos(pi*j/n) for j=0..n-1
//
// We compute log10(tau) and format in scientific notation with 5 sig digits.

fn main() {
    let m: usize = 100;
    let n: usize = 500;
    let pi = std::f64::consts::PI;

    // Precompute eigenvalues of P_m and P_n Laplacians
    let lambda_m: Vec<f64> = (0..m)
        .map(|i| 2.0 - 2.0 * (pi * i as f64 / m as f64).cos())
        .collect();
    let mu_n: Vec<f64> = (0..n)
        .map(|j| 2.0 - 2.0 * (pi * j as f64 / n as f64).cos())
        .collect();

    // Compute sum of log10 of nonzero eigenvalues
    let mut log_sum: f64 = 0.0;
    for i in 0..m {
        for j in 0..n {
            if i == 0 && j == 0 {
                continue;
            }
            let eigenvalue = lambda_m[i] + mu_n[j];
            log_sum += eigenvalue.log10();
        }
    }

    // Subtract log10(m*n)
    let log_tau = log_sum - ((m * n) as f64).log10();

    // Format as scientific notation with 5 significant digits
    // log_tau = integer_part + fractional_part
    // tau = 10^fractional_part * 10^integer_part
    let exponent = log_tau.floor() as i64;
    let mantissa = 10.0_f64.powf(log_tau - exponent as f64);

    // Round mantissa to 5 significant digits: x.xxxx
    // mantissa is in [1, 10)
    let mantissa_rounded = (mantissa * 10000.0).round() / 10000.0;

    println!("{:.4}e{}", mantissa_rounded, exponent);
}
