/*
 * Project Euler Problem 702: Jumping Flea.
 *
 * Compute the sum of J(T) for all upward facing triangles in the top half of
 * a hexagon with side length N = 123456789.
 *
 * Uses recursive inversion counting on sequences modulo powers of 2.
 */
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;
typedef __int128 lll;

ll tr(ll n) {
    return n * (n + 1) / 2;
}

ll iceil_div(ll a, ll b) {
    return (a + b - 1) / b;
}

ll num_inversions(ll n, ll m) {
    if (n == 0) return 0;
    ll big = m % n;
    ll small = n - big;
    ll count = (n * (n - 1) / 2) * tr(m / n);
    if (big > 0) {
        count += num_inversions(n % big, big) * iceil_div(m, n);
    }
    if (small > 0) {
        count -= num_inversions(n % small, small) * (m / n);
    }
    return count;
}

ll num_points_in_shaded(ll n, ll point_dist) {
    return (point_dist - 2) * (point_dist - 1) - num_inversions(n % point_dist, point_dist);
}

int main() {
    ll n = 123456789LL;

    /* Find smallest power of 2 >= n */
    ll l = 1;
    while (l < n) l *= 2;

    /* Compute num_shaded_with_j for each power of 2 */
    int max_j = 0;
    ll *nsj = malloc(100 * sizeof(ll));

    ll k = 1;
    while (k < l) {
        nsj[max_j++] = num_points_in_shaded(n, k);
        k *= 2;
    }

    /* For k == l */
    ll lmod = (-l % n + n) % n;
    nsj[max_j++] = num_points_in_shaded(n, l) - 2 * num_points_in_shaded(lmod, l - n);

    /* For final value */
    nsj[max_j++] = 2 * tr(n) + tr(n - 1);

    ll ans = 0;
    for (int j = 1; j < max_j; j++) {
        ans += (ll)j * (nsj[j] - nsj[j - 1]);
    }

    printf("%lld\n", ans);
    free(nsj);
    return 0;
}
