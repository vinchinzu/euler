/*
 * Project Euler Problem 519: Tricolored Coin Fountains.
 * Count 3-colorings over all fountains of N coins.
 * DP with suffix sums.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;

#define N_VAL 20000
#define M_VAL 1000000000LL

int main() {
    int N = N_VAL;
    int L = (int)sqrt(2.0 * N) + 1;
    ll M = M_VAL;

    /* dp[n][k] and suf[n][k] - use rolling arrays since we only need dp[n-k] */
    /* Actually, dp[n][k] depends on suf[n-k][k-1] and dp[n-k][1], which means
       we need access to all previous rows. Allocate full arrays. */

    /* Use flattened arrays: dp[n*(L+1) + k] */
    ll *dp = (ll*)calloc((ll)(N + 1) * (L + 1), sizeof(ll));
    ll *suf = (ll*)calloc((ll)(N + 1) * (L + 2), sizeof(ll));

    #define DP(n, k) dp[(ll)(n) * (L + 1) + (k)]
    #define SUF(n, k) suf[(ll)(n) * (L + 2) + (k)]

    DP(1, 1) = 3;
    for (int prev = L; prev >= 0; prev--)
        SUF(1, prev) = (SUF(1, prev + 1) + DP(1, prev)) % M;

    for (int n = 2; n <= N; n++) {
        int kmax = (L < n) ? L : n;
        for (int k = 1; k <= kmax; k++) {
            if (n - k < 0) continue;
            if (k == 1) {
                DP(n, k) = (2 * SUF(n - 1, 1)) % M;
            } else {
                ll total = SUF(n - k, k - 1);
                if (k == 2)
                    total = (total + DP(n - k, 1)) % M;
                DP(n, k) = total % M;
            }
        }
        for (int prev = L; prev >= 0; prev--)
            SUF(n, prev) = (SUF(n, prev + 1) + DP(n, prev)) % M;
    }

    ll ans = 0;
    for (int k = 1; k <= L; k++)
        ans = (ans + DP(N, k)) % M;

    printf("%lld\n", ans);

    free(dp);
    free(suf);
    return 0;
}
