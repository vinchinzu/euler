// Project Euler 762 - Amoebas in a 2D Grid
// C(n) = number of distinct arrangements after n divisions on a 4-row grid.
// C(n) satisfies a linear recurrence of order 8 for n >= 9:
//   C(n) = 2*C(n-1) + 2*C(n-2) - C(n-3) - 3*C(n-4) - 5*C(n-5) + 4*C(n-6) - 2*C(n-7) + 4*C(n-8)
// We compute initial values C(0)..C(17) via BFS DP, then iterate the recurrence.

use std::collections::HashMap;

const K: usize = 4;
const MOD: i64 = 1_000_000_000;

fn encode(c: &[i32; K]) -> u32 {
    c[0] as u32 | ((c[1] as u32) << 4) | ((c[2] as u32) << 8) | ((c[3] as u32) << 12)
}

fn decode(key: u32) -> [i32; K] {
    [
        (key & 0xF) as i32,
        ((key >> 4) & 0xF) as i32,
        ((key >> 8) & 0xF) as i32,
        ((key >> 12) & 0xF) as i32,
    ]
}

fn compute_c(n: usize) -> i64 {
    let mut maps: Vec<HashMap<u32, i64>> = vec![HashMap::new(); n + 1];

    let start = [1i32, 0, 0, 0];
    *maps[0].entry(encode(&start)).or_insert(0) += 1;

    for division in 0..n {
        let entries: Vec<(u32, i64)> = maps[division].iter().map(|(&k, &v)| (k, v)).collect();
        for (key, count) in entries {
            if count == 0 { continue; }
            let cc = decode(key);

            let mut n_choices = [0usize; K];
            let mut choices = [[0i32; 2]; K];
            let mut total_combos = 1usize;
            for i in 0..K {
                if cc[i] == 0 {
                    n_choices[i] = 1;
                    choices[i][0] = 0;
                } else {
                    n_choices[i] = 2;
                    choices[i][0] = cc[i] - 1;
                    choices[i][1] = cc[i];
                }
                total_combos *= n_choices[i];
            }

            for combo in 0..total_combos {
                let mut cd = [0i32; K];
                let mut tmp = combo;
                let mut total_div = 0;
                for i in 0..K {
                    cd[i] = choices[i][tmp % n_choices[i]];
                    tmp /= n_choices[i];
                    total_div += cd[i];
                }

                if total_div > 0 && total_div < K as i32 {
                    let next_step = division + total_div as usize;
                    if next_step <= n {
                        let mut new_pos = [0i32; K];
                        for i in 0..K {
                            new_pos[i] = cd[i] + cd[(i + 1) % K];
                        }
                        let e = maps[next_step].entry(encode(&new_pos)).or_insert(0);
                        *e = (*e + count) % MOD;
                    }
                }
            }
        }
    }

    let mut result = 0i64;
    for (&key, &val) in &maps[n] {
        let cc = decode(key);
        if cc.iter().all(|&v| v <= 1) {
            result = (result + val) % MOD;
        }
    }
    result
}

fn main() {
    let n_target: usize = 100_000;

    // Recurrence coefficients: C(n) = sum coeffs[j] * C(n-j-1) for j=0..7
    // Valid for n >= 9
    let coeffs: [i64; 8] = [2, 2, -1, -3, -5, 4, -2, 4];
    let order = 8;

    // We need initial values C(0)..C(order+order-1) to have enough to start
    // computing the recurrence from n=9 onward.
    // Actually we just need C(0)..C(max(8, order)) = C(0)..C(8).
    // But let's compute a few more for safety and verification.
    let n_init = 18; // compute C(0)..C(17) via BFS DP

    let mut vals = vec![0i64; n_target + 1];
    for n in 0..n_init {
        vals[n] = compute_c(n);
    }

    // Extend using recurrence from n=9 onward (initial values already computed up to 17)
    for n in n_init..=n_target {
        let mut v: i64 = 0;
        for j in 0..order {
            v = ((v + coeffs[j] * vals[n - j - 1]) % MOD + MOD) % MOD;
        }
        vals[n] = v;
    }

    println!("{}", vals[n_target]);
}
