/*
 * Project Euler Problem 335 - Gathering the beans
 *
 * M(2^k + 1) = 4^k + 2*2^k - 3^k
 * Find sum_{k=0}^{10^18} M(2^k + 1) mod 7^9.
 *
 * Sum = (4^{N+1}-1)/3 + 2*(2^{N+1}-1) - (3^{N+1}-1)/2  mod 7^9
 * where N = 10^18.
 *
 * Division by 3 and 2 done via modular inverse since gcd(3, 7^9) = 1 and gcd(2, 7^9) = 1.
 */
#include <stdio.h>

typedef long long ll;
typedef __int128 i128;

ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1)
            result = (i128)result * base % mod;
        base = (i128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    ll N = 1000000000000000000LL; /* 10^18 */

    /* M = 7^9 */
    ll M = 1;
    for (int i = 0; i < 9; i++) M *= 7;

    /* inv3 = modular inverse of 3 mod M */
    /* inv2 = modular inverse of 2 mod M */
    ll inv3 = mod_pow(3, M - M / 7 - 1, M); /* Euler's theorem: phi(7^9) = 7^9 - 7^8 */
    ll inv2 = mod_pow(2, M - M / 7 - 1, M);

    /* s1 = (4^{N+1} - 1) / 3 mod M */
    ll s1 = (i128)(mod_pow(4, N + 1, M) - 1 + M) % M * inv3 % M;

    /* s2 = 2 * (2^{N+1} - 1) mod M */
    ll s2 = 2 * ((mod_pow(2, N + 1, M) - 1 + M) % M) % M;

    /* s3 = (3^{N+1} - 1) / 2 mod M */
    ll s3 = (i128)(mod_pow(3, N + 1, M) - 1 + M) % M * inv2 % M;

    ll ans = ((s1 + s2 - s3) % M + M) % M;
    printf("%lld\n", ans);
    return 0;
}
