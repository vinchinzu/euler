const N: usize = 100;

fn main() {
    let mut dp = [[0.0f64; 2 * N]; N + 1];

    // Base cases: player 2 wins if score2 >= N
    for s1 in 0..=N {
        for s2 in N..(2 * N) {
            dp[s1][s2] = 1.0;
        }
    }

    for s1 in (0..N).rev() {
        for s2 in (0..N).rev() {
            let mut best = 0.0f64;
            for t in 1..100 {
                let points = 1usize << (t - 1);
                if s2 + points >= 2 * N { break; }
                let pow2t = 1u64 << t;
                let prob = (dp[s1 + 1][s2 + points]
                    + dp[s1][s2 + points]
                    + (pow2t - 1) as f64 * dp[s1 + 1][s2])
                    / (pow2t + 1) as f64;
                if prob > best { best = prob; }
            }
            dp[s1][s2] = best;
        }
    }

    let ans = (dp[0][0] + dp[1][0]) / 2.0;
    println!("{:.8}", ans);
}
