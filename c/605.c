/*
 * Project Euler Problem 605: Pairwise Coin-Tossing Game
 *
 * Probability that player K wins = ((K-1)(2^N - 1) + N) * 2^(N-K) / (2^N - 1)^2
 * Answer = numerator * denominator mod M
 */
#include <stdio.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 i128;

#define M 100000000LL

ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (i128)result * base % mod;
        base = (i128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    ll N = 100000007LL;
    ll K = 10007LL;

    ll two_n = pow_mod(2, N, M);
    ll two_n_m1 = (two_n - 1 + M) % M;
    ll num = (i128)((K - 1) % M) * two_n_m1 % M;
    num = (num + N % M) % M;
    num = (i128)num * pow_mod(2, N - K, M) % M;
    ll den = (i128)two_n_m1 * two_n_m1 % M;
    ll ans = (i128)num * den % M;

    printf("%lld\n", ans);
    return 0;
}
