/* Project Euler Problem 138: Special isosceles triangles.
 *
 * Sum of L for the first 12 isosceles triangles where h = b +/- 1.
 * L = F(6j+3) / 2 for j = 1..12, where F is Fibonacci.
 *
 * F(75) is very large (~2.3 * 10^15), fits in long long (max ~9.2 * 10^18).
 * Actually F(75) = 2111485077978050. Sum of 12 such values also fits.
 */
#include <stdio.h>

int main(void) {
    /* Precompute Fibonacci up to F(75) (6*12+3 = 75) */
    long long fib[76];
    fib[0] = 0;
    fib[1] = 1;
    for (int i = 2; i <= 75; i++) {
        fib[i] = fib[i - 1] + fib[i - 2];
    }

    long long total = 0;
    for (int j = 1; j <= 12; j++) {
        int m = 6 * j + 3;
        long long L = fib[m] / 2;
        total += L;
    }

    printf("%lld\n", total);
    return 0;
}
