/*
 * Project Euler Problem 542: Geometric Progression.
 * Let S(n) be the maximum sum of a geometric progression with at least 3
 * distinct terms <= n. Find sum_{k=4}^N (-1)^k S(k).
 *
 * For a ratio (k-1)/k with e+1 terms and max coefficient r = floor(n/k^e),
 * S = r * (k^{e+1} - (k-1)^{e+1}).
 *
 * Uses divide-and-conquer: when S(low)==S(high), the alternating sum over
 * the range is easy to compute.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef long long ll;
typedef __int128 i128;

static ll ipow(ll base, int exp) {
    ll result = 1;
    for (int i = 0; i < exp; i++) {
        result *= base;
        if (result < 0) return result; /* overflow sentinel */
    }
    return result;
}

static int ilog2(ll n) {
    if (n <= 0) return 0;
    int r = 0;
    while (n > 1) { n >>= 1; r++; }
    return r;
}

static ll S(ll n) {
    if (n < 3) return 0;

    ll max_S = 0;
    int max_e = ilog2(n);

    for (int e = max_e; e >= 2; e--) {
        if ((ll)(e + 1) * n < max_S) break;
        for (ll k = 2; ; k++) {
            ll ke = ipow(k, e);
            if (ke > 2 * n || ke < 0) break;
            ll r = n / ke;
            if (r > 0) {
                ll sum_val = (ipow(k, e + 1) - ipow(k - 1, e + 1)) * r;
                if (sum_val > max_S) max_S = sum_val;
            }
        }
    }

    return max_S;
}

static ll parity(ll n) {
    return (n % 2 == 0) ? 1 : -1;
}

static ll T(ll low, ll high) {
    if (low + 1 == high) {
        return ((low + high) % 2 == 0) ? 0 : parity(low) * S(low);
    }

    ll s_low = S(low);
    ll s_high = S(high);

    if (s_low == s_high) {
        ll count = high - low;
        if (count % 2 == 0) return 0;
        return parity(low) * s_low;
    }

    ll mid = (low + high) / 2;
    return T(low, mid) + T(mid, high);
}

int main(void) {
    ll N = 100000000000000000LL; /* 10^17 */
    printf("%lld\n", T(4, N + 1));
    return 0;
}
