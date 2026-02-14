/*
 * Project Euler 029 - Distinct Powers
 * How many distinct terms are in the sequence a^b for 2<=a<=100, 2<=b<=100?
 *
 * Strategy: represent each a^b by its canonical prime factorization.
 * Store as (prime, exponent) pairs and use a sorted list of all such
 * factorizations to count distinct entries.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
 * Each a^b can be represented as prime factorization.
 * Max base is 100, so max distinct prime factor is small.
 * We store factorizations as arrays of (prime, exponent*b) pairs.
 * To count distinct, we sort them and count unique.
 */

#define MAX_ENTRIES 9801  /* 99 * 99 */
#define MAX_FACTORS 4     /* 100 = 2^2 * 5^2, at most a few prime factors */

typedef struct {
    int primes[MAX_FACTORS];
    int exponents[MAX_FACTORS];
    int count; /* number of prime factors */
} Factorization;

Factorization entries[MAX_ENTRIES];
int entry_count = 0;

int cmp_fact(const void *a, const void *b) {
    const Factorization *fa = (const Factorization *)a;
    const Factorization *fb = (const Factorization *)b;
    if (fa->count != fb->count) return fa->count - fb->count;
    for (int i = 0; i < fa->count; i++) {
        if (fa->primes[i] != fb->primes[i]) return fa->primes[i] - fb->primes[i];
        if (fa->exponents[i] != fb->exponents[i]) return fa->exponents[i] - fb->exponents[i];
    }
    return 0;
}

int main(void) {
    for (int a = 2; a <= 100; a++) {
        /* Factorize a */
        int primes[MAX_FACTORS], exps[MAX_FACTORS];
        int fc = 0;
        int num = a;
        for (int i = 2; i * i <= num; i++) {
            int cnt = 0;
            while (num % i == 0) {
                cnt++;
                num /= i;
            }
            if (cnt > 0) {
                primes[fc] = i;
                exps[fc] = cnt;
                fc++;
            }
        }
        if (num > 1) {
            primes[fc] = num;
            exps[fc] = 1;
            fc++;
        }

        for (int b = 2; b <= 100; b++) {
            Factorization *e = &entries[entry_count++];
            e->count = fc;
            for (int i = 0; i < fc; i++) {
                e->primes[i] = primes[i];
                e->exponents[i] = exps[i] * b;
            }
        }
    }

    /* Sort and count distinct */
    qsort(entries, entry_count, sizeof(Factorization), cmp_fact);

    int distinct = 1;
    for (int i = 1; i < entry_count; i++) {
        if (cmp_fact(&entries[i], &entries[i - 1]) != 0) {
            distinct++;
        }
    }

    printf("%d\n", distinct);
    return 0;
}
