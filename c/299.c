/*
 * Project Euler Problem 299: Three similar triangles
 *
 * Find the number of integer triplets satisfying the similarity conditions.
 * Uses two cases based on Pythagorean-like parametrization.
 */
#include <stdio.h>
#include <math.h>

static long long gcd(long long a, long long b) {
    while (b) { long long t = b; b = a % b; a = t; }
    return a;
}

static long long f1(long long m, long long n) {
    return m * m - n * n + 2 * m * n;
}

static long long f2(long long m, long long n) {
    return 2 * (m * m + n * n);
}

int main(void) {
    long long N = 100000000LL; /* 10^8 */
    long long ans = 0;

    /* Case 1: ABP ≡ DBP */
    for (long long n = 1; f1(n, n) < N; n++) {
        for (long long m = n + 1; f1(m, n) < N; m += 2) {
            if (gcd(m, n) == 1) {
                ans += ((N - 1) / f1(m, n)) * 2;
            }
        }
    }

    /* Case 2: ABP ≡ BDP */
    for (long long n = 1; f2(n, n) < N; n++) {
        for (long long m = n + 1; f2(m, n) < N; m += 2) {
            if (gcd(m, n) == 1) {
                ans += (N - 1) / f2(m, n);
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
