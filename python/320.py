"""Project Euler Problem 320 - Factorials divisible by large power.

Embedded C solution for speed. Sieve smallest prime factors, track prime
exponents in i!, compute N(i) via Legendre's formula, sum mod 10^18.

Key optimization: cache the Legendre sum for each prime so we never recompute
from scratch -- we only step forward from the cached position.
"""
import subprocess, tempfile, os, sys

C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;

#define MAX_I 1000000
#define MIN_I 10
#define K 1234567890LL

/* Compute Legendre sum: sum_{k>=1} floor(n / p^k) */
static ll legendre(ll n, ll p) {
    ll cur = 0;
    ll pk = p;
    while (pk <= n) {
        cur += n / pk;
        if (pk > n / p) break; /* overflow guard */
        pk *= p;
    }
    return cur;
}

/* Given current n_val (multiple of p) with leg_sum = legendre(n_val, p),
   advance until leg_sum >= needed. Return updated n_val and leg_sum. */
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
    ull MOD_VAL = 1000000000000000000ULL; /* 10^18 */

    /* Sieve smallest prime factor */
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

    /* Collect primes */
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

    /* exp_f[j] = exponent of primes[j] in current i! */
    ll *exp_f = calloc(np, sizeof(ll));
    if (!exp_f) return 1;

    /* n_for_prime[j] = current N(p) value for prime j (always a multiple of p) */
    ll *n_for_prime = calloc(np, sizeof(ll));
    if (!n_for_prime) return 1;

    /* leg_cache[j] = cached legendre(n_for_prime[j], primes[j]) */
    ll *leg_cache = calloc(np, sizeof(ll));
    if (!leg_cache) return 1;

    /* Pre-accumulate exponents for 2..MIN_I-1 */
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

    /* For primes that already have nonzero exponent, compute initial n_for_prime */
    for (int j = 0; j < np; j++) {
        if (exp_f[j] == 0) continue;
        ll needed = K * exp_f[j];
        ll p = primes[j];
        /* Initial estimate: floor((p-1)*needed / p) * p */
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
                /* First time this prime is seen */
                np_val = (p - 1) * needed;
                np_val -= np_val % p;
                cur = legendre(np_val, p);
            }
            /* Advance from cached position */
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
"""

def main():
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(C_CODE)
        c_path = f.name
    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.run([bin_path], capture_output=True, text=True,
                                timeout=280)
        print(result.stdout.strip())
    finally:
        for p in [c_path, bin_path]:
            if os.path.exists(p):
                os.unlink(p)

if __name__ == "__main__":
    main()
