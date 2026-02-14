/*
 * Project Euler Problem 371 - Expected plates until a 1000-sum pair.
 *
 * Oregon license plates: numbers 000-999. Seth records numeric suffixes.
 * He wins when he sees two plates whose numbers sum to 1000.
 *
 * Compute the expected number of plates.
 *
 * State: (k, has500) where k = number of active complementary pairs seen
 * (pairs {x, 1000-x} where 1 <= x <= 499), and has500 = whether 500 was seen.
 * 000 is always safe (0+0=0, not 1000).
 *
 * Transitions from state (k, h):
 * - prob of winning: (2k + h) / 1000  [seeing complement of existing number]
 * - prob of seeing new pair: (998 - 2k - h) / 1000  [new x in 1-499 or 501-999]
 * - prob of seeing 500 (if h=0): 1/1000
 * - prob of seeing 000: 1/1000 [no change]
 *
 * E[k][h] = expected additional plates from state (k, h).
 * E[k][h] = 1 + (safe_prob / (1 - 0)) * ... actually:
 *
 * Let p_win = (2k + h) / 1000
 * Let p_new = (998 - 2k - h) / 1000  (new pair member from 1-499 range or complement)
 * Let p_500 = (1 - h) / 1000  (seeing 500 when not yet seen)
 * Let p_000 = 1 / 1000  (seeing 000, no state change)
 *
 * Verification: p_win + p_new + p_500 + p_000
 *   = (2k + h + 998 - 2k - h + 1 - h + 1) / 1000 = (1000 - h) / 1000...
 * Hmm, let me reconsider.
 *
 * There are 1000 numbers: 000-999. Split into classes:
 * - {000}: seeing it is safe (000+000=0 not 1000)
 * - {500}: seeing a second 500 gives 500+500=1000, so it's a self-complementary
 * - 499 complementary pairs: {1,999}, {2,998}, ..., {499,501}
 *
 * State (k, h): k = number of complementary pairs where at least one member seen,
 *               h = 0 or 1, whether 500 has been seen.
 *
 * Dangerous numbers: for each active pair, one complement is dangerous. Plus if h=1, 500 is dangerous.
 * Number of dangerous values = k + h.
 * Wait: if we've seen x from pair {x, 1000-x}, then seeing 1000-x wins.
 * And if we've seen 500, seeing 500 again wins.
 *
 * P(win) = (k + h) / 1000
 *
 * Fresh pair members (from pairs not yet activated): (499 - k) pairs, 2 members each = 2*(499-k)
 * P(new pair) = 2*(499 - k) / 1000   [seeing either member of an inactive pair]
 *     -> transitions to (k+1, h)
 *
 * Seeing 500 when h=0: P = (1-h)/1000 -> transitions to (k, 1)
 *
 * Seeing 000: P = 1/1000 -> stays (k, h)
 *
 * Seeing a non-dangerous member of an active pair: there are k such values
 *     (the already-seen ones, seeing again doesn't win and doesn't change state)
 *   P = k / 1000 -> stays (k, h)
 *
 * Total: (k+h) + 2*(499-k) + (1-h) + 1 + k = k+h + 998-2k + 1-h + 1 + k = 1000. Good.
 *
 * Stays-same probability = (1 + k) / 1000  (seeing 000 or re-seeing already-seen member)
 *
 * E[k][h] = 1 + P(stay)*E[k][h] + P(new pair)*E[k+1][h] + P(500|h=0)*E[k][1]
 *           (terms for win have E=0)
 *
 * Rearranging:
 * E[k][h] * (1 - P(stay)) = 1 + P(new pair)*E[k+1][h] + P(500|h=0)*E[k][1]
 * E[k][h] = (1 + P(new pair)*E[k+1][h] + P(500|h=0)*E[k][1]) / (1 - P(stay))
 *
 * Base: E[499][1] = 1000/500 = 2 (every plate is either winning or safe)
 * Wait: at (499, 1), dangerous = 499 + 1 = 500. P(win) = 500/1000.
 * stay = (1 + 499)/1000 = 500/1000. No new pairs possible.
 * E[499][1] * (1 - 500/1000) = 1  => E[499][1] = 2.
 *
 * Compute backwards from k=499 down to k=0.
 */

#include <stdio.h>

int main(void) {
    double E[500][2];

    /* Base case: k=499, h=1 */
    /* P_stay = (1 + 499)/1000 = 0.5, P_win = 0.5, no new pairs, no 500 transition */
    E[499][1] = 1.0 / (1.0 - 500.0/1000.0);

    /* k=499, h=0 */
    /* P_stay = (1 + 499)/1000 = 0.5, P_win = 499/1000, P_500 = 1/1000, no new pairs */
    /* E[499][0] * (1 - 0.5) = 1 + (1.0/1000.0) * E[499][1] */
    E[499][0] = (1.0 + (1.0/1000.0) * E[499][1]) / (1.0 - 500.0/1000.0);

    /* For k = 498 down to 0 */
    for (int k = 498; k >= 0; k--) {
        double p_stay = (1.0 + k) / 1000.0;
        double p_new = 2.0 * (499 - k) / 1000.0;
        double p_500_0 = 1.0 / 1000.0;  /* when h=0 */

        /* h=1 first (no 500 transition) */
        /* E[k][1] * (1 - p_stay) = 1 + p_new * E[k+1][1] */
        E[k][1] = (1.0 + p_new * E[k+1][1]) / (1.0 - p_stay);

        /* h=0 */
        /* E[k][0] * (1 - p_stay) = 1 + p_new * E[k+1][0] + p_500_0 * E[k][1] */
        E[k][0] = (1.0 + p_new * E[k+1][0] + p_500_0 * E[k][1]) / (1.0 - p_stay);
    }

    printf("%.8f\n", E[0][0]);
    return 0;
}
