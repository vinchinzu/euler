/*
 * Project Euler Problem 809: Rational Recurrence Relation.
 *
 * f(A+1/B) = Ackermann(B-1, A) which is 2^2^...^2 - 3 (tower of 2s).
 * Find fixed point of 2^x mod M, then subtract 3.
 */
#include <stdio.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 i128;

static ll power(ll base, ll exp, ll mod) {
    ll result = 1;
    base = base % mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (i128)result * base % mod;
        base = (i128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    ll M = 1000000000000000LL;  /* 10^15 */

    /* Find fixed point of 2^x mod M */
    ll b = 0;
    while (1) {
        ll next_b = power(2, b, M);
        if (next_b == b) break;
        b = next_b;
    }

    printf("%lld\n", b - 3);
    return 0;
}
