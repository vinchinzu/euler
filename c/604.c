/*
 * Project Euler Problem 604: Convex path in square
 *
 * Find the maximum number of lattice points in an NxN square that an
 * increasing strictly convex function can pass through.
 *
 * We add pairs of coprime segments sorted by sum k, each contributing
 * k*phi(k)/2 to the width. After full levels, add individual pairs,
 * and possibly one more coprime segment.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 i128;

static ll gcd_ll(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

int main(void) {
    ll N = 1000000000000000000LL; /* 10^18 */

    /* Sieve phi up to a safe limit. N^(1/3) ~ 10^6, use 3*10^6 */
    int limit = 3000000;
    int *phi = (int*)malloc((limit + 1) * sizeof(int));
    for (int i = 0; i <= limit; i++) phi[i] = i;
    for (int i = 2; i <= limit; i++) {
        if (phi[i] == i) { /* i is prime */
            for (int j = i; j <= limit; j += i)
                phi[j] -= phi[j] / i;
        }
    }

    ll ans = 1;
    ll width = 0;
    int k = 2;

    while (width + (ll)k * phi[k] / 2 <= N) {
        ans += phi[k];
        width += (ll)k * phi[k] / 2;
        k++;
    }

    ll num_additions = (N - width) / k;
    ans += 2 * num_additions;
    width += k * num_additions;

    /* Find last segment */
    int found = 0;
    while (!found && width + (k + 1) / 2 <= N) {
        for (ll big = (k + 1) / 2; big <= N - width; big++) {
            if (gcd_ll(k, big) == 1) {
                ans += 1;
                found = 1;
                break;
            }
        }
        k++;
    }

    printf("%lld\n", ans);

    free(phi);
    return 0;
}
