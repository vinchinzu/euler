/*
 * Project Euler 434 - Rigid graphs
 *
 * R(m,n) = number of ways to add diagonals to m*n grid such that
 * the bipartite graph is connected.
 * Translated from python/434.py.
 */
#include <stdio.h>

typedef long long ll;
typedef __int128 lll;

#define NMAX 100
#define M 1000000033LL

static ll ncr_table[NMAX + 1][NMAX + 1];
static ll pow2s[NMAX * NMAX + 1];
static ll R[NMAX + 1][NMAX + 1];

static ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    /* Precompute binomial coefficients */
    for (int i = 0; i <= NMAX; i++) {
        ncr_table[i][0] = 1;
        for (int j = 1; j <= i; j++)
            ncr_table[i][j] = (ncr_table[i-1][j-1] + ncr_table[i-1][j]) % M;
    }

    /* Precompute powers of 2 */
    int max_pow = NMAX * NMAX;
    pow2s[0] = 1;
    for (int i = 1; i <= max_pow; i++)
        pow2s[i] = pow2s[i-1] * 2 % M;

    /* Compute R[m][n] */
    for (int m = 1; m <= NMAX; m++) {
        for (int n = 0; n <= NMAX; n++) {
            R[m][n] = pow2s[m * n];
            for (int a = 1; a <= m; a++) {
                for (int b = 0; b <= n; b++) {
                    if (a < m || b < n) {
                        ll sub = (lll)ncr_table[m-1][a-1] * ncr_table[n][b] % M;
                        sub = (lll)sub * R[a][b] % M;
                        sub = (lll)sub * pow2s[(m-a) * (n-b)] % M;
                        R[m][n] = (R[m][n] - sub % M + M) % M;
                    }
                }
            }
        }
    }

    ll ans = 0;
    for (int m = 1; m <= NMAX; m++)
        for (int n = 1; n <= NMAX; n++)
            ans = (ans + R[m][n]) % M;

    ans = (ans + M) % M;
    printf("%lld\n", ans);
    return 0;
}
