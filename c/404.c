#include <stdio.h>
#include <math.h>

typedef long long ll;

ll gcd_func(ll a, ll b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

int main() {
    ll N = 100000000000000000LL;  /* 10^17 */
    ll ans = 0;

    /* First case: m <= 2n, with m > n, m-n odd, gcd(m,n)=1 */
    /* limit: 4*n^4 <= N => n <= (N/4)^(1/4) */
    ll limit1 = (ll)pow((double)N / 4.0, 0.25);
    /* Adjust to be safe */
    while (4*limit1*limit1*limit1*limit1 <= N) limit1++;
    limit1--;

    for (ll n = 1; n <= limit1; n++) {
        if (4*n*n*n*n > N) break;
        /* m from n+1 to 2n, step 2 (m-n odd means same parity step) */
        for (ll m = n + 1; m <= 2 * n; m += 2) {
            if (gcd_func(m, n) != 1) continue;

            ll x = m*m - n*n - 4*m*n;
            if (x < 0) x = -x;
            ll y = 2*(m*m - n*n + m*n);
            /* a_base = x*y/2 */
            ll xy = x * y;
            if (xy == 0) continue;
            ll a_base = xy / 2;
            if (a_base > N || a_base == 0) continue;

            /* Check (x, y) = 1 iff not both divisible by 5 */
            if (x % 5 == 0 && y % 5 == 0) continue;

            ans += N / a_base;
        }
    }

    /* Second case: m >= 3n, with m > n, m-n odd, gcd(m,n)=1 */
    /* limit: 20*n^4 <= N => n <= (N/20)^(1/4) */
    ll limit2 = (ll)pow((double)N / 20.0, 0.25);
    while (20*limit2*limit2*limit2*limit2 <= N) limit2++;
    limit2--;

    for (ll n = 1; n <= limit2; n++) {
        if (20*n*n*n*n > N) break;
        /* m starts at 3n, step depends on parity */
        /* m - n must be odd: if n is even, m must be odd; if n is odd, m must be even */
        /* Actually m and n must have different parity (m-n odd) */
        ll m_start = 3 * n;
        if ((m_start - n) % 2 == 0) m_start++;  /* ensure m-n is odd */

        for (ll m = m_start; ; m += 2) {
            if (gcd_func(m, n) != 1) continue;

            ll x = m*m - n*n + 4*m*n;
            ll y_val = m*m - n*n - m*n;
            if (y_val < 0) y_val = -y_val;
            ll y = 2 * y_val;

            ll xy = x * y;
            if (xy == 0) continue;
            ll a_base = xy / 2;
            if (a_base > N) break;
            if (a_base == 0) continue;

            if (x % 5 == 0 && y % 5 == 0) continue;

            ans += N / a_base;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
