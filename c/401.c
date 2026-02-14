#include <stdio.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

/*
 * sum_sq_mod(n, M): compute sum_{k=1}^n k^2 mod M
 * = n*(n+1)*(2n+1)/6 mod M
 * Since n*(n+1)*(2n+1) can overflow __int128 for large n,
 * we compute n*(n+1)*(2n+1) mod (6*M), then divide by 6.
 * 6*M = 6*10^9 fits in long long.
 */
ll sum_sq_mod(ll n, ll M) {
    ll M6 = 6 * M;
    /* n * (n+1) * (2n+1) mod M6 */
    /* Each factor < 10^15, product < 10^45, but we reduce after each mult */
    lll val = (lll)(n % M6);
    val = val * ((n + 1) % M6) % M6;
    val = val * ((2 * n + 1) % M6) % M6;
    return (ll)(val / 6);
}

int main() {
    ll N = 1000000000000000LL;  /* 10^15 */
    ll M = 1000000000LL;        /* 10^9 */
    ll L = (ll)sqrt((double)N);
    while ((L+1)*(L+1) <= N) L++;
    while (L*L > N) L--;

    ll ans = 0;

    /* Part 1: d from 1 to L */
    for (ll d = 1; d <= L; d++) {
        ll q = N / d;
        ll dmod = d % M;
        ll d2mod = (dmod * dmod) % M;
        ll qmod = q % M;
        ans = (ans + qmod * d2mod) % M;
    }

    /* Part 2: group by t = floor(N/d) for d > L */
    ll tmax = N / (L + 1);
    for (ll t = 1; t <= tmax; t++) {
        ll d_hi = N / t;
        ll d_lo = N / (t + 1) + 1;
        if (d_lo <= L) d_lo = L + 1;
        if (d_lo > d_hi) continue;

        ll s = (sum_sq_mod(d_hi, M) - sum_sq_mod(d_lo - 1, M) + M) % M;
        ans = (ans + (t % M) * s % M) % M;
    }

    if (ans < 0) ans += M;
    printf("%lld\n", ans);
    return 0;
}
