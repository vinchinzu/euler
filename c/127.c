/* Project Euler Problem 127: abc-hits.
 *
 * Find sum of c for all abc-hits below 120000.
 * An abc-hit: a + b = c, gcd(a,b) = 1, rad(a)*rad(b)*rad(c) < c.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#define LIMIT 120000

static int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

typedef struct {
    int rad;
    int num;
} RadPair;

int cmp_radpair(const void *a, const void *b) {
    const RadPair *pa = (const RadPair *)a;
    const RadPair *pb = (const RadPair *)b;
    if (pa->rad != pb->rad) return pa->rad - pb->rad;
    return pa->num - pb->num;
}

int main(void) {
    int *rad = malloc(LIMIT * sizeof(int));
    for (int i = 0; i < LIMIT; i++) rad[i] = 1;

    /* Sieve for primes and build radicals */
    bool *is_prime = calloc(LIMIT, sizeof(bool));
    for (int i = 2; i < LIMIT; i++) is_prime[i] = true;
    for (int i = 2; (long long)i * i < LIMIT; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j < LIMIT; j += i)
                is_prime[j] = false;
        }
    }
    for (int p = 2; p < LIMIT; p++) {
        if (is_prime[p]) {
            for (int m = p; m < LIMIT; m += p)
                rad[m] *= p;
        }
    }
    free(is_prime);

    /* Sort (rad, num) pairs for numbers 1..LIMIT-1 */
    int count = LIMIT - 1;
    RadPair *pairs = malloc(count * sizeof(RadPair));
    for (int n = 1; n < LIMIT; n++) {
        pairs[n - 1].rad = rad[n];
        pairs[n - 1].num = n;
    }
    qsort(pairs, count, sizeof(RadPair), cmp_radpair);

    /* Extract sorted rads for binary search */
    int *sorted_rads = malloc(count * sizeof(int));
    int *sorted_nums = malloc(count * sizeof(int));
    for (int i = 0; i < count; i++) {
        sorted_rads[i] = pairs[i].rad;
        sorted_nums[i] = pairs[i].num;
    }
    free(pairs);

    long long sum_c = 0;

    for (int c = 3; c < LIMIT; c++) {
        int rad_c = rad[c];
        if (rad_c == c) continue; /* c is squarefree with rad(c)=c, skip */

        int max_rad_a = (c - 1) / rad_c;
        if (max_rad_a == 0) continue;

        /* Binary search: upper bound for max_rad_a in sorted_rads */
        int lo = 0, hi = count;
        while (lo < hi) {
            int mid = (lo + hi) / 2;
            if (sorted_rads[mid] <= max_rad_a) lo = mid + 1;
            else hi = mid;
        }
        int limit_idx = lo;
        int limit_a = c / 2;

        for (int i = 0; i < limit_idx; i++) {
            int a = sorted_nums[i];
            if (a >= limit_a || a >= c) continue;
            if (gcd(a, c) != 1) continue;

            int b = c - a;
            if (a >= b) continue;
            if ((long long)rad[a] * rad[b] * rad_c >= c) continue;

            sum_c += c;
        }
    }

    printf("%lld\n", sum_c);

    free(rad);
    free(sorted_rads);
    free(sorted_nums);
    return 0;
}
