// Project Euler 628 - Open chess positions
// Single-pass factorial computation for formula involving sums of k!

const N_VAL: u64 = 100_000_000;
const M_VAL: u64 = 1_008_691_207;

fn main() {
    let mut fact_k = 1u64;
    let mut sum1 = 0u64;
    let mut sum2 = 0u64;
    let mut fact_n = 1u64;

    for k in 0..=N_VAL {
        if k > 0 {
            fact_k = fact_k * k % M_VAL;
        }
        if k >= 1 && k <= N_VAL - 1 {
            sum1 += fact_k;
            if sum1 >= M_VAL {
                sum1 -= M_VAL;
            }
        }
        if k <= N_VAL - 2 {
            let coeff = (N_VAL - 1 - k) % M_VAL;
            sum2 += coeff * fact_k % M_VAL;
            if sum2 >= M_VAL {
                sum2 -= M_VAL;
            }
        }
        if k == N_VAL {
            fact_n = fact_k;
        }
    }

    let ans = (fact_n + M_VAL * 2 - 1 - 2 * sum1 % M_VAL + sum2) % M_VAL;
    println!("{}", ans);
}
