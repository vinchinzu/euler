/*
 * Project Euler 282 - The Ackermann Function
 *
 * Compute sum_{n=0}^{6} A(n,n) mod 14^8.
 * A(n,n) for n<=3: brute force.
 * A(4,4) = 2^^7 - 3.
 * A(n,n) for n>=5 = 2^^(huge) - 3 (converges mod M via iterated totient).
 */
#include <stdio.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 i128;

static ll pow_mod(ll base, ll exp, ll mod) {
    if (mod == 1) return 0;
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1)
            result = (i128)result * base % mod;
        exp >>= 1;
        base = (i128)base * base % mod;
    }
    return result;
}

static ll totient(ll n) {
    ll result = n;
    for (ll p = 2; p * p <= n; p++) {
        if (n % p == 0) {
            while (n % p == 0) n /= p;
            result -= result / p;
        }
    }
    if (n > 1) result -= result / n;
    return result;
}

/* Compute 2^^height mod m (tower of 2s of given height) */
static ll tower2(ll height, ll m) {
    if (m == 1) return 0;
    if (m == 2) return 0;
    if (height == 0) return 1 % m;
    ll phi_m = totient(m);
    ll exp = tower2(height - 1, phi_m);
    return pow_mod(2, exp, m);
}

/* Brute force Ackermann for small values (iterative with stack) */
static ll ackermann_brute(int m, int n) {
    /* Use iterative approach with explicit stack */
    /* For small m,n this terminates quickly */
    if (m == 0) return n + 1;
    if (m == 1) return n + 2;
    if (m == 2) return 2 * n + 3;
    if (m == 3) return (1LL << (n + 3)) - 3;
    /* Should not reach here for our use case */
    return -1;
}

int main(void) {
    ll M = 1;
    for (int i = 0; i < 8; i++) M *= 14; /* 14^8 */

    ll ans = 0;

    /* A(0,0) = 1 */
    ans = (ans + 1) % M;
    /* A(1,1) = 3 */
    ans = (ans + 3) % M;
    /* A(2,2) = 7 */
    ans = (ans + 7) % M;
    /* A(3,3) = 2^6 - 3 = 61 */
    ans = (ans + 61) % M;

    /* A(4,4) = 2^^7 - 3 */
    ll a44 = (tower2(7, M) - 3 + M) % M;
    ans = (ans + a44) % M;

    /* A(5,5) = 2^^(huge) - 3; converges for tower height >> log_2(M) */
    /* Use height 2^63 which is way more than enough */
    ll a55 = (tower2((ll)1e18, M) - 3 + M) % M;
    ans = (ans + a55) % M;

    /* A(6,6) = same tower convergence */
    ll a66 = (tower2((ll)1e18, M) - 3 + M) % M;
    ans = (ans + a66) % M;

    printf("%lld\n", ans);
    return 0;
}
