/*
 * Project Euler 659 - Largest prime
 * P(k) = largest prime dividing (2k)^2 + 1.
 * Sieve approach: for each prime p = 1 mod 4, find roots of -1 mod p,
 * then divide out p from all P[k] where (2k)^2 + 1 = 0 mod p.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;

#define N_VAL 10000000
#define M_VAL 1000000000000000000LL

/* Tonelli-Shanks: compute sqrt(-1) mod p for p = 1 mod 4 */
static ll sqrt_mod_neg1(ll p) {
    /* Find sqrt(-1) mod p */
    /* -1 = p-1, need x^2 = p-1 mod p */
    /* p = 1 mod 4, so (p-1)/2 is even */

    if (p % 4 == 3) return -1; /* no solution */

    /* p-1 = Q * 2^S */
    ll Q = p - 1;
    int S = 0;
    while (Q % 2 == 0) { Q /= 2; S++; }

    /* Find quadratic non-residue */
    ll z = 2;
    while (1) {
        ll test = 1;
        ll base = z % p;
        ll exp = (p - 1) / 2;
        ll b2 = base;
        ll e2 = exp;
        while (e2 > 0) {
            if (e2 & 1) test = (__int128)test * b2 % p;
            b2 = (__int128)b2 * b2 % p;
            e2 >>= 1;
        }
        if (test == p - 1) break;
        z++;
    }

    int M_s = S;
    ll c = 1;
    { ll base = z % p; ll exp = Q;
      while (exp > 0) { if (exp & 1) c = (__int128)c * base % p; base = (__int128)base * base % p; exp >>= 1; }
    }
    ll t = 1;
    { ll base = (p - 1) % p; ll exp = Q;
      while (exp > 0) { if (exp & 1) t = (__int128)t * base % p; base = (__int128)base * base % p; exp >>= 1; }
    }
    ll R = 1;
    { ll base = (p - 1) % p; ll exp = (Q + 1) / 2;
      while (exp > 0) { if (exp & 1) R = (__int128)R * base % p; base = (__int128)base * base % p; exp >>= 1; }
    }

    while (t != 1) {
        int i = 1;
        ll tt = (__int128)t * t % p;
        while (tt != 1) { tt = (__int128)tt * tt % p; i++; }

        ll b = c;
        for (int j = 0; j < M_s - i - 1; j++) b = (__int128)b * b % p;
        M_s = i;
        c = (__int128)b * b % p;
        t = (__int128)t * c % p;
        R = (__int128)R * b % p;
    }

    return R;
}

int main(void) {
    int N = N_VAL;

    /* P[k] = (2k)^2 + 1 */
    ll *P = (ll *)malloc((N + 1) * sizeof(ll));
    for (int k = 1; k <= N; k++) {
        P[k] = (ll)(2 * k) * (2 * k) + 1;
    }

    /* Sieve primes up to 2*N that are 1 mod 4 */
    int sieve_limit = 2 * N;
    char *is_prime = (char *)malloc(sieve_limit + 1);
    memset(is_prime, 1, sieve_limit + 1);
    is_prime[0] = is_prime[1] = 0;
    int sq = (int)sqrt((double)sieve_limit);
    for (int i = 2; i <= sq; i++)
        if (is_prime[i])
            for (int j = i * i; j <= sieve_limit; j += i)
                is_prime[j] = 0;

    for (int p = 5; p <= sieve_limit; p++) {
        if (!is_prime[p]) continue;
        if (p % 4 != 1) continue;

        ll sv = sqrt_mod_neg1(p);
        if (sv < 0) continue;

        /* We need (2k)^2 + 1 = 0 mod p, so 2k = +/- sv mod p */
        /* k = sv * inv(2) mod p, or k = (p - sv) * inv(2) mod p */
        ll inv2 = (p + 1) / 2;
        ll k_starts[2];
        k_starts[0] = ((__int128)sv * inv2) % p;
        k_starts[1] = ((__int128)(p - sv) * inv2) % p;

        for (int si = 0; si < 2; si++) {
            ll ks = k_starts[si];
            if (ks == 0) ks = p;
            for (ll k = ks; k <= N; k += p) {
                while (P[k] % p == 0 && P[k] > p) {
                    P[k] /= p;
                }
            }
        }
    }

    free(is_prime);

    ll ans = 0;
    for (int k = 1; k <= N; k++) {
        ans = (ans + P[k]) % M_VAL;
    }

    printf("%lld\n", ans);

    free(P);
    return 0;
}
