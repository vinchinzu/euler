/* Project Euler Problem 139: Pythagorean tiles.
 *
 * Count Pythagorean triples (a,b,c) with perimeter < 100,000,000
 * where c % |a-b| == 0.
 * Generate primitive triples via (m,k) parametrization, count multiples.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define PERIMETER_LIMIT 100000000

static int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

int main(void) {
    long long total = 0;
    int m_limit = (int)sqrt((double)PERIMETER_LIMIT / 2.0) + 1;

    for (int m = 2; m <= m_limit; m++) {
        for (int k = 1; k < m; k++) {
            if ((m - k) % 2 == 0) continue;
            if (gcd(m, k) != 1) continue;

            int a0 = m * m - k * k;
            int b0 = 2 * m * k;
            int c0 = m * m + k * k;
            int p0 = a0 + b0 + c0;

            if (p0 >= PERIMETER_LIMIT) break;

            int diff = abs(a0 - b0);
            if (diff > 0 && c0 % diff == 0) {
                /* Count all multiples d*p0 < PERIMETER_LIMIT */
                total += (PERIMETER_LIMIT - 1) / p0;
            }
        }
    }

    printf("%lld\n", total);
    return 0;
}
