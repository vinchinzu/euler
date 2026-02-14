/* Project Euler Problem 129: Repunit divisibility.
 *
 * Find the least n with gcd(n,10)=1 such that A(n) > 1,000,000.
 * A(n) = least k where R(k) is divisible by n.
 * Use the simple approach: compute A(n) by iterating remainders.
 */
#include <stdio.h>
#include <stdbool.h>

#define TARGET 1000000

/* Compute A(n): least k such that R(k) % n == 0 */
static int repunit_order(int n) {
    int remainder = 1 % n;
    int k = 1;
    while (remainder != 0) {
        remainder = (remainder * 10 + 1) % n;
        k++;
    }
    return k;
}

int main(void) {
    /* A(n) <= n, so we need n > 1,000,000.
     * Start searching from 1,000,001 upward, skipping even and multiples of 5. */
    for (int n = TARGET + 1; ; n++) {
        if (n % 2 == 0 || n % 5 == 0) continue;
        int a = repunit_order(n);
        if (a > TARGET) {
            printf("%d\n", n);
            return 0;
        }
    }
    return 1;
}
