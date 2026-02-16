// Project Euler 938 - Card Game Probability
// DP: P(r,b) = (prob_rr * P(r-2,b) + prob_mixed * P(r,b-1)) / (1 - prob_bb)
// Using O(B) space with 3 rows, rotating buffers.

fn main() {
    const R: usize = 24690;
    const B: usize = 12345;

    let mut dp_prev2 = vec![0.0f64; B + 1]; // P(r-2, b), initially P(0, b)
    let mut dp_prev = vec![0.0f64; B + 1];  // P(r-1, b), initially P(1, b)
    let mut dp_curr = vec![0.0f64; B + 1];

    // Base: P(0, b) = 1 for b >= 1, P(0,0) = 0
    for i in 1..=B {
        dp_prev2[i] = 1.0;
    }

    // P(1, b): prob_rr=0, just mixed
    for b in 1..=B {
        let total = 1.0 + b as f64;
        let prob_bb = (b as f64 * (b as f64 - 1.0)) / (total * (total - 1.0));
        let prob_mixed = (2.0 * 1.0 * b as f64) / (total * (total - 1.0));
        let p_mixed = dp_prev2[b - 1];
        let temp = prob_mixed * p_mixed;
        dp_prev[b] = temp / (1.0 - prob_bb);
    }

    // Compute for r from 2 to R
    for r in 2..=R {
        for b in 0..=B {
            dp_curr[b] = 0.0;
        }
        for b in 1..=B {
            let rf = r as f64;
            let bf = b as f64;
            let total = rf + bf;
            let prob_rr = (rf * (rf - 1.0)) / (total * (total - 1.0));
            let prob_bb = (bf * (bf - 1.0)) / (total * (total - 1.0));
            let prob_mixed = (2.0 * rf * bf) / (total * (total - 1.0));

            let p_rr = dp_prev2[b];
            let p_mixed = dp_curr[b - 1];
            let temp = prob_rr * p_rr + prob_mixed * p_mixed;
            dp_curr[b] = temp / (1.0 - prob_bb);
        }
        // Rotate arrays
        std::mem::swap(&mut dp_prev2, &mut dp_prev);
        std::mem::swap(&mut dp_prev, &mut dp_curr);
    }

    println!("{:.10}", dp_prev[B]);
}
