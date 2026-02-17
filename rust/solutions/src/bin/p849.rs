// Project Euler Problem 849: The Tournament
// Count distinct final point-multisets for double round-robin

const MOD: i64 = 1_000_000_007;

fn count_outcomes(n: usize) -> i64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let vmax = 2 * (n - 1);
    let off = vmax;
    let v = 2 * vmax + 1;

    // maxh[i] = 2 * i * (n - i)
    let maxh: Vec<usize> = (0..=n).map(|i| 2 * i * (n - i)).collect();

    // dp[h][vi] where vi encodes d = vi-off, after i steps.
    // Start at i=1: p_1 = d_1 (since p_0=0), and d_1 >= 0.
    let mut dp: Vec<Vec<i64>> = vec![vec![0; v]; vmax + 1];
    for d in 0..=vmax {
        dp[d][d + off] = 1;
    }

    for step in 1..n {
        let mh = maxh[step];
        let mh_next = maxh[step + 1];
        let mut nxt: Vec<Vec<i64>> = vec![vec![0; v]; mh_next + 1];

        for h in 0..=mh {
            let row = &dp[h];
            let mut cum: i64 = 0;

            let base = h as i64 - off as i64 - 4;
            let mut vi_start = -base;
            if vi_start < 4 {
                vi_start = 4;
            }
            if vi_start > v as i64 {
                vi_start = v as i64;
            }
            let mut vi_end = mh_next as i64 - base;
            if vi_end > (v - 1) as i64 {
                vi_end = (v - 1) as i64;
            }

            // cum for vi < vi_start
            for vi in 0..(vi_start as usize) {
                cum += row[vi];
                if cum >= MOD {
                    cum -= MOD;
                }
            }

            // cum + updates for vi in [vi_start, vi_end]
            if vi_start <= vi_end {
                for vi in (vi_start as usize)..=(vi_end as usize) {
                    cum += row[vi];
                    if cum >= MOD {
                        cum -= MOD;
                    }
                    let h2 = (base + vi as i64) as usize;
                    let idx = vi - 4;
                    let val = nxt[h2][idx] + cum;
                    nxt[h2][idx] = if val >= MOD { val - MOD } else { val };
                }

                // finish cum for vi > vi_end
                for vi in (vi_end as usize + 1)..v {
                    cum += row[vi];
                    if cum >= MOD {
                        cum -= MOD;
                    }
                }
            } else {
                // no update region
                for vi in (vi_start as usize)..v {
                    cum += row[vi];
                    if cum >= MOD {
                        cum -= MOD;
                    }
                }
            }

            let total = cum;

            // For the largest 4 velocities, the threshold d'+4 saturates at vmax,
            // so they all receive the full sum 'total'.
            if total > 0 {
                let base_h = h as i64 - off as i64;
                for vi in (v - 4)..v {
                    let h2 = (base_h + vi as i64) as usize;
                    if h2 <= mh_next {
                        let val = nxt[h2][vi] + total;
                        nxt[h2][vi] = if val >= MOD { val - MOD } else { val };
                    }
                }
            }
        }

        dp = nxt;
    }

    // At step n, height must be 0; sum over all last differences.
    dp[0].iter().sum::<i64>() % MOD
}

fn main() {
    // Test: F(2) = 3
    assert_eq!(count_outcomes(2), 3);

    // Test: F(7) = 32923
    assert_eq!(count_outcomes(7), 32923);

    println!("{}", count_outcomes(100));
}
