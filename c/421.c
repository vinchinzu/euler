/*
 * Project Euler 421 - Prime factors of n^15+1
 *
 * For each prime p <= K, contribution is p * (number of n in [1,N] with p | n^15+1).
 * n^15 = -1 (mod p) means n is in a coset of the 15th roots of unity mod p.
 * Extracted from embedded C in python/421.py.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __uint128_t u128;

#define KMAX 100000001

static char is_composite[KMAX];

void sieve(int limit) {
    memset(is_composite, 0, limit + 1);
    is_composite[0] = is_composite[1] = 1;
    for (int i = 2; (ll)i * i <= limit; i++) {
        if (!is_composite[i]) {
            for (ll j = (ll)i * i; j <= limit; j += i)
                is_composite[j] = 1;
        }
    }
}

static inline ull pow_mod(ull base, ull exp, ull mod) {
    ull result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (u128)result * base % mod;
        base = (u128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

ll gcd_ll(ll a, ll b) { while (b) { ll t = b; b = a % b; a = t; } return a; }

int main() {
    ll N = 100000000000LL;  /* 10^11 */
    int K = 100000000;       /* 10^8 */
    int R = 15;

    sieve(K);

    ll ans = 0;

    for (ll p = 2; p <= K; p++) {
        if (is_composite[p]) continue;

        ll g_val = gcd_ll(p - 1, R);

        ull nth_root = 1;
        for (ull g = 1; g < (ull)p; g++) {
            if (g == 1) {
                nth_root = 1;
            } else {
                nth_root = pow_mod(g, (p - 1) / g_val, p);
            }
            ull r = nth_root;
            int e = 1;
            while (r != 1) {
                r = (u128)r * nth_root % p;
                e++;
            }
            if (e == g_val) break;
        }

        ull r = 1;
        for (int e = 0; e < g_val; e++) {
            ans += p * ((N + (ll)r) / p);
            r = (u128)r * nth_root % p;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
