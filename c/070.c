/* Project Euler 070 - Totient permutation */
#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <math.h>

#define N_LIMIT 10000000
#define SIEVE_LIMIT 5000

static bool sieve[SIEVE_LIMIT + 1];
static int primes[700];
static int nprimes;

static bool are_permutations(int n1, int n2) {
    int counts[10] = {0};
    while (n1 > 0) {
        counts[n1 % 10]++;
        n1 /= 10;
    }
    while (n2 > 0) {
        counts[n2 % 10]--;
        n2 /= 10;
    }
    for (int i = 0; i < 10; i++) {
        if (counts[i] != 0) return false;
    }
    return true;
}

int main(void) {
    /* Sieve of Eratosthenes */
    memset(sieve, 1, sizeof(sieve));
    sieve[0] = sieve[1] = false;
    for (int i = 2; i * i <= SIEVE_LIMIT; i++) {
        if (sieve[i]) {
            for (int j = i * i; j <= SIEVE_LIMIT; j += i) {
                sieve[j] = false;
            }
        }
    }
    nprimes = 0;
    for (int i = 2; i <= SIEVE_LIMIT; i++) {
        if (sieve[i]) primes[nprimes++] = i;
    }

    double min_ratio = 1e9;
    int result_n = 0;

    for (int i = 0; i < nprimes; i++) {
        int p1 = primes[i];
        if ((long long)p1 * p1 >= N_LIMIT) break;

        for (int j = i; j < nprimes; j++) {
            int p2 = primes[j];
            long long n = (long long)p1 * p2;
            if (n >= N_LIMIT) break;
            if (p1 == p2) continue;

            long long phi_n = (long long)(p1 - 1) * (p2 - 1);

            if (are_permutations((int)n, (int)phi_n)) {
                double ratio = (double)n / phi_n;
                if (ratio < min_ratio) {
                    min_ratio = ratio;
                    result_n = (int)n;
                }
            }
        }
    }

    printf("%d\n", result_n);
    return 0;
}
