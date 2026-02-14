/*
 * Project Euler Problem 804: Counting Binary Quadratic Representations.
 *
 * Count lattice points with x^2+xy+41y^2 <= N using completing the square:
 * (2x+y)^2 + 163*y^2 <= 4N. Enumerate y, count valid u=2x+y with matching parity.
 */
#include <stdio.h>
#include <math.h>

typedef long long ll;

ll isqrt_ll(ll n) {
    if (n < 0) return -1;
    ll x = (ll)sqrtl((long double)n);
    while (x > 0 && x * x > n) x--;
    while ((x + 1) * (x + 1) <= n) x++;
    return x;
}

int main() {
    ll N = 10000000000000000LL;  /* 10^16 */
    ll coeff = 163;  /* 4*41 - 1 */
    ll four_N = 4 * N;

    ll max_y = isqrt_ll(four_N / coeff);
    while (coeff * (max_y + 1) * (max_y + 1) <= four_N) max_y++;
    while (coeff * max_y * max_y > four_N) max_y--;

    ll ans = 0;

    for (ll y = -max_y; y <= max_y; y++) {
        ll disc = four_N - coeff * y * y;
        if (disc < 0) continue;
        ll sd = isqrt_ll(disc);

        /* u ranges from -sd to sd, u must have same parity as y */
        int p_y = (int)((y % 2 + 2) % 2);  /* 0 or 1 */

        ll u_lo = -sd;
        int p_lo = (int)((u_lo % 2 + 2) % 2);
        if (p_lo != p_y) u_lo++;

        ll u_hi = sd;
        int p_hi = (int)((u_hi % 2 + 2) % 2);
        if (p_hi != p_y) u_hi--;

        if (u_lo > u_hi) continue;
        ans += (u_hi - u_lo) / 2 + 1;
    }

    ans--;  /* subtract (0,0) */
    printf("%lld\n", ans);
    return 0;
}
