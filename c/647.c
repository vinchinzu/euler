/*
 * Project Euler 647 - Linear Transformations of Polygonal Numbers
 * For positive odd k, find all (A,B) with A,B positive integers such that
 * A * X_n + B is always a k-gonal number. Sum A+B over all such pairs
 * for all odd k.
 *
 * A must be an odd perfect square. For sqrt(A) = s (odd), iterate over
 * odd divisors d of (s-1)/2, set k = d+2, compute B.
 */
#include <stdio.h>
#include <math.h>

typedef long long ll;

int main(void) {
    ll N = 1000000000000LL; /* 10^12 */
    int L = (int)sqrt((double)N);
    /* Make sure L*L <= N */
    while ((ll)(L+1)*(L+1) <= N) L++;
    while ((ll)L*L > N) L--;

    ll ans = 0;

    for (int sqrt_a = 1; sqrt_a <= L; sqrt_a += 2) { /* odd sqrt_a */
        ll A = (ll)sqrt_a * sqrt_a;
        int d_max = (sqrt_a - 1) / 2;
        if (d_max == 0) continue;

        /* Iterate over all odd divisors of d_max */
        for (int d = 1; (ll)d * d <= d_max; d++) {
            if (d_max % d != 0) continue;
            /* Process divisor d */
            if (d % 2 == 1) {
                /* k = d + 2 */
                ll B = ((A - 1) / (8 * d)) * ((ll)(d - 2) * (d - 2));
                if (B >= 1 && B <= N) {
                    ans += A + B;
                }
            }
            /* Process complementary divisor d_max/d */
            int d2 = d_max / d;
            if (d2 != d && d2 % 2 == 1) {
                ll B = ((A - 1) / (8 * d2)) * ((ll)(d2 - 2) * (d2 - 2));
                if (B >= 1 && B <= N) {
                    ans += A + B;
                }
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
