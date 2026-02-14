/* Project Euler 357 - Prime Generating Integers
 *
 * Sum of all n <= 10^8 such that for every divisor d of n, d + n/d is prime.
 * Key: n+1 must be prime, n must be even (or 1), d=2 check filters most.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define LIMIT 100000001

static char *is_composite;  /* 0 = prime, 1 = composite */

void sieve(void) {
    is_composite = (char*)calloc(LIMIT + 1, 1);
    is_composite[0] = is_composite[1] = 1;
    for (long long i = 2; i * i <= LIMIT; i++) {
        if (!is_composite[i]) {
            for (long long j = i * i; j <= LIMIT; j += i)
                is_composite[j] = 1;
        }
    }
}

int check_divisors(long long n) {
    for (long long d = 2; d * d <= n; d++) {
        if (n % d == 0) {
            long long quotient = n / d;
            if (is_composite[d + quotient])
                return 0;
        }
    }
    return 1;
}

int main(void) {
    sieve();

    long long total = 0;

    /* n=1: 1+1=2 is prime */
    if (!is_composite[2])
        total += 1;

    /* For n > 1, n must be even (n+1 is odd prime) */
    for (long long p = 3; p <= LIMIT; p += 2) {
        if (is_composite[p]) continue;

        long long n = p - 1;
        if (n > 100000000LL) break;

        /* Quick check: d=2, 2 + n/2 must be prime */
        if (is_composite[2 + n / 2]) continue;

        if (check_divisors(n))
            total += n;
    }

    printf("%lld\n", total);
    free(is_composite);
    return 0;
}
