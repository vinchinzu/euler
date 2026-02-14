/*
 * Project Euler 207: Integer Partition Equations
 *
 * 4^t = 2^t + k. Let x = 2^t, then x^2 - x = k, so k = x(x-1) for x >= 2.
 * t = log2(x). A "perfect" partition has integer t, i.e., x is a power of 2.
 * Find smallest k = m(m+1) such that perfect/total < 1/12345.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    double R = 1.0 / 12345.0;
    long long n = 2;
    while ((double)((int)(log2((double)n))) / (double)(n - 1) >= R) {
        n++;
    }
    printf("%lld\n", n * n - n);
    return 0;
}
