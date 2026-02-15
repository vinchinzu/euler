// Project Euler 852 - Coins in a Box (DP with backward induction)

const REWARD: f64 = 20.0;
const PENALTY: f64 = -50.0;
const COST_PER_TOSS: f64 = 1.0;
const MAX_TOSS: usize = 300;

fn compute_posterior(fair: usize, unfair: usize, heads: usize, tails: usize) -> f64 {
    let total_coins = fair + unfair;
    if total_coins == 0 { return 0.0; }
    let total_tosses = heads + tails;
    if total_tosses == 0 { return fair as f64 / total_coins as f64; }

    let prior_fair = fair as f64 / total_coins as f64;
    let log_like_fair = total_tosses as f64 * 0.5f64.ln();
    let log_like_unfair = heads as f64 * 0.75f64.ln() + tails as f64 * 0.25f64.ln();

    let log_num = log_like_fair + prior_fair.ln();
    let log_den_term2 = log_like_unfair + (1.0 - prior_fair).ln();

    let max_log = log_num.max(log_den_term2);
    let log_den = max_log + ((log_num - max_log).exp() + (log_den_term2 - max_log).exp()).ln();

    (log_num - log_den).exp()
}

fn main() {
    let n = 50usize;
    let mut global_memo = vec![vec![0.0f64; n + 2]; n + 2];
    let mut global_memo_set = vec![vec![false; n + 2]; n + 2];
    global_memo[0][0] = 0.0;
    global_memo_set[0][0] = true;

    for total_coins in 1..=(2 * n) {
        let start_fair = if total_coins > n { total_coins - n } else { 0 };
        let end_fair = total_coins.min(n);

        for fair in start_fair..=end_fair {
            let unfair = total_coins - fair;

            let future_fair = if fair > 0 && global_memo_set[fair - 1][unfair] {
                global_memo[fair - 1][unfair]
            } else { 0.0 };
            let future_unfair = if unfair > 0 && global_memo_set[fair][unfair - 1] {
                global_memo[fair][unfair - 1]
            } else { 0.0 };

            let mut prev = vec![0.0f64; MAX_TOSS + 2];
            let mut curr = vec![0.0f64; MAX_TOSS + 2];

            // Initialize for total_tosses = MAX_TOSS
            for h in 0..=MAX_TOSS {
                let t = MAX_TOSS - h;
                let pf = compute_posterior(fair, unfair, h, t);
                let ev_fair = pf * (REWARD + future_fair) + (1.0 - pf) * (PENALTY + future_unfair);
                let ev_unfair = (1.0 - pf) * (REWARD + future_unfair) + pf * (PENALTY + future_fair);
                prev[h] = ev_fair.max(ev_unfair);
            }

            for total in (0..MAX_TOSS).rev() {
                for h in 0..=total {
                    let t = total - h;
                    let pf = compute_posterior(fair, unfair, h, t);
                    let ev_fair = pf * (REWARD + future_fair) + (1.0 - pf) * (PENALTY + future_unfair);
                    let ev_unfair = (1.0 - pf) * (REWARD + future_unfair) + pf * (PENALTY + future_fair);
                    let best_guess = ev_fair.max(ev_unfair);

                    let p_heads = pf * 0.5 + (1.0 - pf) * 0.75;
                    let ev_toss = p_heads * prev[h + 1] + (1.0 - p_heads) * prev[h] - COST_PER_TOSS;

                    curr[h] = best_guess.max(ev_toss);
                }
                std::mem::swap(&mut prev, &mut curr);
            }

            global_memo[fair][unfair] = prev[0];
            global_memo_set[fair][unfair] = true;
        }
    }

    println!("{:.6}", global_memo[n][n]);
}
