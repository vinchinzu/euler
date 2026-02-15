// Project Euler 863 - Dice Emulation
// Value iteration for R(n) = min expected rolls using 5-sided and 6-sided dice.
// S(1000) = sum R(k) for k=2..1000.

fn main() {
    let mut total_s = 0.0f64;

    for k in 2..=1000usize {
        let mut v = vec![0.0f64; k];
        let max_iter = 200_000;
        let tol = 1e-11;

        for _ in 0..max_iter {
            let mut max_diff = 0.0f64;

            for r in 1..k {
                // Try D5
                let r5 = r * 5;
                let rem5 = r5 % k;
                let term5 = if rem5 != 0 { (rem5 as f64 / r5 as f64) * v[rem5] } else { 0.0 };
                let val5 = 1.0 + term5;

                // Try D6
                let r6 = r * 6;
                let rem6 = r6 % k;
                let term6 = if rem6 != 0 { (rem6 as f64 / r6 as f64) * v[rem6] } else { 0.0 };
                let val6 = 1.0 + term6;

                let best = val5.min(val6);
                let diff = (best - v[r]).abs();
                if diff > max_diff { max_diff = diff; }
                v[r] = best;
            }

            if max_diff < tol { break; }
        }

        total_s += v[1];
    }

    println!("{:.6}", total_s);
}
