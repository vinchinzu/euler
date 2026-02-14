/*
 * Project Euler Problem 583: Heron envelopes.
 *
 * Find the sum of perimeters of Heron envelopes with perimeter <= N=10^7.
 * A Heron envelope is a pentagon ABCDE consisting of a rectangle ABDE below
 * an isosceles triangle BCD, with all sides and diagonals integral.
 *
 * For each even a, find all other legs b such that (a, b, c) is Pythagorean,
 * then find valid (b1, b2, b3) combos where 2*b1 + b2 = b3.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>

typedef int64_t i64;

#define LIMIT 10000000

/* Smallest prime factor sieve */
int *ff;

void sieve_spf(int n) {
    ff = (int *)calloc(n + 1, sizeof(int));
    for (int i = 2; i <= n; i++) {
        if (ff[i] == 0) {
            ff[i] = i;
            for (i64 j = (i64)i * i; j <= n; j += i)
                if (ff[j] == 0) ff[j] = i;
        }
    }
}

/* Number of divisors using SPF sieve */
int *num_factors;
void compute_num_factors(int n) {
    num_factors = (int *)calloc(n + 1, sizeof(int));
    num_factors[1] = 1;
    for (int i = 2; i <= n; i++) {
        int ii = i;
        int mult = 1;
        while (ii % ff[i] == 0) {
            ii /= ff[i];
            mult += 2;
        }
        num_factors[i] = num_factors[ii] * mult;
    }
}

int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

int isqrt_i(i64 n) {
    i64 r = (i64)sqrt((double)n);
    while (r * r > n) r--;
    while ((r + 1) * (r + 1) <= n) r++;
    return (int)r;
}

/* Compare function for qsort */
int cmp_int(const void *a, const void *b) {
    int ia = *(const int *)a;
    int ib = *(const int *)b;
    return (ia > ib) - (ia < ib);
}

int main() {
    int N = LIMIT;

    sieve_spf(N);
    compute_num_factors(N);

    /* For each a, store indices into a flat array of "other legs" */
    int *start_indices = (int *)calloc(N + 2, sizeof(int));
    for (int i = 1; i <= N; i++) {
        start_indices[i + 1] = start_indices[i] + num_factors[i] / 2;
    }
    int *end_indices = (int *)malloc((N + 2) * sizeof(int));
    memcpy(end_indices, start_indices, (N + 2) * sizeof(int));
    int total_size = start_indices[N + 1];
    int *other_legs = (int *)calloc(total_size, sizeof(int));

    /* Generate Pythagorean triples using Euclid's formula */
    int m_limit = isqrt_i(2LL * N);
    for (int m = 2; m <= m_limit; m++) {
        for (int n = 1; n < m; n++) {
            if ((m + n) % 2 == 1 && gcd(m, n) == 1) {
                int a0 = m * m - n * n;
                int b0 = 2 * m * n;
                int c0 = m * m + n * n;
                if (c0 > 2 * N) break;

                for (int k = 1; (i64)k * c0 <= 2 * N; k++) {
                    int a = k * a0, b = k * b0;
                    /* Store (a, b) pair: leg a has other_leg b, and vice versa */
                    if (a <= N && end_indices[a] < start_indices[a + 1]) {
                        other_legs[end_indices[a]++] = b;
                    }
                    if (b <= N && end_indices[b] < start_indices[b + 1]) {
                        other_legs[end_indices[b]++] = a;
                    }
                    /* Swap a,b and store */
                    if (b <= N && end_indices[b] < start_indices[b + 1]) {
                        other_legs[end_indices[b]++] = a;
                    }
                    if (a <= N && end_indices[a] < start_indices[a + 1]) {
                        other_legs[end_indices[a]++] = b;
                    }
                }
            }
        }
    }

    /* Wait - the Python code adds BOTH (a,b) and (b,a) for each triple,
     * then ALSO stores both legs for each value. Let me re-examine the Python.
     *
     * The Python generates (a, b, c) and (b, a, c) for each primitive triple*k,
     * then for each triple stores: if a <= N, other_legs[a] gets b.
     * if b <= N, other_legs[b] gets a.
     *
     * So for a primitive triple (a0, b0, c0), it generates:
     * (k*a0, k*b0, k*c0): stores b0*k in other_legs[a0*k], a0*k in other_legs[b0*k]
     * (k*b0, k*a0, k*c0): stores a0*k in other_legs[b0*k], b0*k in other_legs[a0*k]
     * So each leg gets the other leg TWICE.
     *
     * Actually wait, looking at the Python again more carefully, the dedup happens
     * because it's storing in a flat array with exact sizes based on num_factors/2.
     * The num_factors count should already account for the right number of other legs.
     *
     * Let me re-read the Python... The generate_pythagorean_triples function
     * returns BOTH orderings already. Then the main loop stores both legs.
     * So for primitive (3,4,5): triples has (3,4,5) and (4,3,5).
     * For (3,4,5): other_legs[3] gets 4, other_legs[4] gets 3.
     * For (4,3,5): other_legs[4] gets 3, other_legs[3] gets 4.
     * So each leg gets the other leg twice - once from each ordering.
     *
     * But the num_factors[a]/2 should give the right count (number of Pythagorean
     * triples with one leg = a, counting both orderings? No, it's the number of
     * representations of a^2 as sum of two squares, related to divisor function).
     *
     * This is getting complicated. Let me just redo this more carefully.
     */

    /* Reset and redo: the Python generates all (a, b, c) with c <= 2*N
     * using Euclid's formula, generating BOTH (a,b,c) and (b,a,c).
     * Then for each triple, stores BOTH directions: other_legs[a] <- b AND other_legs[b] <- a.
     * This means each pair is stored 4 times total. But the array sizes are based on num_factors/2.
     *
     * Let me just restart with a simpler approach: for each a, collect all b such that
     * a^2 + b^2 = c^2 for some c (i.e., b is a "Pythagorean other leg" for a).
     * Use a direct method.
     */

    /* Actually, I'll re-read the Python more carefully and replicate exactly. */

    /* Reset */
    memcpy(end_indices, start_indices, (N + 2) * sizeof(int));
    memset(other_legs, 0, total_size * sizeof(int));

    /* Generate Pythagorean triples with c <= 2*N */
    for (int m = 2; (i64)m * m <= 2 * N; m++) {
        for (int n = 1; n < m; n++) {
            if ((m + n) % 2 == 1 && gcd(m, n) == 1) {
                int a0 = m * m - n * n;
                int b0 = 2 * m * n;
                int c0 = m * m + n * n;

                if (c0 > 2 * N) break;

                for (int k = 1; (i64)k * c0 <= 2 * N; k++) {
                    int a = k * a0, b = k * b0, c = k * c0;

                    /* Triple (a, b, c): store in both directions */
                    if (c <= N) {
                        if (a <= N && end_indices[a] < start_indices[a + 1]) {
                            other_legs[end_indices[a]++] = b;
                        }
                        if (b <= N && end_indices[b] < start_indices[b + 1]) {
                            other_legs[end_indices[b]++] = a;
                        }
                    }

                    /* Triple (b, a, c): store in both directions */
                    if (c <= N) {
                        if (b <= N && end_indices[b] < start_indices[b + 1]) {
                            other_legs[end_indices[b]++] = a;
                        }
                        if (a <= N && end_indices[a] < start_indices[a + 1]) {
                            other_legs[end_indices[a]++] = b;
                        }
                    }
                }
            }
        }
    }

    i64 ans = 0;

    /* Process each even a */
    for (int a = 2; a <= N; a += 2) {
        int start = start_indices[a];
        int end = end_indices[a];
        if (end <= start) continue;

        /* Sort other legs */
        qsort(other_legs + start, end - start, sizeof(int), cmp_int);

        /* Find valid (b1, b2, b3) where 2*b1 + b2 = b3 */
        for (int i = start; i < end; i++) {
            int b1 = other_legs[i];
            int j = start;
            int k = start + 1;

            while (k < end) {
                int b2 = other_legs[j];
                int b3 = other_legs[k];

                if (b2 > 2 * b1) break;

                if (2 * b1 + b2 < b3) {
                    j++;
                } else if (2 * b1 + b2 > b3) {
                    k++;
                } else {
                    /* Found valid combination */
                    int perim = a + 2 * b1 + isqrt_i((i64)a * a + (i64)b2 * b2);
                    if (b2 % 2 == 0 && perim <= N) {
                        ans += perim;
                    }
                    j++;
                    k++;
                }
            }
        }
    }

    printf("%lld\n", (long long)ans);

    free(ff); free(num_factors); free(start_indices);
    free(end_indices); free(other_legs);
    return 0;
}
