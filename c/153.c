/*
 * Project Euler 153 - Investigating Gaussian Integers
 *
 * Sum of all divisors of all n from 1 to 10^8, including Gaussian integer divisors.
 * S1 = sum of sigma_1(k) for k=1..N (real divisors)
 * S2 = 2 * sum over coprime (u,v) with u>=v>=1 of (u+v)*G(N/(u^2+v^2))
 * where G(n) = sum_{k=1}^{n} sigma_1(k) computed via hyperbola method.
 */
#include <stdio.h>
#include <math.h>

typedef long long ll;

static int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

/* G(n) = sum_{k=1}^{n} floor(n/k) * k ... wait, that's sum of divisor sums.
 * Actually G(n) = sum_{i=1}^{n} sigma_1(i) = sum_{k=1}^{n} k * floor(n/k)
 * No. sigma_1(i) = sum of divisors of i. sum_{i=1}^n sigma_1(i) = sum_{d=1}^{n} d * floor(n/d).
 * And sum_{k=1}^{n} floor(n/k) * k uses hyperbola method.
 */
static ll g_function(ll n) {
    if (n <= 0) return 0;
    ll result = 0;
    ll k = 1;
    while (k <= n) {
        ll q = n / k;
        ll next_k = n / q + 1;
        ll last_k = next_k - 1;
        if (last_k > n) last_k = n;
        ll count = last_k - k + 1;
        ll sum_k = count * (k + last_k) / 2;
        result += sum_k * q;
        k = next_k;
    }
    return result;
}

int main(void) {
    ll N = 100000000LL; /* 10^8 */
    int sqrt_limit = (int)sqrt((double)N);

    /* S1 = G(N) */
    ll total_sum = g_function(N);

    ll s2_prime = 0;

    /* Case u = v = 1: d = 2, contribution = 1 * G(N/2) */
    /* Wait: the Python has (u+v) for u>v, but for u=v=1 it adds u*G(N/d) = 1*G(N/2) */
    s2_prime += g_function(N / 2);

    /* Case u > v >= 1, gcd(u,v)=1 */
    for (int u = 2; u <= sqrt_limit; u++) {
        for (int v = 1; v < u; v++) {
            if (gcd(u, v) != 1) continue;
            ll d = (ll)u * u + (ll)v * v;
            if (d > N) break;
            s2_prime += (ll)(u + v) * g_function(N / d);
        }
    }

    printf("%lld\n", total_sum + 2 * s2_prime);
    return 0;
}
