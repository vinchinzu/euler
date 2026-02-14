/*
 * Project Euler Problem 409: Nim Extreme.
 *
 * Find the number of winning positions in a game of Nim with N non-empty
 * piles such that each pile has size less than 2^N and no two piles have
 * the same size.
 *
 * dp[k] = total_positions[k-1] - dp[k-1] - dp[k-2] * (k-1) * (pow2 - (k-1))
 * answer = total_positions[N] - dp[N]
 */
#include <stdio.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL

static ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    int N = 10000000;
    ll M = MOD;

    ll pow2 = pow_mod(2, N, M);

    /* num_positions[k] = product_{i=1}^{k} (pow2 - i) mod M */
    /* We only need the previous two dp values and all num_positions */
    /* But num_positions depends on k, so compute incrementally */

    ll np_prev = 1;  /* num_positions[0] */
    ll dp_prev2 = 0; /* dp[0] = 1, but dp[1] is not used, let's be careful */

    /* dp[0] = 1 (XOR of empty set is 0, which is losing) */
    /* dp[1] is not defined in the recurrence (k starts at 2) */
    /* But we need dp[0] and effectively dp[-1] doesn't exist */
    /* Actually the recurrence says:
     * dp[0] = 1
     * dp[k] = num_positions[k-1] - dp[k-1] - dp[k-2]*(k-1)*(pow2-(k-1))
     * for k >= 2.
     * dp[1] is not explicitly computed by the recurrence either (since k=1
     * would need dp[-1]). Looking at the Python code, dp[1] is never set
     * (stays 0), and the loop starts at k=2. */

    ll dp0 = 1;
    ll dp1 = 0;  /* dp[1] = 0 since it's not computed */
    ll np = 1;   /* num_positions[0] */

    /* Compute num_positions[1] = (pow2 - 1) */
    np = np * ((pow2 - 1 + M) % M) % M;
    /* dp[2] = np[1] - dp[1] - dp[0] * 1 * (pow2 - 1) */

    ll dp_km2 = dp0;  /* dp[k-2] for k=2 is dp[0] */
    ll dp_km1 = dp1;  /* dp[k-1] for k=2 is dp[1] */
    ll np_km1 = np;    /* num_positions[k-1] for k=2 is np[1] */

    for (int k = 2; k <= N; k++) {
        ll new_dp = (np_km1 - dp_km1 - (lll)dp_km2 % M * ((k - 1) % M) % M * ((pow2 - (k - 1) + M) % M) % M) % M;
        new_dp = (new_dp % M + M) % M;

        /* Update num_positions: np[k] = np[k-1] * (pow2 - k) */
        np = np * ((pow2 - k % M + M) % M) % M;

        dp_km2 = dp_km1;
        dp_km1 = new_dp;
        np_km1 = np;
    }

    ll ans = (np - dp_km1 % M + M) % M;
    /* Wait, np at this point is num_positions[N].
     * But np_km1 is num_positions[N-1] since we just finished k=N loop.
     * Actually np was updated to np[k] = np[k-1]*(pow2-k) inside the loop.
     * After k=N: np = num_positions[N], dp_km1 = dp[N]. */

    /* Actually let me re-check. After the loop with k=N:
     * np was updated to np * (pow2 - k) = np[N-1] * (pow2 - N) = np[N]. Hmm,
     * but np was initially np[1], then for k=2 we do np = np * (pow2-2) = np[2].
     * So after k=N, np = np[N]. dp_km1 = dp[N]. Good. */

    printf("%lld\n", (np - dp_km1 + M) % M);
    return 0;
}
