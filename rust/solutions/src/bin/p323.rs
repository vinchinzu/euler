const BITS: usize = 32;

fn main() {
    let mut binom = [[0i64; BITS + 1]; BITS + 1];
    for n in 0..=BITS {
        binom[n][0] = 1;
        for k in 1..=n {
            binom[n][k] = binom[n - 1][k - 1] + binom[n - 1][k];
        }
    }

    let mut expected = [0.0f64; BITS + 1];
    expected[0] = 0.0;

    for k in 1..=BITS {
        let inv_two_k = 1.0 / (1u64 << k) as f64;
        let p_stay = inv_two_k;

        let mut sum_transitions = 0.0;
        for j in 1..k {
            let p_j = binom[k][j] as f64 * inv_two_k;
            sum_transitions += p_j * expected[k - j];
        }

        expected[k] = (1.0 + sum_transitions) / (1.0 - p_stay);
    }

    println!("{:.10}", expected[BITS]);
}
