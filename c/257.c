/*
 * Project Euler Problem 257: Angular Bisectors
 *
 * Count triangles with perimeter <= N=10^8 where the ratio of areas
 * of ABC to AEG (formed by angle bisector intersections) is integral.
 */
#include <stdio.h>
#include <math.h>

typedef long long ll;

static ll gcd(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

static ll perim1(ll m, ll n) { return 2*m*m + n*n + 3*m*n; }
static ll perim2(ll m, ll n) { return 3*m*m + n*n + 4*m*n; }

int main(void) {
    ll N = 100000000LL;
    ll L = (ll)sqrt((double)(N / 3));

    /* Precompute GCDs */
    /* Since L can be up to ~5773, we need gcd table up to L+1 */
    /* But we also iterate up to 2*L in the r=3 odd case */
    /* For the gcd calls: gcd(m, n%m) where m <= 2*L and n <= 3*m */
    /* We'll just use the gcd function directly */

    ll ans = 0;

    /* r = 2 case */
    for (ll m = 1; m <= L; m++) {
        if (perim1(m, m) > N) break;
        for (ll n = m + 1; n < 2 * m; n++) {
            if (perim1(m, n) > N) break;
            if (n % 2 != 0 && gcd(m, n % m) == 1) {
                ans += N / perim1(m, n);
            }
        }
    }

    /* r = 3 case (even) */
    for (ll m = 1; m <= L; m++) {
        if (perim2(m, m) > N) break;
        for (ll n = m + 1; n < 3 * m; n += 2) {
            if (perim2(m, n) > N) break;
            if (n % 3 != 0 && gcd(m, n % m) == 1) {
                ans += N / perim2(m, n);
            }
        }
    }

    /* r = 3 case (odd m, n) */
    for (ll m = 1; m <= 2 * L; m += 2) {
        if (perim2(m, m) > 2 * N) break;
        for (ll n = m + 2; n < 3 * m; n += 2) {
            if (perim2(m, n) > 2 * N) break;
            if (n % 3 != 0 && gcd(m, n % m) == 1) {
                ans += 2 * N / perim2(m, n);
            }
        }
    }

    /* r = 4 case (equilateral triangles) */
    ans += N / 3;

    printf("%lld\n", ans);
    return 0;
}
