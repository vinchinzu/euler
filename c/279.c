/*
 * Project Euler 279: Triangles with integral sides and integral angle
 *
 * By Niven's theorem, only 60, 90, and 120 degree angles have rational cosine.
 * Enumerate primitive triangles for each family, sum floor(N/p) for each
 * primitive perimeter p.
 */
#include <stdio.h>
#include <math.h>

static int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

int main(void) {
    long long N = 100000000LL; /* 10^8 */

    long long ans = N / 3; /* equilateral triangles */

    int m_limit = (int)sqrt((double)(N / 2)) + 2;

    /* 60-degree Type I: perimeter = 2m^2 + 2n^2 + 5mn */
    for (int m = 2; m <= m_limit; m++) {
        long long mm2 = 2LL * m * m;
        long long m5 = 5LL * m;
        if (mm2 + m5 + 2 > N) break;
        int bad3 = m % 3;
        for (int n = 1; n < m; n++) {
            long long p = mm2 + 2LL * n * n + m5 * n;
            if (p > N) break;
            if (n % 3 != bad3 && gcd(m, n) == 1)
                ans += N / p;
        }
    }

    /* 60-degree Type II: perimeter = 3m(m+n) */
    for (int m = 2; m <= m_limit; m++) {
        long long m3 = 3LL * m;
        if (m3 * (m + 1) > N) break;
        int bad3 = m % 3;
        for (int n = 1; n < m; n++) {
            long long p = m3 * (m + n);
            if (p > N) break;
            if (n % 3 != bad3 && gcd(m, n) == 1)
                ans += N / p;
        }
    }

    /* 120-degree: perimeter = 2m^2 + n^2 + 3mn */
    for (int m = 2; m <= m_limit; m++) {
        long long mm2 = 2LL * m * m;
        long long m3 = 3LL * m;
        if (mm2 + m3 + 1 > N) break;
        int bad3 = m % 3;
        for (int n = 1; n < m; n++) {
            long long p = mm2 + (long long)n * n + m3 * n;
            if (p > N) break;
            if (n % 3 != bad3 && gcd(m, n) == 1)
                ans += N / p;
        }
    }

    /* 90-degree (Pythagorean): perimeter = 2m(m+n), (m-n) odd */
    for (int m = 2; m <= m_limit; m++) {
        long long m2 = 2LL * m;
        if (m2 * (m + 1) > N) break;
        int n_start = (m & 1) ? 2 : 1;
        for (int n = n_start; n < m; n += 2) {
            long long p = m2 * (m + n);
            if (p > N) break;
            if (gcd(m, n) == 1)
                ans += N / p;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
