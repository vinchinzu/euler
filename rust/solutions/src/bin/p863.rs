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
                let r5 = r * 5;
                let rem5 = r5 % k;
                
                let r6 = r * 6;
                let rem6 = r6 % k;

                // SAFETY: rem5 < k and rem6 < k by definition of modulo
                // v has size k, so indices are in bounds
                // Note: v[0] = 0.0, so no need to check rem5 == 0 or rem6 == 0
                let v_rem5 = unsafe { *v.get_unchecked(rem5) };
                let v_rem6 = unsafe { *v.get_unchecked(rem6) };
                
                let val5 = 1.0 + (rem5 as f64 * v_rem5) / (r5 as f64);
                let val6 = 1.0 + (rem6 as f64 * v_rem6) / (r6 as f64);

                let best = val5.min(val6);
                
                // SAFETY: r < k by loop bounds, v has size k
                let v_r = unsafe { v.get_unchecked_mut(r) };
                let diff = (best - *v_r).abs();
                if diff > max_diff {
                    max_diff = diff;
                }
                *v_r = best;
            }

            if max_diff < tol {
                break;
            }
        }

        total_s += v[1];
    }

    println!("{:.6}", total_s);
}
