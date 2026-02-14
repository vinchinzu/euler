/*
 * Project Euler Problem 719: Number Splitting.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>

typedef long long ll;

int can_make(ll target, ll digits) {
    if (target < 0 || digits < target) return 0;
    if (digits == 0) return target == 0;
    ll pow_val = 1;
    while (pow_val <= digits) {
        if (can_make(target - digits / pow_val, digits % pow_val))
            return 1;
        pow_val *= 10;
    }
    return 0;
}

int main(void) {
    ll n = 1000000000000LL; /* 10^12 */
    ll ans = 0;
    ll max_i = 1000000; /* isqrt(10^12) */

    for (ll i = 2; i <= max_i; i++) {
        ll i_sq = i * i;
        /* mod 9 filter */
        if (i % 9 == (i_sq % 9)) {
            if (can_make(i, i_sq))
                ans += i_sq;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
