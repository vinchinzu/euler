/*
 * Project Euler Problem 709: Even Stevens.
 *
 * dp[n] = sum_{k even, 0..n-1} C(n-1, k) * dp[k] * dp[n-k-1]
 * with dp[0] = 1. N = 24680, mod = 1020202009.
 *
 * Precompute Pascal's triangle mod M up to N.
 */
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;

#define N 24680
#define M 1020202009LL

ll ncr[N + 1][2]; /* We only need ncr[i-1][k] - but that's too big. Use 1D rolling. */

int main() {
    /* Precompute binomial coefficients using a flat 1D approach.
     * We need C(i-1, k) for k up to i-1.
     * Store as a 2D array is too big (24681^2). Instead, compute C row by row. */

    /* Actually, we need C(n-1, k) for various n and k. Let's use a row-at-a-time approach
     * for the DP, computing C(i-1, k) as we go. */

    ll *dp = (ll *)calloc(N + 1, sizeof(ll));
    dp[0] = 1;

    /* We'll compute C(i-1, k) row by row */
    ll *row = (ll *)calloc(N + 1, sizeof(ll));

    for (int i = 1; i <= N; i++) {
        /* Build row = C(i-1, 0..i-1) */
        /* Pascal's triangle: row[0] = 1, row[j] = row[j-1] * (i-1-j+1) / j */
        row[0] = 1;
        for (int j = 1; j < i; j++) {
            row[j] = row[j - 1] % M * ((i - j) % M) % M;
            /* Need modular inverse of j */
            /* Use Fermat's little theorem: j^(M-2) mod M */
            /* But M might not be prime... let's check: 1020202009 */
            /* 1020202009 = ? Let's try a different approach. */
            /* Actually, we can build Pascal's triangle row by row using addition. */
            row[j] = 0; /* Reset - we'll use additive Pascal */
        }
        /* Rebuild with additive Pascal */
        /* C(i-1, j) = C(i-2, j-1) + C(i-2, j) */
        /* But we'd need to store previous row. Let's do it properly. */
        break; /* We need a better approach */
    }

    /* Better approach: precompute full Pascal's triangle using 1D rolling.
     * Store only one row at a time, but we need to look up C(i-1, k) for DP.
     * Problem: we need C(i-1, k) for the current i, and k ranges over [0, i-1].
     * We need ALL of these for a single i. So let's store them.
     *
     * Actually, since we're doing DP for i in order, and we need C(i-1, k),
     * let's maintain a rolling Pascal's row. */

    /* C_row[j] = C(current_n, j) */
    ll *C_row = (ll *)calloc(N + 1, sizeof(ll));
    /* Start with C(0, j): C(0, 0) = 1 */
    C_row[0] = 1;

    dp[0] = 1;
    for (int i = 1; i <= N; i++) {
        /* C_row currently holds C(i-1, j) */
        ll val = 0;
        for (int k = 0; k < i; k += 2) {
            val = (val + C_row[k] % M * (dp[k] % M) % M * (dp[i - k - 1] % M)) % M;
        }
        dp[i] = val;

        /* Update C_row from C(i-1, j) to C(i, j) */
        /* C(i, j) = C(i-1, j-1) + C(i-1, j) */
        ll prev = 0;
        for (int j = 0; j <= i; j++) {
            ll tmp = C_row[j];
            C_row[j] = (prev + C_row[j]) % M;
            prev = tmp;
        }
    }

    printf("%lld\n", dp[N]);

    free(dp);
    free(row);
    free(C_row);
    return 0;
}
