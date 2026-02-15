// Project Euler 762 - Amoeba Division
// BFS DP with Lagrange interpolation for N=100000, K=4.

use std::collections::HashMap;

const K: usize = 4;
const MOD_VAL: i64 = 1_000_000_000;
const N_TARGET: i64 = 100_000;
const MAX_N_COMPUTE: usize = 200;

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
                        *e = (*e + count) % MOD_VAL;
                    }
                }
            }
        }
    }

    let mut result = 0i64;
    for (&key, &val) in &maps[n] {
        let cc = decode(key);
        if cc.iter().all(|&v| v <= 1) {
            result = (result + val) % MOD_VAL;
        }
    }
    result
}

fn main() {
    let mut vals = vec![0i64; MAX_N_COMPUTE];
    for n in 0..MAX_N_COMPUTE {
        vals[n] = compute_c(n);
    }

    // Find polynomial degree via finite differences
    let mut diff = vals.clone();
    let mut degree = 0usize;

    for d in 1..MAX_N_COMPUTE - 1 {
        for i in 0..MAX_N_COMPUTE - d {
            diff[i] = ((diff[i + 1] - diff[i]) % MOD_VAL + MOD_VAL) % MOD_VAL;
        }
        let all_same = (1..MAX_N_COMPUTE - d - 5).all(|i| diff[i] == diff[0]);
        if all_same {
            degree = d;
            break;
        }
    }

    if degree > 0 && degree < MAX_N_COMPUTE - 10 {
        // Recompute forward differences
        let mut fd: Vec<Vec<i64>> = Vec::new();
        fd.push(vals[..=degree].to_vec());
        for d in 1..=degree {
            let prev = &fd[d - 1];
            let mut row = Vec::new();
            for i in 0..prev.len() - 1 {
                row.push(((prev[i + 1] - prev[i]) % MOD_VAL + MOD_VAL) % MOD_VAL);
            }
            fd.push(row);
        }

        // Newton interpolation
        let mut ans = 0i64;
        for k in 0..=degree {
            // Compute binom(N_TARGET, k) using i128
            let mut num: i128 = 1;
            let mut den: i128 = 1;
            for j in 0..k as i64 {
                num *= (N_TARGET - j) as i128;
                den *= (j + 1) as i128;
            }
            let bk = ((num / den) % MOD_VAL as i128 + MOD_VAL as i128) as i64 % MOD_VAL;
            ans = (ans as i128 + fd[k][0] as i128 * bk as i128 % MOD_VAL as i128) as i64 % MOD_VAL;
        }

        println!("{}", ans);
    } else {
        eprintln!("ERROR: could not determine polynomial degree (got {})", degree);
    }
}
