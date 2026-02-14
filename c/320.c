/*
 * Project Euler Problem 320: Factorials divisible by large power
 *
 * Sieve smallest prime factors, track prime exponents in i!,
 * compute N(i) via Legendre's formula, sum mod 10^18.
 * (Extracted from embedded C in Python solution)
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;

#define MAX_I 1000000
#define MIN_I 10
#define K 1234567890LL

static ll legendre(ll n, ll p) {
    ll cur = 0;
    ll pk = p;
    while (pk <= n) {
        cur += n / pk;
        if (pk > n / p) break;
        pk *= p;
    }
    return cur;
}

static void advance(ll p, ll needed, ll *pn_val, ll *pleg_sum) {
    ll n_val = *pn_val;
    ll cur = *pleg_sum;
    while (cur < needed) {
        n_val += p;
        ll kk = n_val;
        while (kk % p == 0) {
            cur++;
            kk /= p;
        }
    }
    *pn_val = n_val;
    *pleg_sum = cur;
}

int main(void) {
    ull MOD_VAL = 1000000000000000000ULL;

    int *spf = malloc((MAX_I + 1) * sizeof(int));
    if (!spf) return 1;
    for (int i = 0; i <= MAX_I; i++) spf[i] = i;
    for (int i = 2; (ll)i * i <= MAX_I; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j <= MAX_I; j += i) {
                if (spf[j] == j) spf[j] = i;
            }
        }
    }

    int np = 0;
    for (int i = 2; i <= MAX_I; i++)
        if (spf[i] == i) np++;

    int *primes = malloc(np * sizeof(int));
    if (!primes) return 1;
    int *pidx = calloc(MAX_I + 1, sizeof(int));
    if (!pidx) return 1;

    int idx = 0;
    for (int i = 2; i <= MAX_I; i++) {
        if (spf[i] == i) {
            primes[idx] = i;
            pidx[i] = idx;
            idx++;
        }
    }

    ll *exp_f = calloc(np, sizeof(ll));
    if (!exp_f) return 1;
    ll *n_for_prime = calloc(np, sizeof(ll));
    if (!n_for_prime) return 1;
    ll *leg_cache = calloc(np, sizeof(ll));
    if (!leg_cache) return 1;

    for (int j = 2; j < MIN_I; j++) {
        int n = j;
        while (n > 1) {
            int p = spf[n];
            int pi = pidx[p];
            while (n % p == 0) {
                n /= p;
                exp_f[pi]++;
            }
        }
    }

    for (int j = 0; j < np; j++) {
        if (exp_f[j] == 0) continue;
        ll needed = K * exp_f[j];
        ll p = primes[j];
        ll np_val = (p - 1) * needed;
        np_val -= np_val % p;
        ll cur = legendre(np_val, p);
        advance(p, needed, &np_val, &cur);
        n_for_prime[j] = np_val;
        leg_cache[j] = cur;
    }

    ll max_n = 0;
    for (int j = 0; j < np; j++) {
        if (n_for_prime[j] > max_n) max_n = n_for_prime[j];
    }

    ull ans = 0;
    int changed_buf[20];

    for (int i = MIN_I; i <= MAX_I; i++) {
        int n = i;
        int num_changed = 0;
        while (n > 1) {
            int p = spf[n];
            int pi = pidx[p];
            int already = 0;
            for (int c = 0; c < num_changed; c++) {
                if (changed_buf[c] == pi) { already = 1; break; }
            }
            if (!already) changed_buf[num_changed++] = pi;
            while (n % p == 0) {
                n /= p;
                exp_f[pi]++;
            }
        }

        for (int c = 0; c < num_changed; c++) {
            int pi = changed_buf[c];
            ll p = primes[pi];
            ll needed = K * exp_f[pi];
            ll np_val = n_for_prime[pi];
            ll cur = leg_cache[pi];

            if (np_val == 0) {
                np_val = (p - 1) * needed;
                np_val -= np_val % p;
                cur = legendre(np_val, p);
            }
            advance(p, needed, &np_val, &cur);
            n_for_prime[pi] = np_val;
            leg_cache[pi] = cur;

            if (np_val > max_n) max_n = np_val;
        }

        ans = (ans + (ull)max_n) % MOD_VAL;
    }

    printf("%llu\n", ans);

    free(spf);
    free(primes);
    free(pidx);
    free(exp_f);
    free(n_for_prime);
    free(leg_cache);
    return 0;
}
