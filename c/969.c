/* Project Euler 969 - Kangaroo Hops
 * Sum_{n=1}^{10^18} S(n) mod 10^9+7
 * Uses Stirling numbers and primorial factorization.
 */
#include <stdio.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;
#define MOD 1000000007LL

ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    ll M = 1000000000000000000LL; /* 10^18 */
    int maxj = 60;
    int primes[] = {2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59};
    int nprimes = 17;

    /* Stirling numbers of the second kind */
    ll stir[61][61];
    for (int j = 0; j <= maxj; j++)
        for (int i = 0; i <= maxj; i++)
            stir[j][i] = 0;
    stir[0][0] = 1;
    for (int j = 1; j <= maxj; j++) {
        stir[j][0] = 0;
        for (int i = 1; i <= j; i++) {
            stir[j][i] = ((lll)i * stir[j-1][i] + stir[j-1][i-1]) % MOD;
        }
    }

    ll fact[61];
    fact[0] = 1;
    for (int i = 1; i <= maxj; i++)
        fact[i] = fact[i-1] * i % MOD;

    ll total = 0;
    for (int j = 0; j <= maxj; j++) {
        /* Compute primorial P = product of primes <= j */
        ll P = 1;
        int overflow = 0;
        for (int pi = 0; pi < nprimes; pi++) {
            if (primes[pi] > j) break;
            /* Check overflow: P * primes[pi] <= M */
            if (P > (M - (ll)j + 1) / primes[pi]) { overflow = 1; break; }
            P *= primes[pi];
        }
        if (overflow) continue;
        if (P > M - (ll)j + 1) continue;

        ll K = (M - (ll)j) / P;
        if (K <= 0) continue;

        ll s;
        if (j == 0) {
            s = K % MOD;
        } else {
            s = 0;
            for (int i = 1; i <= j; i++) {
                ll falling = 1;
                for (int t = 0; t <= i; t++) {
                    ll term = ((K + 1 - t) % MOD + MOD) % MOD;
                    falling = (lll)falling * term % MOD;
                }
                ll term = (lll)stir[j][i] * falling % MOD;
                ll inv = mod_pow(i + 1, MOD - 2, MOD);
                term = (lll)term * inv % MOD;
                s = (s + term) % MOD;
            }
        }

        ll signs;
        if (j % 2 == 0) signs = 1;
        else signs = MOD - 1;

        ll P_mod = P % MOD;
        ll pj = mod_pow(P_mod, j, MOD);
        ll inv_fact = mod_pow(fact[j], MOD - 2, MOD);
        ll contrib = (lll)signs * pj % MOD;
        contrib = (lll)contrib * s % MOD;
        contrib = (lll)contrib * inv_fact % MOD;
        total = (total + contrib) % MOD;
    }
    printf("%lld\n", total);
    return 0;
}
