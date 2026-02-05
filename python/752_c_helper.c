/*
 * C helper for Project Euler Problem 752
 * Computes g(x) efficiently using iteration
 */

#include <stdio.h>
#include <stdlib.h>

/*
 * Compute g(x): the smallest n such that (1+√7)^n ≡ 1 (mod x)
 * Returns 0 if no such n exists within the limit
 */
long long compute_g(long long x, long long limit) {
    long long alpha = 1;
    long long beta = 0;
    long long new_alpha, new_beta;
    long long n;

    for (n = 1; n < limit; n++) {
        /* (α + β√7)(1 + √7) = α + 7β + (α + β)√7 */
        new_alpha = (alpha + 7 * beta) % x;
        new_beta = (alpha + beta) % x;

        alpha = new_alpha;
        beta = new_beta;

        if (alpha == 1 && beta == 0) {
            return n;
        }
    }

    return 0;
}

int main(int argc, char *argv[]) {
    if (argc != 3) {
        fprintf(stderr, "Usage: %s <x> <limit>\n", argv[0]);
        return 1;
    }

    long long x = atoll(argv[1]);
    long long limit = atoll(argv[2]);

    long long result = compute_g(x, limit);
    printf("%lld\n", result);

    return 0;
}
