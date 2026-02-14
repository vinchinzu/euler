// Project Euler 205: Dice Game
// Peter rolls 9 four-sided dice, Colin rolls 6 six-sided dice.
// Find the probability that Peter beats Colin.

fn main() {
    let mut peter = [0i64; 37];
    let mut colin = [0i64; 37];

    // Peter: 9d4
    {
        let mut dp = [0i64; 37];
        dp[0] = 1;
        for _ in 0..9 {
            let mut new_dp = [0i64; 37];
            for s in 0..=36 {
                if dp[s] == 0 { continue; }
                for face in 1..=4 {
                    if s + face <= 36 {
                        new_dp[s + face] += dp[s];
                    }
                }
            }
            dp = new_dp;
        }
        peter = dp;
    }

    // Colin: 6d6
    {
        let mut dp = [0i64; 37];
        dp[0] = 1;
        for _ in 0..6 {
            let mut new_dp = [0i64; 37];
            for s in 0..=36 {
                if dp[s] == 0 { continue; }
                for face in 1..=6 {
                    if s + face <= 36 {
                        new_dp[s + face] += dp[s];
                    }
                }
            }
            dp = new_dp;
        }
        colin = dp;
    }

    let peter_total: f64 = 4.0_f64.powi(9);
    let colin_total: f64 = 6.0_f64.powi(6);
    let total = peter_total * colin_total;

    let mut wins: i64 = 0;
    for p in 1..=36 {
        for c in 1..p {
            wins += peter[p] * colin[c];
        }
    }

    println!("{:.7}", wins as f64 / total);
}
