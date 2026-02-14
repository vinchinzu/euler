/*
 * Project Euler 785 - Binary Quadratic Diophantine
 *
 * Find sum of x+y+z for all 1<=x<=y<=z<=N such that
 * 15(x^2+y^2+z^2)=34(xy+yz+xz) and GCD(x,y,z)=1.
 *
 * Parameterization: x=3(m+n)(n-m), y=3m(17m+2n), z=(14m+5n)(4m+n)
 * GCD is 9 if m+n=0(mod 3), else 1.
 */
#include <stdio.h>
#include <math.h>

static int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

int main(void) {
    long long N = 1000000000LL;
    long long ans = 0;

    /* Case 1: m+n != 0 (mod 3) */
    int m_max1 = (int)sqrt((double)N / 95.0) + 1;
    for (int m = 1; m <= m_max1; m++) {
        if (95LL * m * m > N) break;
        for (int n_val = m + 1; ; n_val++) {
            long long z = (long long)(14 * m + 5 * n_val) * (4 * m + n_val);
            if (z > N) break;
            if (m % 19 != n_val % 19 && gcd(m, n_val) == 1 && (m + n_val) % 3 != 0) {
                ans += 8LL * (13LL * m * m + 5LL * m * n_val + (long long)n_val * n_val);
            }
        }
    }

    /* Case 2: m+n = 0 (mod 3) */
    int m_max2 = (int)sqrt(9.0 * N / 95.0) + 1;
    for (int m = 1; m <= m_max2; m++) {
        if (95LL * m * m > 9 * N) break;
        int n_start = m + (m % 3 == 0 ? 3 : (3 - m % 3));
        /* Ensure (m + n_start) % 3 == 0 */
        while ((m + n_start) % 3 != 0) n_start++;
        for (int n_val = n_start; ; n_val += 3) {
            long long z = (long long)(14 * m + 5 * n_val) * (4 * m + n_val);
            if (z > 9 * N) break;
            if (m % 19 != n_val % 19 && gcd(m, n_val) == 1) {
                ans += 8LL * (13LL * m * m + 5LL * m * n_val + (long long)n_val * n_val) / 9;
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
