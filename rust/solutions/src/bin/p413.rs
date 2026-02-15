use std::collections::HashMap;

use euler_utils::gcd;

const BASE: usize = 10;
const MAXD: usize = 20;

fn main() {
    let n_max = 19;
    let mut total_ans: i64 = 0;

    for d in 1..=n_max {
        let g = gcd(d as u64, BASE as u64) as usize;
        let cap = if g == 1 { 1 } else { 2 };

        let mut dp: HashMap<u64, i64> = HashMap::new();

        // Encode state: counts[r] for r in 0..d (capped at cap), plus hits flag
        let encode = |counts: &[usize], hits: usize| -> u64 {
            let mut key = 0u64;
            for r in 0..d {
                let c = counts[r].min(2);
                key |= (c as u64) << (2 * r);
            }
            key |= (hits as u64) << 62;
            key
        };

        let decode = |key: u64| -> (Vec<usize>, usize) {
            let mut counts = vec![0usize; d];
            for r in 0..d {
                counts[r] = ((key >> (2 * r)) & 3) as usize;
            }
            let hits = ((key >> 62) & 1) as usize;
            (counts, hits)
        };

        let init_counts = vec![0usize; d];
        let init_key = encode(&init_counts, 0);
        dp.insert(init_key, 1);

        for pos in 0..d {
            let mut dp2: HashMap<u64, i64> = HashMap::new();

            for (&key, &val) in &dp {
                if val == 0 { continue; }
                let (old_counts, old_hits) = decode(key);

                let start_digit = if pos == 0 { 1 } else { 0 };
                for digit in start_digit..BASE {
                    let mut new_counts = vec![0usize; d];
                    for r in 0..d {
                        if old_counts[r] > 0 {
                            let new_r = (r * 10 + digit) % d;
                            new_counts[new_r] += old_counts[r];
                        }
                    }
                    new_counts[digit % d] += 1;

                    let mut new_hits = old_hits;
                    new_hits += new_counts[0];

                    if new_hits > 1 { continue; }

                    for r in 0..d {
                        if new_counts[r] > cap { new_counts[r] = cap; }
                    }

                    let new_key = encode(&new_counts, new_hits);
                    *dp2.entry(new_key).or_insert(0) += val;
                }
            }

            dp = dp2;
        }

        let mut d_ans: i64 = 0;
        for (&key, &val) in &dp {
            let hits = ((key >> 62) & 1) as usize;
            if hits == 1 {
                d_ans += val;
            }
        }
        total_ans += d_ans;
    }

    println!("{}", total_ans);
}
