// Problem 906: A Collective Decision
//
// P(n) = probability that 3 random permutations of {1,...,n} have a Condorcet winner.
// P(3) = 17/18, P(10) ~ 0.6760292265, find P(20000) to 10 decimal places.
//
// Formula: P(n) = (1/n^2) * sum_{b1+b2+b3 <= m} (m-b1)!(m-b2)!(m-b3)! / ((m-B)! m!^2)
// where m = n-1, B = b1+b2+b3.
//
// After summing over b3 using hockey stick identity:
// S = ((m+1)/m!) * sum_{u=0}^{m} I(u) / ((u+1)*(m-u)!)
// where I(u) = sum_{k=0}^{u} (m-k)! * (m-u+k)!
//
// Computed in log-space to avoid overflow. Parallelized with rayon.

use rayon::prelude::*;

fn solve(n: usize) -> f64 {
    let m = n - 1;

    // Precompute log_fact[i] = log(i!) for i = 0..=m+1
    let mut log_fact = vec![0.0f64; m + 2];
    for i in 1..=m + 1 {
        log_fact[i] = log_fact[i - 1] + (i as f64).ln();
    }

    // log_coeff = log((m+1)/m!) = log(m+1) - log(m!)
    let log_coeff = ((m + 1) as f64).ln() - log_fact[m];

    let total: f64 = (0..=m).into_par_iter().map(|u| {
        // Compute I(u) = sum_{k=0}^u exp(log_fact[m-k] + log_fact[m-u+k])
        // Find actual max
        let mut max_log = f64::NEG_INFINITY;
        for k in 0..=u {
            let log_val = log_fact[m - k] + log_fact[m - u + k];
            if log_val > max_log {
                max_log = log_val;
            }
        }

        let mut sum_exp = 0.0f64;
        for k in 0..=u {
            let log_val = log_fact[m - k] + log_fact[m - u + k];
            let diff = log_val - max_log;
            if diff > -50.0 {
                sum_exp += diff.exp();
            }
        }

        let log_i_u = max_log + sum_exp.ln();
        let log_contrib = log_coeff + log_i_u - ((u + 1) as f64).ln() - log_fact[m - u];
        log_contrib.exp()
    }).sum();

    total / (n as f64 * n as f64)
}

fn main() {
    println!("{:.10}", solve(20000));
}
