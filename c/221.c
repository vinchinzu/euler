/*
 * Project Euler Problem 221: Alexandrian Integers
 *
 * A is Alexandrian if A = p*q*r and 1/A = 1/p + 1/q + 1/r.
 * This implies A = p*q*r and p*q + p*r + q*r = 1.
 *
 * Setting p = a, we get q*r = a^2+1 and q+r = 2a + (a^2+1)/d + d
 * for each divisor d of a^2+1. Then A = a * (a + d) * (a + (a^2+1)/d).
 *
 * We enumerate a from 1..L, find all divisors of a^2+1, compute A.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define N_TARGET 150000
#define L 80000
#define MAX_ALEX 500000

static long long alexandrians[MAX_ALEX];
static int alex_count = 0;

/* Find all divisors of n and store A = a*(a+d)*(a+n/d) */
static void find_alexandrians(long long a) {
    long long n = a * a + 1;
    long long sq = (long long)sqrt((double)n);
    for (long long d = 1; d <= sq; d++) {
        if (n % d == 0) {
            long long e = n / d;
            long long A = a * (a + d) * (a + e);
            if (alex_count < MAX_ALEX)
                alexandrians[alex_count++] = A;
            if (d != e) {
                A = a * (a + e) * (a + d);
                /* This is the same value, skip */
            }
        }
    }
}

static int cmp_ll(const void *a, const void *b) {
    long long x = *(const long long *)a;
    long long y = *(const long long *)b;
    if (x < y) return -1;
    if (x > y) return 1;
    return 0;
}

int main(void) {
    for (long long a = 1; a <= L; a++) {
        long long n = a * a + 1;
        long long sq = (long long)sqrt((double)n);
        for (long long d = 1; d <= sq; d++) {
            if (n % d == 0) {
                long long e = n / d;
                long long p = a + d;
                long long q = a + e;
                long long A = a * p;
                /* Check overflow before final multiply */
                if (A <= (long long)9e18 / q) {
                    A *= q;
                    if (alex_count < MAX_ALEX)
                        alexandrians[alex_count++] = A;
                }
            }
        }
    }

    /* Sort and remove duplicates */
    qsort(alexandrians, alex_count, sizeof(long long), cmp_ll);

    /* Remove duplicates */
    int unique = 0;
    for (int i = 0; i < alex_count; i++) {
        if (i == 0 || alexandrians[i] != alexandrians[i - 1])
            alexandrians[unique++] = alexandrians[i];
    }

    printf("%lld\n", alexandrians[N_TARGET - 1]);
    return 0;
}
