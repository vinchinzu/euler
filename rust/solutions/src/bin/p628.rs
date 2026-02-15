// Project Euler 628 - Open chess positions
// Single-pass factorial computation for formula involving sums of k!

const N_VAL: i64 = 100_000_000;
const M_VAL: i64 = 1_008_691_207;

fn main() {
    let mut fact_k = 1i64;
    let mut sum1 = 0i64;
    let mut sum2 = 0i64;
    let mut fact_n = 1i64;

    for k in 0..=N_VAL {
        if k > 0 { fact_k = (fact_k as i128 * k as i128 % M_VAL as i128) as i64; }
        if k >= 1 && k <= N_VAL - 1 {
            sum1 = (sum1 + fact_k) % M_VAL;
        }
        if k <= N_VAL - 2 {
            let coeff = (N_VAL - 1 - k) % M_VAL;
            sum2 = (sum2 + (coeff as i128 * fact_k as i128 % M_VAL as i128) as i64) % M_VAL;
        }
        if k == N_VAL { fact_n = fact_k; }
    }

    let ans = ((fact_n - 1 - 2 * sum1 + sum2) % M_VAL + M_VAL) % M_VAL;
    println!("{}", ans);
}
