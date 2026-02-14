/*
 * Project Euler Problem 319: Bounded Sequences
 *
 * t(n) = sum_{k=1}^n (3^k - 2^k) - sum_{k=2}^n t(floor(n/k))
 * Uses sqrt decomposition for O(n^{3/4}).
 * (Extracted from embedded C in Python solution)
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;

ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (unsigned __int128)result * base % mod;
        base = (unsigned __int128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    ll N = 10000000000LL;
    ll M = 1000000000LL;
    ll M2 = 2 * M;

    int cap = 300000;
    ll *values = (ll *)malloc(cap * sizeof(ll));
    int nv = 0;
    ll k = 1;
    while (k <= N) {
        ll v = N / k;
        values[nv++] = v;
        k = N / v + 1;
    }

    for (int i = 0; i < nv / 2; i++) {
        ll tmp = values[i];
        values[i] = values[nv - 1 - i];
        values[nv - 1 - i] = tmp;
    }

    ll sqN = (ll)sqrtl((long double)N);
    while ((sqN + 1) * (sqN + 1) <= N) sqN++;
    while (sqN * sqN > N) sqN--;

    ll *t_arr = (ll *)calloc(nv, sizeof(ll));

    for (int idx = 0; idx < nv; idx++) {
        ll n = values[idx];
        ll l = (ll)sqrtl((long double)n);
        while ((l + 1) * (l + 1) <= n) l++;
        while (l * l > n) l--;

        ll p3 = mod_pow(3, n + 1, M2);
        ll val3 = (p3 - 1) / 2;
        ll val2 = (mod_pow(2, n + 1, M) - 1 + M) % M;
        ll result = (val3 - val2 + M) % M;

        for (ll kk = 2; kk <= l; kk++) {
            ll nk = n / kk;
            int idx2;
            if (nk <= sqN)
                idx2 = (int)(nk - 1);
            else
                idx2 = nv - (int)(N / nk);
            result = (result - t_arr[idx2] + M) % M;
        }

        ll upper = (l > 0) ? n / l : 1;
        for (ll q = 1; q < upper; q++) {
            ll count = n / q - n / (q + 1);
            ll sub = (count % M) * t_arr[(int)(q - 1)] % M;
            result = (result - sub + M) % M;
        }

        t_arr[idx] = result;
    }

    printf("%lld\n", t_arr[nv - 1]);

    free(values);
    free(t_arr);
    return 0;
}
