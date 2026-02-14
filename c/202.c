/*
 * Project Euler 202: Laserbeam
 *
 * Three mirrors in equilateral triangle. Find number of ways a laser beam
 * can enter a vertex, bounce exactly 12017639147 times, then exit through
 * the same vertex.
 *
 * Uses inclusion-exclusion over prime factors of target.
 */
#include <stdio.h>

int main(void) {
    long long N = 12017639147LL;
    long long target = (N + 3) / 2;

    /* Factor target */
    long long primes[64];
    int np = 0;
    {
        long long t = target;
        for (long long d = 2; d * d <= t; d++) {
            if (t % d == 0) {
                primes[np++] = d;
                while (t % d == 0) t /= d;
            }
        }
        if (t > 1) primes[np++] = t;
    }

    long long ans = 0;
    for (int subset = 0; subset < (1 << np); subset++) {
        long long prod = 1;
        int bits = 0;
        for (int i = 0; i < np; i++) {
            if (subset & (1 << i)) {
                prod *= primes[i];
                bits++;
            }
        }
        long long contrib = (target / prod - 2) / 3;
        if (bits % 2 == 0)
            ans += contrib;
        else
            ans -= contrib;
    }

    printf("%lld\n", ans);
    return 0;
}
