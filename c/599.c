/*
 * Project Euler Problem 599: Distinct colorings of a 2x2x2 Rubik's Cube.
 *
 * The number of distinct colorings using N colors.
 */

#include <stdio.h>
#include <stdint.h>

typedef long long ll;

/* Compute C(n, k) */
ll comb(int n, int k) {
    if (k < 0 || k > n) return 0;
    if (k == 0 || k == n) return 1;
    if (k > n - k) k = n - k;
    ll result = 1;
    for (int i = 0; i < k; i++) {
        result = result * (n - i) / (i + 1);
    }
    return result;
}

int main() {
    int N = 10;

    ll num_multicolored_corners = (ll)N * (N - 1) * (N - 2) / 3 + (ll)N * (N - 1);
    ll num_corners = num_multicolored_corners + N;

    ll ans = comb((int)(num_corners + 7), 8) + 2 * comb((int)(num_multicolored_corners + 7), 8);

    printf("%lld\n", ans);
    return 0;
}
