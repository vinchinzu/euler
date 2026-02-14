/*
 * Project Euler 028 - Number Spiral Diagonals
 * Sum of diagonals in a 1001x1001 spiral.
 */
#include <stdio.h>

int main(void) {
    int n = 1001;
    int m = (n - 1) / 2;

    long long sum_k_squared = (long long)m * (m + 1) * (2 * m + 1) / 6;
    long long sum_k = (long long)m * (m + 1) / 2;
    long long sum_constant = 4LL * m;

    long long total = 1 + 16 * sum_k_squared + 4 * sum_k + sum_constant;

    printf("%lld\n", total);
    return 0;
}
