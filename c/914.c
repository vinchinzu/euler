/* Project Euler Problem 914 - Pythagorean Triangles in Circle
 * For a given R, find largest inradius of primitive Pythagorean triangles
 * fitting inside (without touching) a circle of radius R.
 * Inradius = n*(m-n) for triple parametrized by (m,n).
 * Circumradius = c/2 = (m^2+n^2)/2, must be < R.
 */
#include <stdio.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;

ll gcd(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

ll isqrt_ll(ll n) {
    if (n <= 0) return 0;
    ll x = (ll)sqrt((double)n);
    while (x * x > n) x--;
    while ((x+1)*(x+1) <= n) x++;
    return x;
}

int main(void) {
    ll R = 1000000000000000000LL; /* 10^18 */
    ll limit = 2 * R;
    double sqrt_r = sqrt((double)R);

    double ratio_n = sqrt(1.0 - sqrt(2.0) / 2.0);
    ll n_center = (ll)(ratio_n * sqrt_r + 0.5);

    ll window = (ll)(sqrt_r / 1000.0);
    if (window < 1000) window = 1000;
    ll max_window = (ll)sqrt_r + 5;
    if (max_window < 1000) max_window = 1000;

    ll best = 0;
    ll initial_min = 1, initial_max = 1;

    while (1) {
        ll n_min = n_center - window;
        if (n_min < 1) n_min = 1;
        ll n_max = n_center + window;

        for (ll n = n_min; n <= n_max; n++) {
            ll t = limit - n * n - 1;
            if (t <= 0) continue;

            ll m_max = isqrt_ll(t);
            if (m_max <= n) continue;

            ll m = m_max;
            if ((m - n) % 2 == 0) m--;

            while (m > n) {
                if (m * m + n * n >= limit) break;
                if (gcd(n, m) == 1) {
                    ll val = n * (m - n);
                    if (val > best) best = val;
                    break;
                }
                m -= 2;
            }
        }

        initial_min = n_min;
        initial_max = n_max;

        if (best > 0 || window >= max_window) break;
        window *= 2;
    }

    /* Expand search downward */
    for (ll n = initial_min - 1; n > 0; n--) {
        ll t = limit - n * n;
        if (t <= 0) break;
        double r_upper = n * (sqrt((double)t) - n);
        if (r_upper <= best + 1) break;

        ll t_adj = t - 1;
        if (t_adj <= 0) continue;
        ll m_max = isqrt_ll(t_adj);
        if (m_max <= n) continue;

        ll m = m_max;
        if ((m - n) % 2 == 0) m--;

        while (m > n) {
            if (m * m + n * n >= limit) break;
            if (gcd(n, m) == 1) {
                ll val = n * (m - n);
                if (val > best) best = val;
                break;
            }
            m -= 2;
        }
    }

    /* Expand search upward */
    for (ll n = initial_max + 1; ; n++) {
        ll t = limit - n * n;
        if (t <= 0) break;
        double r_upper = n * (sqrt((double)t) - n);
        if (r_upper <= best + 1) break;

        ll t_adj = t - 1;
        if (t_adj <= 0) continue;
        ll m_max = isqrt_ll(t_adj);
        if (m_max <= n) continue;

        ll m = m_max;
        if ((m - n) % 2 == 0) m--;

        while (m > n) {
            if (m * m + n * n >= limit) break;
            if (gcd(n, m) == 1) {
                ll val = n * (m - n);
                if (val > best) best = val;
                break;
            }
            m -= 2;
        }
    }

    printf("%lld\n", best);
    return 0;
}
