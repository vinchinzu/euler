/*
 * Project Euler Problem 555: McCarthy 91 function.
 * Find sum_{1<=s<k<=N} SF(M, k, s) where M = N = 10^6.
 *
 * Fixed points exist when s is divisible by d = k - s.
 * The sum of fixed points for given (s, d) is triangular(m+d-s) - triangular(m-s).
 */
#include <stdio.h>

typedef long long ll;

static ll triangular(ll n) {
    if (n < 0) return 0;
    return n * (n + 1) / 2;
}

int main(void) {
    int N = 1000000;
    int m_param = 1000000;

    ll ans = 0;
    for (int d = 1; d <= N / 2; d++) {
        for (int s = d; s <= N - d; s += d) {
            ll t1 = triangular((ll)m_param + d - s);
            ll t2 = triangular((ll)m_param - s);
            ans += t1 - t2;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
