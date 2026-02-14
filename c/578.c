/*
 * Project Euler Problem 578: Integers with Decreasing Prime Powers
 *
 * Count integers n = prod(p_i^e_i) up to N=10^13 such that e_i >= e_j if p_i <= p_j.
 * Uses recursive inclusion-exclusion over powerful numbers.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define MAXP 3700000

static int *prime_list;
static int nprime;

static void sieve(int limit) {
    char *is_prime = (char *)calloc(limit + 1, 1);
    for (int i = 2; i <= limit; i++) is_prime[i] = 1;
    for (long long i = 2; i * i <= limit; i++)
        if (is_prime[i])
            for (long long j = i * i; j <= limit; j += i)
                is_prime[j] = 0;
    nprime = 0;
    for (int i = 2; i <= limit; i++)
        if (is_prime[i]) nprime++;
    prime_list = (int *)malloc(nprime * sizeof(int));
    int idx = 0;
    for (int i = 2; i <= limit; i++)
        if (is_prime[i]) prime_list[idx++] = i;
    free(is_prime);
}

static long long NN;
static long long ans;

static int parity(int n) {
    return (n % 2 == 0) ? 1 : -1;
}

static void find_remaining(int min_index, long long n, int count, int threshold) {
    ans += (NN / n) * parity(count);
    for (int index = min_index; index < nprime; index++) {
        long long p = prime_list[index];
        if (index >= threshold) {
            if (n > NN / (p * p)) break;
            find_remaining(index + 1, n * p * p, count + 1, threshold);
        } else {
            if (n > NN / p) break;
            find_remaining(index + 1, n * p, count + 1, threshold);
        }
    }
}

static void find_powerfuls(int min_index, long long n, int prev_e) {
    find_remaining(0, n, 0, min_index);
    for (int index = min_index; index < nprime; index++) {
        long long p = prime_list[index];
        if (n > NN / (p * p)) break;
        long long nn = n * p;
        for (int e = 2; e <= prev_e; e++) {
            nn *= p;
            if (nn > NN / p) break;
            find_powerfuls(index + 1, nn * p, e);
        }
    }
}

int main(void) {
    NN = 10000000000000LL;  /* 10^13 */
    int limit = (int)sqrt((double)NN) + 1;
    sieve(limit);

    ans = 0;
    find_powerfuls(0, 1, 999);

    printf("%lld\n", ans);

    free(prime_list);
    return 0;
}
