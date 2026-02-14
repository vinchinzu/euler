/*
 * Project Euler Problem 326
 *
 * a_n sequence with period-6 closed form.
 * Prefix sums S(n) mod M are periodic with period 6M.
 * Count pairs (p,q) with 1<=p<=q<=N where sum_{i=p}^{q} a_i = 0 mod M.
 *
 * N = 10^12, M = 10^6.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

int main(void) {
    ll N = 1000000000000LL;  /* 10^12 */
    ll Mval = 1000000;       /* 10^6 */
    ll period = 6 * Mval;    /* 6 * 10^6 */

    /* Build a[0..period-1] using closed form */
    int *a = (int *)malloc(period * sizeof(int));
    for (ll i = 0; i < Mval; i++) {
        a[6 * i]     = (int)(3 * i);
        a[6 * i + 1] = (int)(4 * i + 1);
        a[6 * i + 2] = (int)(3 * i + 1);
        a[6 * i + 3] = (int)i;
        a[6 * i + 4] = (int)(6 * i + 3);
        a[6 * i + 5] = (int)i;
    }

    /* Count occurrences of each S(i) mod M value */
    ll *counts = (ll *)calloc(Mval, sizeof(ll));
    ll s = 0;
    for (ll i = 0; i < period; i++) {
        s = (s + a[i]) % Mval;
        /* This S value appears at indices i, i+period, i+2*period, ... up to N */
        counts[s] += (N - i + period) / period;
    }

    ll ans = 0;
    for (ll c_idx = 0; c_idx < Mval; c_idx++) {
        ll c = counts[c_idx];
        ans += c * (c - 1) / 2;
    }

    printf("%lld\n", ans);

    free(a);
    free(counts);
    return 0;
}
