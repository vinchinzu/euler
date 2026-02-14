/*
 * Project Euler Problem 617: Mirror Power Sequence
 *
 * Count (n, e)-MPS sequences where n <= 10^18.
 */
#include <stdio.h>
#include <stdint.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 i128;

static ll isqrt_ll(ll n) {
    ll x = (ll)sqrtl((long double)n);
    while (x * x > n) x--;
    while ((x + 1) * (x + 1) <= n) x++;
    return x;
}

/* Safe integer power: returns 0 if overflow would occur past limit */
static ll safe_pow(ll base, int exp, ll limit) {
    ll result = 1;
    for (int i = 0; i < exp; i++) {
        if (result > limit / base) return limit + 1; /* overflow sentinel */
        result *= base;
    }
    return result;
}

int main(void) {
    ll N = 1000000000000000000LL; /* 10^18 */

    ll ans = isqrt_ll(N) - 2;

    for (ll a0 = 2; ; a0++) {
        ll a0_cubed = safe_pow(a0, 3, N);
        if (a0_cubed + a0 > N) break;

        for (int e = 2; ; e++) {
            ll a0_e = safe_pow(a0, e, N);
            if (a0_e + a0 > N) break;

            /* Build tower */
            ll as_list[100];
            int as_len = 0;
            ll a = a0;
            while (1) {
                ll ae = safe_pow(a, e, N);
                if (ae > N) break;
                as_list[as_len++] = a;
                a = ae;
                if (as_len >= 100) break;
            }

            for (int start = 0; start < as_len; start++) {
                for (int end = start; end < as_len; end++) {
                    ll ae = safe_pow(as_list[end], e, N);
                    if (ae > N) break;
                    if ((e > 2 || end > 0) && ae + as_list[start] <= N) {
                        ans += (start == 0) ? end + 1 : 1;
                    }
                }
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
