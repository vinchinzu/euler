/*
 * Project Euler 852 - Coins in a Box
 *
 * S(N) = expected score when playing optimally with N fair + N unfair coins.
 * S(50) rounded to 6 decimal places.
 *
 * Algorithm: DP over (fair, unfair) states. For each coin, backward induction
 * over toss outcomes (heads, tails) to find optimal strategy.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

#define REWARD 20.0
#define PENALTY (-50.0)
#define COST_PER_TOSS 1.0
#define MAX_TOSS 300

/* Global memo for (fair, unfair) -> expected future score */
static double global_memo[52][52];
static int global_memo_set[52][52];

static double compute_posterior(int fair, int unfair, int heads, int tails) {
    int total_coins = fair + unfair;
    if (total_coins == 0) return 0.0;

    int total_tosses = heads + tails;
    if (total_tosses == 0)
        return (double)fair / total_coins;

    double prior_fair = (double)fair / total_coins;

    /* Use log-likelihoods for numerical stability */
    double log_like_fair = total_tosses * log(0.5);
    double log_like_unfair = heads * log(0.75) + tails * log(0.25);

    /* Bayes with log trick */
    double log_num = log_like_fair + log(prior_fair);
    double log_den_term2 = log_like_unfair + log(1.0 - prior_fair);

    /* log(num + den_term2) */
    double max_log = log_num > log_den_term2 ? log_num : log_den_term2;
    double log_den = max_log + log(exp(log_num - max_log) + exp(log_den_term2 - max_log));

    return exp(log_num - log_den);
}

static double get_global_memo(int f, int u) {
    if (f < 0 || u < 0) return 0.0;
    if (!global_memo_set[f][u]) return 0.0;
    return global_memo[f][u];
}

static double solve_current_coin(int fair, int unfair) {
    double future_fair_remains = (fair > 0) ? get_global_memo(fair - 1, unfair) : 0.0;
    double future_unfair_remains = (unfair > 0) ? get_global_memo(fair, unfair - 1) : 0.0;

    /* local_memo[h][t] for h+t <= MAX_TOSS */
    /* To save memory, process layer by layer from total_tosses = MAX_TOSS down to 0 */
    /* For a given total_tosses T, h ranges 0..T, t = T-h */
    /* We need values for T+1 to compute T. So keep two layers. */

    /* Actually, we need local_memo(h+1, t) and local_memo(h, t+1) from the next layer.
     * With layers indexed by total_tosses, we process backwards.
     * Current layer T needs (h+1, T-h) from layer T+1 and (h, T+1-h) from layer T+1.
     * Both are from layer T+1. So we keep two 1D arrays indexed by h.
     */

    /* prev[h] = local_memo[h][T_prev - h] for the previous (larger) total_tosses */
    double *prev = (double *)malloc((MAX_TOSS + 2) * sizeof(double));
    double *curr = (double *)malloc((MAX_TOSS + 2) * sizeof(double));

    /* Initialize for total_tosses = MAX_TOSS */
    for (int h = 0; h <= MAX_TOSS; h++) {
        int t = MAX_TOSS - h;
        double prob_fair = compute_posterior(fair, unfair, h, t);

        double ev_guess_fair = prob_fair * (REWARD + future_fair_remains)
                             + (1.0 - prob_fair) * (PENALTY + future_unfair_remains);
        double ev_guess_unfair = (1.0 - prob_fair) * (REWARD + future_unfair_remains)
                               + prob_fair * (PENALTY + future_fair_remains);

        prev[h] = ev_guess_fair > ev_guess_unfair ? ev_guess_fair : ev_guess_unfair;
    }

    /* Process backwards */
    for (int T = MAX_TOSS - 1; T >= 0; T--) {
        for (int h = 0; h <= T; h++) {
            int t = T - h;
            double prob_fair = compute_posterior(fair, unfair, h, t);

            double ev_guess_fair = prob_fair * (REWARD + future_fair_remains)
                                 + (1.0 - prob_fair) * (PENALTY + future_unfair_remains);
            double ev_guess_unfair = (1.0 - prob_fair) * (REWARD + future_unfair_remains)
                                   + prob_fair * (PENALTY + future_fair_remains);

            double best_guess = ev_guess_fair > ev_guess_unfair ? ev_guess_fair : ev_guess_unfair;

            /* Toss value */
            double p_heads = prob_fair * 0.5 + (1.0 - prob_fair) * 0.75;
            double val_h = prev[h + 1]; /* local_memo(h+1, t) from layer T+1 */
            double val_t = prev[h];     /* local_memo(h, t+1) from layer T+1 */

            double ev_toss = p_heads * val_h + (1.0 - p_heads) * val_t - COST_PER_TOSS;

            curr[h] = best_guess > ev_toss ? best_guess : ev_toss;
        }

        /* Swap */
        double *tmp = prev;
        prev = curr;
        curr = tmp;
    }

    double result = prev[0]; /* local_memo(0, 0) */
    free(prev);
    free(curr);
    return result;
}

int main(void) {
    int N = 50;

    memset(global_memo_set, 0, sizeof(global_memo_set));
    global_memo[0][0] = 0.0;
    global_memo_set[0][0] = 1;

    for (int total_coins = 1; total_coins <= 2 * N; total_coins++) {
        int start_fair = total_coins > N ? total_coins - N : 0;
        int end_fair = total_coins < N ? total_coins : N;

        for (int fair = start_fair; fair <= end_fair; fair++) {
            int unfair = total_coins - fair;

            double val = solve_current_coin(fair, unfair);
            global_memo[fair][unfair] = val;
            global_memo_set[fair][unfair] = 1;
        }
    }

    printf("%.6f\n", global_memo[N][N]);
    return 0;
}
