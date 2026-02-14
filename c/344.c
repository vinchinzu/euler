/* Project Euler 344 - Silver Dollar Game
 *
 * Count winning configurations in a 1xN board with C coins and 1 silver dollar.
 * Uses Nim theory with XOR-zero DP and CRT.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

ll W(int N, int C, ll mod) {
    int k = C - C / 2;  /* 50 for C=100 */

    /* Precompute factorials and inverse factorials */
    ll *fact = (ll*)malloc((N + 1) * sizeof(ll));
    ll *inv_fact = (ll*)malloc((N + 1) * sizeof(ll));
    fact[0] = 1;
    for (int i = 1; i <= N; i++)
        fact[i] = fact[i-1] * i % mod;
    inv_fact[N] = mod_pow(fact[N], mod - 2, mod);
    for (int i = N - 1; i >= 0; i--)
        inv_fact[i] = inv_fact[i+1] * (i + 1) % mod;

    /* nCr function via closure over fact/inv_fact */
    #define nCr(a, b) ((b) < 0 || (b) > (a) ? 0 : fact[a] % mod * inv_fact[b] % mod * inv_fact[(a)-(b)] % mod)

    /* Small nCr table for XOR zero DP */
    ll **nCrs = (ll**)malloc((k + 2) * sizeof(ll*));
    for (int i = 0; i < k + 2; i++) {
        nCrs[i] = (ll*)calloc(k + 1, sizeof(ll));
        for (int j = 0; j <= k && j <= i; j++)
            nCrs[i][j] = nCr(i, j);
    }

    /* num_xor_zero_sets: count ways that num_piles non-negative integers XOR to 0 and sum to each value */
    int max_sum = N;

    ll *xz = (ll*)calloc(max_sum + 1, sizeof(ll));
    xz[0] = 1;
    int num_piles = C / 2 + 1;
    for (int i = 2; i <= max_sum; i += 2) {
        int lim = num_piles / 2;
        if (lim > i / 2) lim = i / 2;
        for (int np = 0; np <= lim; np++) {
            xz[i] = (xz[i] + xz[i / 2 - np] % mod * nCrs[num_piles][2 * np]) % mod;
        }
    }

    ll *xz_minus = (ll*)calloc(max_sum + 1, sizeof(ll));
    xz_minus[0] = 1;
    int num_piles2 = C / 2;
    for (int i = 2; i <= max_sum; i += 2) {
        int lim = num_piles2 / 2;
        if (lim > i / 2) lim = i / 2;
        for (int np = 0; np <= lim; np++) {
            xz_minus[i] = (xz_minus[i] + xz_minus[i / 2 - np] % mod * nCrs[num_piles2][2 * np]) % mod;
        }
    }

    ll res = (ll)(C + 1) % mod * nCr(N, C + 1) % mod;
    for (int i = 0; i < N - C; i++) {
        res = (res - xz[i] % mod * nCr(N - C - 1 - i + k, k) % mod + mod) % mod;
    }

    ll num_losing_late = 0;
    for (int i = 0; i <= N - C; i++) {
        num_losing_late = (num_losing_late + xz[i] % mod * nCr(N - C - i + k, k)) % mod;
    }
    for (int i = 0; i <= N - C; i++) {
        num_losing_late = (num_losing_late - xz_minus[i] % mod * nCr(N - C - i + k, k) % mod + mod) % mod;
    }

    res = (res - (ll)(C - 1) % mod * num_losing_late % mod + mod) % mod;

    #undef nCr

    for (int i = 0; i < k + 2; i++) free(nCrs[i]);
    free(nCrs);
    free(fact);
    free(inv_fact);
    free(xz);
    free(xz_minus);

    return res;
}

int main(void) {
    int N = 1000000;
    int C = 100;
    ll M1 = 1000003;
    ll M2 = 1000033;

    ll w1 = W(N, C, M1);
    ll w2 = W(N, C, M2);

    /* CRT: find x such that x = w1 mod M1 and x = w2 mod M2 */
    ll M = M1 * M2;
    ll x = ((__int128)w1 * M2 % M * mod_pow(M2, M1 - 2, M1) % M
          + (__int128)w2 * M1 % M * mod_pow(M1, M2 - 2, M2) % M) % M;

    printf("%lld\n", x);
    return 0;
}
