/*
 * Project Euler Problem 322 - Binomial coefficients divisible by 10.
 *
 * Count i in [n, m) where C(i, n) divisible by 10 = lcm(2,5).
 * By Lucas' theorem, C(i,n) is divisible by p iff there's a carry when
 * adding (i-n) and n in base p.
 *
 * Uses inclusion-exclusion for p=2 and p=5.
 * m = 10^18, n = 10^12 - 10.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 i128;

/*
 * Count d in [0, limit) such that adding d and k in base p has no carries.
 * Recursive digit-by-digit processing.
 */
ll num_no_carries(ll limit, ll k, int p) {
    if (limit <= 0) return 0;
    ll largest_pow = 1;
    while (largest_pow * p <= limit)
        largest_pow *= p;

    ll num_remaining = 1;
    ll pp = 1;
    while (pp < largest_pow) {
        num_remaining *= (p - (k / pp % p));
        pp *= p;
    }

    ll result = 0;
    int top_digit_limit = (int)(limit / largest_pow);
    int top_digit_k = (int)(k / largest_pow % p);

    for (int i = 0; i < p; i++) {
        if (i + top_digit_k >= p) break;
        if (i == top_digit_limit) {
            result += num_no_carries(limit % largest_pow, k, p);
        } else if (i < top_digit_limit) {
            result += num_remaining;
        }
    }
    return result;
}

/*
 * Extract digits of k in base 5.
 * Returns number of digits, digits stored in out[] (MSD first).
 */
int to_base5(ll k, int *out, int maxlen) {
    int tmp[200];
    int cnt = 0;
    ll t = k;
    while (t > 0) {
        tmp[cnt++] = (int)(t % 5);
        t /= 5;
    }
    /* Reverse */
    for (int i = 0; i < cnt; i++)
        out[i] = tmp[cnt - 1 - i];
    return cnt;
}

int main(void) {
    ll N = 1000000000000000000LL; /* 10^18 */
    ll K = 1000000000000LL - 10;  /* 10^12 - 10 */
    ll limit = N - K;

    ll no_carry_2 = num_no_carries(limit, K, 2);
    ll no_carry_5 = num_no_carries(limit, K, 5);

    /* Build all no-carry base-5 values for K */
    int k5_digits[200];
    int ndigits = to_base5(K, k5_digits, 200);

    /* Generate all values that have no carry with K in base 5 */
    /* Start with {0}, for each digit d of K, allowed digits are 0..4-d */
    /* Use dynamic array of values */
    ll *vals = (ll *)malloc(sizeof(ll));
    vals[0] = 0;
    int nvals = 1;

    for (int di = 0; di < ndigits; di++) {
        int d = k5_digits[di];
        int allowed = 5 - d; /* digits 0..allowed-1 */
        int new_nvals = nvals * allowed;
        ll *new_vals = (ll *)malloc((size_t)new_nvals * sizeof(ll));
        int idx = 0;
        for (int vi = 0; vi < nvals; vi++) {
            for (int a = 0; a < allowed; a++) {
                new_vals[idx++] = vals[vi] * 5 + a;
            }
        }
        free(vals);
        vals = new_vals;
        nvals = new_nvals;
    }

    ll big_pow5 = 1;
    for (int i = 0; i < ndigits; i++) big_pow5 *= 5;

    /*
     * Count d = val + j*big_pow5 < limit with d & K == 0 (no carry in base 2)
     */
    ll no_carry_both = 0;
    ull K_u = (ull)K;

    for (int vi = 0; vi < nvals; vi++) {
        ll val = vals[vi];
        if (val >= limit) continue;
        ll max_j = (limit - 1 - val) / big_pow5;
        for (ll j = 0; j <= max_j; j++) {
            ull d = (ull)val + (ull)j * (ull)big_pow5;
            if ((d & K_u) == 0)
                no_carry_both++;
        }
    }

    ll ans = limit - no_carry_2 - no_carry_5 + no_carry_both;
    printf("%lld\n", ans);

    free(vals);
    return 0;
}
