/* Project Euler Problem 136: Singleton difference.
 *
 * Count n < 50,000,000 with exactly 1 solution to x^2 - y^2 - z^2 = n.
 *
 * x = a+d, y = a, z = a-d with a > d > 0.
 * n = a(4d-a). Let u = a, m = 4d-a, then n = u*m.
 * d = (u+m)/4, need (u+m) % 4 == 0, d >= 1 (u+m >= 4), a > d (u > (u+m)/4 => 3u > m),
 * a < 4d (u < u+m => m > 0), and m <= 3u-4 (from a <= 4d-1: u <= (u+m)-1 => m >= 1,
 * and d >= 1: (u+m)/4 >= 1 => u+m >= 4).
 * Also a >= d+1 means u >= (u+m)/4 + 1, so 4u >= u+m+4, so 3u >= m+4, so m <= 3u-4.
 *
 * So iterate over u, and for each u find valid m values with (u+m)%4==0.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LIMIT_N 50000000

int main(void) {
    unsigned char *counts = calloc(LIMIT_N, sizeof(unsigned char));
    if (!counts) return 1;

    for (int u = 1; u < LIMIT_N; u++) {
        /* m constraints: m >= 1, m <= 3u-4, (u+m) % 4 == 0, u*m < LIMIT_N */
        int max_m_div = (LIMIT_N - 1) / u;  /* u*m < LIMIT_N => m <= (LIMIT_N-1)/u */
        int max_m_cond = 3 * u - 4;
        if (max_m_cond < 1) continue;
        int max_m = max_m_div < max_m_cond ? max_m_div : max_m_cond;
        if (max_m < 1) continue;

        /* Starting m: smallest m >= 1 with (u+m) % 4 == 0 */
        int rem = u % 4;
        int first_m = (4 - rem) % 4;
        if (first_m == 0) first_m = 4;
        if (first_m < 1) first_m += 4;

        for (int m = first_m; m <= max_m; m += 4) {
            long long n = (long long)u * m;
            if (n >= LIMIT_N) break;
            if (counts[n] < 2) counts[n]++;
        }
    }

    int result = 0;
    for (int i = 1; i < LIMIT_N; i++) {
        if (counts[i] == 1) result++;
    }

    printf("%d\n", result);
    free(counts);
    return 0;
}
