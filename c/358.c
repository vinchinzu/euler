/* Project Euler 358 - Cyclic Numbers
 *
 * Find the cyclic number with first 11 digits "00000000137" and last 5 digits "56789".
 * Return the sum of all its digits.
 *
 * A cyclic number for prime p is (10^(p-1) - 1) / p = repeating decimal of 1/p.
 * Constraints narrow p to a small range, then verify via long division.
 */

#include <stdio.h>

typedef long long ll;
typedef __int128 i128;

int is_prime(ll n) {
    if (n < 2) return 0;
    if (n == 2) return 1;
    if (n % 2 == 0) return 0;
    if (n < 9) return 1;
    if (n % 3 == 0) return 0;

    /* Miller-Rabin with sufficient witnesses */
    ll d = n - 1;
    int r = 0;
    while (d % 2 == 0) { r++; d /= 2; }

    int witnesses[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37};
    for (int wi = 0; wi < 12; wi++) {
        ll a = witnesses[wi];
        if (a >= n) continue;

        /* Compute a^d mod n */
        ll x = 1;
        ll base = a % n;
        ll exp = d;
        while (exp > 0) {
            if (exp & 1) x = (i128)x * base % n;
            base = (i128)base * base % n;
            exp >>= 1;
        }

        if (x == 1 || x == n - 1) continue;
        int found = 0;
        for (int j = 0; j < r - 1; j++) {
            x = (i128)x * x % n;
            if (x == n - 1) { found = 1; break; }
        }
        if (!found) return 0;
    }
    return 1;
}

ll extended_gcd(ll a, ll b, ll *x, ll *y) {
    if (a == 0) { *x = 0; *y = 1; return b; }
    ll x1, y1;
    ll g = extended_gcd(b % a, a, &x1, &y1);
    *x = y1 - (b / a) * x1;
    *y = x1;
    return g;
}

ll mod_inverse(ll a, ll m) {
    ll x, y;
    extended_gcd(a, m, &x, &y);
    return ((x % m) + m) % m;
}

int main(void) {
    /* Range from first-digits constraint: 1/p starts with 0.00000000137 */
    ll lower = 100000000000LL / 138 + 1;  /* p > 10^11/138 */
    ll upper = 100000000000LL / 137;       /* p < 10^11/137 */

    /* Last digits constraint: 56789 * p + 1 = 0 (mod 10^5) */
    ll inv = mod_inverse(56789, 100000);
    ll target_remainder = (99999 * inv) % 100000;

    ll start = lower + (target_remainder - lower % 100000 + 100000) % 100000;

    for (ll p = start; p <= upper; p += 100000) {
        if (!is_prime(p)) continue;

        /* Verify first digits more precisely */
        ll first_digits = 10000000000000LL / p;
        if (first_digits < 13700 || first_digits > 13799) continue;

        /* Verify last digits */
        if ((56789LL * p + 1) % 100000 != 0) continue;

        /* Compute digit sum via long division and verify full reptend */
        ll digit_sum = 0;
        ll n = 1;
        int full_reptend = 1;

        for (ll i = 1; i < p; i++) {
            n *= 10;
            ll digit = n / p;
            digit_sum += digit;
            n = n % p;

            if (n == 1 && i < p - 1) {
                full_reptend = 0;
                break;
            }
        }

        if (!full_reptend) continue;
        if (n != 1) continue;

        printf("%lld\n", digit_sum);
        return 0;
    }

    return 1;
}
