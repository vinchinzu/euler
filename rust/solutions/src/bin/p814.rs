// Project Euler 814 - Mezzo-forte
// Count configurations of 4N people in a circle with exactly N mutual pairs

const MOD: i64 = 998244353;

fn main() {
    let n: usize = 1000;

    let mut ans: i64 = 0;

    for sa in 0..2usize {
        for sb in 0..2usize {
            // dp_prev[a][b][k]
            let mut dp_prev = vec![vec![vec![0i64; n + 3]; 2]; 2];
            dp_prev[sa][sb][0] = 1;

            for _step in 1..2 * n {
                let mut dp_cur = vec![vec![vec![0i64; n + 3]; 2]; 2];

                for a in 0..2usize {
                    for b in 0..2usize {
                        for da in 0..3usize {
                            for db in 0..3usize {
                                let mut delta = 0usize;
                                if da == 1 && db == 1 { delta += 1; }
                                if da == 0 && a == 1 { delta += 1; }
                                if db == 0 && b == 1 { delta += 1; }
                                let na = da / 2;
                                let nb = db / 2;

                                if delta > n { continue; }
                                let kmax = n - delta;
                                let src = &dp_prev[a][b];
                                for k in 0..=kmax {
                                    if src[k] != 0 {
                                        dp_cur[na][nb][k + delta] =
                                            (dp_cur[na][nb][k + delta] + src[k]) % MOD;
                                    }
                                }
                            }
                        }
                    }
                }

                dp_prev = dp_cur;
            }

            // Final count
            for a in 0..2usize {
                for b in 0..2usize {
                    for da in 0..3usize {
                        for db in 0..3usize {
                            if sa == da / 2 && sb == db / 2 {
                                let mut k = n as i32;
                                if da == 1 && db == 1 { k -= 1; }
                                if da == 0 && b == 1 { k -= 1; }
                                if db == 0 && a == 1 { k -= 1; }
                                if k >= 0 {
                                    ans = (ans + dp_prev[a][b][k as usize]) % MOD;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
