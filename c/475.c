/*
 * Project Euler Problem 475: Music festival
 *
 * Count ways to rearrange N=600 musicians from N/K=150 quartets (K=4)
 * into N/3=200 trios such that no two from same quartet share a trio.
 *
 * Recursive DP with memoization: f(m1, m2, m3) counts ways to form
 * groups of K from m1 singles, m2 pairs, m3 triples.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 i128;

#define NN 600
#define KK 4
#define MOD 1000000007LL

static ll fact[KK * NN + 1];
static ll inv_fact[KK * NN + 1];

ll pow_mod(ll base, ll exp, ll m) {
    ll result = 1;
    base %= m;
    while (exp > 0) {
        if (exp & 1) result = (i128)result * base % m;
        base = (i128)base * base % m;
        exp >>= 1;
    }
    return result;
}

void init_factorials(void) {
    int limit = KK * NN;
    fact[0] = 1;
    for (int i = 1; i <= limit; i++)
        fact[i] = (i128)fact[i - 1] * i % MOD;
    inv_fact[limit] = pow_mod(fact[limit], MOD - 2, MOD);
    for (int i = limit - 1; i >= 0; i--)
        inv_fact[i] = (i128)inv_fact[i + 1] * (i + 1) % MOD;
}

ll nCr(int n, int r) {
    if (r < 0 || r > n) return 0;
    return (i128)fact[n] * inv_fact[r] % MOD * inv_fact[n - r] % MOD;
}

/* Cache: m1 can be 0..200, m2 can be 0..200, m3 can be 0..200
 * But total m1+m2+m3 decreases by 1 each step (we form one group of K from
 * d1+d2+d3=K people, and the new state has m1-d1+d2, m2-d2+d3, m3-d3).
 * m1+m2+m3 = m1+m2+m3 - d1 + d2 - d2 + d3 - d3 = m1+m2+m3 - d1
 * Hmm, that's not exactly right. Let me trace:
 * new_m1 = m1 - d1 + d2
 * new_m2 = m2 - d2 + d3
 * new_m3 = m3 - d3
 * new total = m1+m2+m3 - d1 + d2 - d2 + d3 - d3 = m1+m2+m3 - d1
 * Since d1+d2+d3 = K, this doesn't cleanly reduce. Max values are bounded though.
 */
#define MAX_M 201
static ll cache[MAX_M][MAX_M][MAX_M];
static char cached[MAX_M][MAX_M][MAX_M];

ll f(int m1, int m2, int m3) {
    if (m1 + m2 + m3 == 0) return 1;
    if (m1 < 0 || m2 < 0 || m3 < 0) return 0;
    if (cached[m1][m2][m3]) return cache[m1][m2][m3];
    cached[m1][m2][m3] = 1;

    ll result = 0;
    for (int d1 = 0; d1 <= m1 && d1 <= KK; d1++) {
        for (int d2 = 0; d2 <= m2 && d2 <= KK - d1; d2++) {
            int d3 = KK - d1 - d2;
            if (d3 < 0 || d3 > m3) continue;
            ll ways = nCr(m1, d1);
            ways = (i128)ways * pow_mod(2, d2, MOD) % MOD;
            ways = (i128)ways * nCr(m2, d2) % MOD;
            ways = (i128)ways * pow_mod(3, d3, MOD) % MOD;
            ways = (i128)ways * nCr(m3, d3) % MOD;
            ll sub = f(m1 - d1 + d2, m2 - d2 + d3, m3 - d3);
            result = (result + (i128)ways * sub) % MOD;
        }
    }

    cache[m1][m2][m3] = result;
    return result;
}

int main(void) {
    init_factorials();
    memset(cached, 0, sizeof(cached));

    ll ans = f(0, 0, NN / 3);

    /* Multiply by (K!)^(N/K) */
    ans = (i128)ans * pow_mod(fact[KK], NN / KK, MOD) % MOD;
    /* Multiply by (1/3!)^(N/3) */
    ans = (i128)ans * pow_mod(inv_fact[3], NN / 3, MOD) % MOD;
    /* Multiply by 1/(N/3)! */
    ans = (i128)ans * inv_fact[NN / 3] % MOD;

    printf("%lld\n", ans);
    return 0;
}
