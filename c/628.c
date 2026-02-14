/*
 * Project Euler 628: Open chess positions
 *
 * Number of pawn configurations on NxN board with N pawns (1 per row/col)
 * such that a rook can move from bottom-left to top-right.
 *
 * ans = (N! - 1) - 2*sum_{k=1}^{N-1} k! + sum_{k=0}^{N-2} (N-1-k)*k!
 * All mod M = 1008691207.
 */
#include <stdio.h>

typedef long long ll;
typedef __int128 lll;

#define N_VAL 100000000
#define M_VAL 1008691207LL

int main() {
    /* Precompute factorials mod M iteratively (no array needed for 10^8) */
    /* Actually we need random access to factorials[k] in the sums.
     * But we can compute the sums in a single pass. */

    ll fact_k = 1; /* k! */
    ll sum1 = 0;   /* sum_{k=1}^{N-1} k! */
    ll sum2 = 0;   /* sum_{k=0}^{N-2} (N-1-k) * k! */
    ll fact_N = 1; /* will be N! at end */

    for (int k = 0; k <= N_VAL; k++) {
        if (k > 0) fact_k = (lll)fact_k * k % M_VAL;

        if (k >= 1 && k <= N_VAL - 1) {
            sum1 = (sum1 + fact_k) % M_VAL;
        }

        if (k <= N_VAL - 2) {
            ll coeff = (N_VAL - 1 - k) % M_VAL;
            sum2 = (sum2 + (lll)coeff * fact_k) % M_VAL;
        }

        if (k == N_VAL) fact_N = fact_k;
    }

    ll ans = ((fact_N - 1 - 2 * sum1 + sum2) % M_VAL + M_VAL) % M_VAL;
    printf("%lld\n", ans);
    return 0;
}
