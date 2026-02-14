/*
 * Project Euler Problem 301: Nim
 *
 * Count n <= 2^30 where n XOR 2n XOR 3n == 0.
 * These are exactly numbers with no consecutive 1-bits in binary.
 * Count = Fibonacci(32) with F(1)=1, F(2)=2 => F(31) = 2178309.
 */
#include <stdio.h>

int main(void) {
    long long a = 1, b = 2;
    for (int i = 0; i < 29; i++) {
        long long t = a + b;
        a = b;
        b = t;
    }
    printf("%lld\n", b);
    return 0;
}
