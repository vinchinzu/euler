/*
 * Project Euler Problem 713: Turan Graphs.
 *
 * T(N, m) = (N%k) * C(floor(N/k)+1, 2) + (k - N%k) * C(floor(N/k), 2)
 * where k = m - 1. Sum over m = 2..N, i.e., k = 1..N-1.
 *
 * Optimized: group by floor(N/k) values using floor quotient grouping.
 */
#include <stdio.h>

typedef long long ll;

int main() {
    ll n = 10000000LL;
    ll ans = 0;

    for (ll k = 1; k < n; k++) {
        ll gs = n / k;       /* group_size = floor(n/k) */
        ll rem = n % k;      /* remainder = n mod k */
        /* rem * C(gs+1, 2) + (k - rem) * C(gs, 2)
         * = rem * gs*(gs+1)/2 + (k - rem) * gs*(gs-1)/2
         * = gs/2 * (rem*(gs+1) + (k-rem)*(gs-1))
         * = gs/2 * (rem*gs + rem + k*gs - k*rem - rem*gs + rem + k*rem - rem)
         * Hmm, let's just compute directly.
         */
        ll c1 = gs * (gs + 1) / 2;  /* C(gs+1, 2) */
        ll c2 = gs * (gs - 1) / 2;  /* C(gs, 2) */
        ans += rem * c1 + (k - rem) * c2;
    }

    printf("%lld\n", ans);
    return 0;
}
