/*
 * Project Euler Problem 403: Lattice points enclosed by parabola and line.
 *
 * For each pair (r, s) with r <= s, a = r+s, b = -r*s, the number of lattice
 * points is L = ((s-r)^3 + 5(s-r)) / 6 + 1.
 * Sum over all valid (r, s) pairs where |a| <= N and |b| <= N.
 */
#include <stdio.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

#define N_VAL 1000000000000LL
#define M_VAL 100000000LL

static ll sum_powers(ll n, int exp, ll mod) {
    if (n <= 0) return 0;
    ll m6 = 6 * mod;
    if (exp == 1) {
        /* n*(n+1)/2 mod mod */
        lll val = (lll)(n % (2 * mod)) * ((n + 1) % (2 * mod));
        return (ll)((val / 2) % mod);
    } else if (exp == 3) {
        /* (n*(n+1)/2)^2 mod mod */
        lll half = (lll)(n % (2 * mod)) * ((n + 1) % (2 * mod));
        ll h = (ll)((half / 2) % mod);
        return (ll)((lll)h * h % mod);
    }
    return 0;
}

int main(void) {
    ll N = N_VAL;
    ll L = (ll)sqrt((double)N);
    while ((L+1)*(L+1) <= N) L++;
    while (L*L > N) L--;
    ll M = M_VAL;

    ll ans = 0;
    for (ll r = -L; r <= L; r++) {
        ll max_s;
        if (r == 0) max_s = N;
        else {
            ll t1 = N / (r < 0 ? -r : r);
            ll t2 = N - r;
            max_s = t1 < t2 ? t1 : t2;
        }
        ll d1 = max_s - r;
        ll d2 = L - r;

        /* Compute 2 * (sum_cubes(d1)/(6M) + 5*sum1(d1)/(6M) + (d1+1))
         *       -     (sum_cubes(d2)/(6M) + 5*sum1(d2)/(6M) + (d2+1)) */
        ll mod6 = 6 * M;

        /* For d1: (sum_cubes + 5*sum1) / 6 + count */
        ll sc1 = sum_powers(d1, 3, mod6);
        ll s1_1 = sum_powers(d1, 1, mod6);
        ll term1 = ((sc1 + 5 * s1_1 % mod6) % mod6) / 6 + (d1 + 1) % M;

        ll sc2 = sum_powers(d2, 3, mod6);
        ll s1_2 = sum_powers(d2, 1, mod6);
        ll term2 = ((sc2 + 5 * s1_2 % mod6) % mod6) / 6 + (d2 + 1) % M;

        ans = (ans + (2 * term1 - term2) % M + M) % M;
    }
    ans = ((ans % M) + M) % M;
    printf("%lld\n", ans);
    return 0;
}
