/*
 * Project Euler Problem 552: Chinese Remainder Theorem / Garner's algorithm.
 * For each prime p_i, check if any partial CRT reconstruction A_n (n < i)
 * is divisible by p_i.
 *
 * Extracted from embedded C in Python solution, made standalone with
 * integrated prime sieve.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;

int main(void) {
    int N = 300000;

    /* Sieve primes up to N */
    char *is_prime = (char*)calloc(N + 1, 1);
    memset(is_prime + 2, 1, N - 1);
    for (int i = 2; (ll)i * i <= N; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= N; j += i)
                is_prime[j] = 0;
        }
    }

    int L = 0;
    for (int i = 2; i <= N; i++)
        if (is_prime[i]) L++;

    ll *primes = (ll*)malloc(L * sizeof(ll));
    int idx = 0;
    for (int i = 2; i <= N; i++)
        if (is_prime[i]) primes[idx++] = i;
    free(is_prime);

    ll *garner = (ll*)calloc(L, sizeof(ll));
    ll ans = 0;

    for (int i = 0; i < L; i++) {
        ll p = primes[i];
        ll prod = 1;
        ll A = 0;
        int good = 0;

        for (int j = 0; j < i; j++) {
            A = (A + prod % p * (garner[j] % p)) % p;
            prod = prod % p * (primes[j] % p) % p;
            if (A == 0 && j > 0) {
                good = 1;
            }
        }

        /* Compute garner[i] */
        if (prod % p != 0) {
            ll need = ((i + 1 - A) % p + p) % p;
            /* modular inverse of prod mod p using Fermat's little theorem */
            ll base = prod % p;
            ll exp = p - 2;
            ll inv = 1;
            while (exp > 0) {
                if (exp & 1) inv = inv * base % p;
                base = base * base % p;
                exp >>= 1;
            }
            garner[i] = need * inv % p;
        } else {
            garner[i] = 0;
        }

        if (good) {
            ans += p;
        }
    }

    printf("%lld\n", ans);
    free(primes);
    free(garner);
    return 0;
}
