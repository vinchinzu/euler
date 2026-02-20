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
    let max_h = *maxh.iter().max().unwrap();

    // Flat 1D DP tables, pre-allocated to max height
    let mut dp = vec![0i64; (max_h + 1) * v];
    let mut nxt = vec![0i64; (max_h + 1) * v];

    for d in 0..=vmax {
        dp[d * v + d + off] = 1;
    }

    for step in 1..n {
        let mh = maxh[step];
        let mh_next = maxh[step + 1];
        nxt[..(mh_next + 1) * v].fill(0);

        for h in 0..=mh {
            let row_start = h * v;
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
                cum += dp[row_start + vi];
                if cum >= MOD {
                    cum -= MOD;
                }
            }

            // cum + updates for vi in [vi_start, vi_end]
            if vi_start <= vi_end {
                for vi in (vi_start as usize)..=(vi_end as usize) {
                    cum += dp[row_start + vi];
                    if cum >= MOD {
                        cum -= MOD;
                    }
                    let h2 = (base + vi as i64) as usize;
                    let nxt_idx = h2 * v + vi - 4;
                    let val = nxt[nxt_idx] + cum;
                    nxt[nxt_idx] = if val >= MOD { val - MOD } else { val };
                }

                // finish cum for vi > vi_end
                for vi in (vi_end as usize + 1)..v {
                    cum += dp[row_start + vi];
                    if cum >= MOD {
                        cum -= MOD;
                    }
                }
            } else {
                // no update region
                for vi in (vi_start as usize)..v {
                    cum += dp[row_start + vi];
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
                        let nxt_idx = h2 * v + vi;
                        let val = nxt[nxt_idx] + total;
                        nxt[nxt_idx] = if val >= MOD { val - MOD } else { val };
                    }
                }
            }
        }

        std::mem::swap(&mut dp, &mut nxt);
    }

    // At step n, height must be 0; sum over all last differences.
    dp[..v].iter().sum::<i64>() % MOD
}

fn main() {
    // Test: F(2) = 3
    assert_eq!(count_outcomes(2), 3);

    // Test: F(7) = 32923
    assert_eq!(count_outcomes(7), 32923);

    println!("{}", count_outcomes(100));
}
