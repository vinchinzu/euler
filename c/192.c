/* Project Euler 192: Best Approximations.
   For each non-square n in [2, 100000], find the best rational approximation
   to sqrt(n) with denominator <= 10^12. Sum the denominators.
   Uses continued fraction expansion and semiconvergent comparison. */
#include <stdio.h>
#include <math.h>

/* We need 128-bit integers for the cross-multiplication comparisons */
typedef __int128 i128;
typedef unsigned __int128 u128;

static long long isqrt_ll(long long n) {
    long long r = (long long)sqrt((double)n);
    while (r * r > n) r--;
    while ((r + 1) * (r + 1) <= n) r++;
    return r;
}

int main(void) {
    int N = 100000;
    long long K = 1000000000000LL;
    long long total = 0;

    for (int n = 2; n <= N; n++) {
        long long a0 = isqrt_ll(n);
        if (a0 * a0 == n) continue;

        /* CF expansion of sqrt(n) */
        long long m = 0, d = 1, a = a0;
        long long A_prev = 1, B_prev = 0;
        long long A_curr = a0, B_curr = 1;

        while (1) {
            m = d * a - m;
            d = (n - m * m) / d;
            a = (a0 + m) / d;

            long long A_next = a * A_curr + A_prev;
            long long B_next = a * B_curr + B_prev;

            if (B_next > K) {
                /* Last convergent with B <= K is (A_curr, B_curr) */
                long long h = (K - B_prev) / B_curr;
                long long den1 = B_curr;
                long long den2 = B_prev + h * B_curr;

                if (h > a / 2) {
                    total += den2;
                } else if (h < (a + 1) / 2) {
                    total += den1;
                } else {
                    /* Tie-breaking comparison */
                    long long num1 = A_curr;
                    long long num2 = A_prev + h * A_curr;

                    i128 cross1 = (i128)num1 * den2;
                    i128 cross2 = (i128)num2 * den1;
                    i128 bot = (i128)den1 * den2;

                    i128 c1sq = cross1 * cross1;
                    i128 c2sq = cross2 * cross2;
                    i128 diff_sq = (c1sq - c2sq);
                    diff_sq = diff_sq * diff_sq;

                    i128 cdiff = cross1 - cross2;
                    i128 rhs = 4 * (i128)n * cdiff * cdiff * bot * bot;

                    int cond1 = diff_sq > rhs;
                    int cond2 = cross1 > cross2;
                    if (cond1 ^ cond2) {
                        total += den1;
                    } else {
                        total += den2;
                    }
                }
                break;
            }

            A_prev = A_curr; A_curr = A_next;
            B_prev = B_curr; B_curr = B_next;
        }
    }

    printf("%lld\n", total);
    return 0;
}
