// Project Euler Problem 951
// Card game with 2n cards (n red, n black).
// F(n) = C(2n,n) - (configs with no run of length 2).
// Find F(26).
//
// DP state: (r, b, length, color) where r,b in [0..26], length in [1..52], color in {0,1}

fn main() {
    let n = 26usize;
    let max_rb = n + 1;
    let max_len = 2 * n + 1;

    // dp[r][b][len][color], flat indexing
    let state_size = max_rb * max_rb * max_len * 2;
    let idx = |r: usize, b: usize, len: usize, color: usize| -> usize {
        r * max_rb * max_len * 2 + b * max_len * 2 + len * 2 + color
    };

    let mut dp = vec![0i64; state_size];

    // Initial: "R" (color 0=Red) or "B" (color 1=Black)
    dp[idx(1, 0, 1, 0)] = 1; // one red card
    dp[idx(0, 1, 1, 1)] = 1; // one black card

    for k in 1..(2 * n) {
        let mut new_dp = vec![0i64; state_size];
        for r in 0..=n {
            for b in 0..=n {
                if r + b != k {
                    continue;
                }
                for len in 1..=k {
                    for color in 0..=1 {
                        let count = dp[idx(r, b, len, color)];
                        if count == 0 {
                            continue;
                        }

                        // Try adding Red (0)
                        if r + 1 <= n {
                            if color == 0 {
                                // Extend red run
                                new_dp[idx(r + 1, b, len + 1, 0)] += count;
                            } else {
                                // Switch from black to red, check finished black run != 2
                                if len != 2 {
                                    new_dp[idx(r + 1, b, 1, 0)] += count;
                                }
                            }
                        }

                        // Try adding Black (1)
                        if b + 1 <= n {
                            if color == 1 {
                                // Extend black run
                                new_dp[idx(r, b + 1, len + 1, 1)] += count;
                            } else {
                                // Switch from red to black, check finished red run != 2
                                if len != 2 {
                                    new_dp[idx(r, b + 1, 1, 1)] += count;
                                }
                            }
                        }
                    }
                }
            }
        }
        dp = new_dp;
    }

    // Sum valid final states: r=n, b=n, final run length != 2
    let mut bad_count: i64 = 0;
    for len in 1..=(2 * n) {
        if len == 2 {
            continue;
        }
        for color in 0..=1 {
            bad_count += dp[idx(n, n, len, color)];
        }
    }

    // Compute C(2n, n) using u128 to avoid overflow
    let mut total: u128 = 1;
    for i in 1..=n as u128 {
        total = total * (n as u128 + i) / i;
    }

    let fair = total as i64 - bad_count;
    println!("{}", fair);
}
