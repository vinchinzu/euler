/*
 * Project Euler Problem 320: Factorials divisible by large power
 *
 * Sieve smallest prime factors, track prime exponents in i!,
 * compute N(i) via Legendre's formula, sum mod 10^18.
 * (Extracted from embedded C in Python solution)
 *
 * Fixed: Use binary search instead of linear stepping in advance()
 * to handle small primes (p=2,3,5) where needed can jump by K per step.
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

/* Find the smallest multiple of p, n_val, such that legendre(n_val, p) >= needed.
 * Start from *pn_val (a multiple of p) with legendre sum *pleg_sum.
 * If the gap is large, use binary search first. */
static void advance(ll p, ll needed, ll *pn_val, ll *pleg_sum) {
    ll n_val = *pn_val;
    ll cur = *pleg_sum;

    if (cur >= needed) {
        /* Already there */
        *pn_val = n_val;
        *pleg_sum = cur;
        return;
    }

    /* If gap is large, use binary search to find approximate position */
    ll gap = needed - cur;
    if (gap > 1000) {
        /* Estimate: legendre(n, p) ~ n/(p-1). So we need roughly delta_n = gap * (p-1) more. */
        ll est = n_val + gap * (p - 1);
        /* Round down to multiple of p */
        est -= est % p;
        /* Make sure est >= n_val */
        if (est < n_val) est = n_val;

        /* Binary search between n_val and est*2 for the right position */
        ll lo = n_val, hi = est + gap * (p - 1);
        hi -= hi % p;
        if (hi < lo) hi = lo;

        /* First check if legendre(hi) >= needed */
        ll leg_hi = legendre(hi, p);
        while (leg_hi < needed) {
            hi *= 2;
            hi -= hi % p;
            leg_hi = legendre(hi, p);
        }

        /* Binary search for smallest multiple of p with legendre >= needed */
        while (lo < hi) {
            ll mid = lo + (hi - lo) / 2;
            mid -= mid % p;
            if (mid < lo) { mid = lo; break; }
            if (mid == lo) break;
            ll leg_mid = legendre(mid, p);
            if (leg_mid >= needed) {
                hi = mid;
            } else {
                lo = mid + p;
                lo -= lo % p;
            }
        }
        n_val = lo;
        cur = legendre(n_val, p);
    }

    /* Fine-tune with linear stepping (should be at most a few steps) */
    while (cur < needed) {
        n_val += p;
        ll kk = n_val;
        while (kk % p == 0) {
            cur++;
            kk /= p;
        }
    }

    /* Also go back if we overshot too far (find the SMALLEST multiple with leg >= needed) */
    /* Actually, we want the smallest n_val (multiple of p) with legendre >= needed.
     * The linear stepping above guarantees we stop at the first one from below. */

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
