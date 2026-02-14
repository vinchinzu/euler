/* Project Euler 195: Inscribed circles of 60-degree triangles. */
#include <stdio.h>
#include <math.h>

static int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

int main(void) {
    long long N = 1053779;
    double sqrt3 = sqrt(3.0);
    double inv_2sqrt3 = 1.0 / (2.0 * sqrt3);
    double limit_3N = 3.0 * N;

    long long ans = 0;

    for (int n = 1; ; n++) {
        int m_start = 2 * n + 1;
        double ir_start = (double)(m_start + n) * (m_start - 2 * n) * inv_2sqrt3;
        if (ir_start > limit_3N) break;

        for (int m = m_start; ; m++) {
            double ir = (double)(m + n) * (m - 2 * n) * inv_2sqrt3;
            if (ir > limit_3N) break;
            if (gcd(m, n) == 1) {
                if ((m + n) % 3 == 0) {
                    ans += (long long)(N / (ir / 3.0));
                } else {
                    ans += (long long)(N / ir);
                }
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
