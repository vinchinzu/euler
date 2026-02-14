/*
 * Project Euler Problem 601: Divisibility streaks
 *
 * streak(n) = max k such that 1,2,...,k all divide n-1
 * P(s, N) = floor((N-2)/lcm(1..s)) - floor((N-2)/lcm(1..s+1))
 * Answer = sum_{i=1}^{31} P(i, 4^i)
 */
#include <stdio.h>
#include <stdint.h>

typedef unsigned __int128 u128;

static long long gcd(long long a, long long b) {
    while (b) { long long t = b; b = a % b; a = t; }
    return a;
}

static long long lcm(long long a, long long b) {
    return a / gcd(a, b) * b;
}

int main(void) {
    int N = 31;
    long long ans = 0;

    for (int i = 1; i <= N; i++) {
        /* Compute 4^i using __int128 since 4^31 is large */
        u128 four_i = 1;
        for (int j = 0; j < i; j++) four_i *= 4;

        /* Compute lcm(1..i) */
        long long lcm_s = 1;
        for (int j = 1; j <= i; j++)
            lcm_s = lcm(lcm_s, j);

        long long lcm_s1 = lcm(lcm_s, i + 1);

        u128 n_minus_2 = four_i - 2;
        long long count = (long long)(n_minus_2 / (u128)lcm_s)
                        - (long long)(n_minus_2 / (u128)lcm_s1);
        ans += count;
    }

    printf("%lld\n", ans);
    return 0;
}
