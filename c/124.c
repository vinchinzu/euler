/* Project Euler Problem 124: Ordered radicals.
 *
 * Compute rad(n) for 1..100000 via sieve, sort by (rad, n), return 10000th.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#define LIMIT 100000
#define TARGET_K 10000

typedef struct {
    int rad;
    int n;
} Pair;

int cmp_pair(const void *a, const void *b) {
    const Pair *pa = (const Pair *)a;
    const Pair *pb = (const Pair *)b;
    if (pa->rad != pb->rad) return (pa->rad > pb->rad) - (pa->rad < pb->rad);
    return (pa->n > pb->n) - (pa->n < pb->n);
}

int main(void) {
    int *rad = malloc((LIMIT + 1) * sizeof(int));
    for (int i = 0; i <= LIMIT; i++) rad[i] = 1;

    /* Sieve for smallest prime factors and build radicals */
    bool *is_prime = calloc(LIMIT + 1, sizeof(bool));
    for (int i = 2; i <= LIMIT; i++) is_prime[i] = true;
    for (int i = 2; (long long)i * i <= LIMIT; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= LIMIT; j += i)
                is_prime[j] = false;
        }
    }

    for (int p = 2; p <= LIMIT; p++) {
        if (is_prime[p]) {
            for (int m = p; m <= LIMIT; m += p) {
                rad[m] *= p;
            }
        }
    }

    Pair *pairs = malloc(LIMIT * sizeof(Pair));
    for (int n = 1; n <= LIMIT; n++) {
        pairs[n - 1].rad = rad[n];
        pairs[n - 1].n = n;
    }

    qsort(pairs, LIMIT, sizeof(Pair), cmp_pair);

    printf("%d\n", pairs[TARGET_K - 1].n);

    free(rad);
    free(is_prime);
    free(pairs);
    return 0;
}
