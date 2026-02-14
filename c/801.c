/*
 * Project Euler Problem 801: x^y = y^x (mod n).
 *
 * Sieve primes, factor p-1 for primes p in [A, A+B],
 * compute multiplicative function, sum results.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define A_VAL 10000000000000000LL  /* 10^16 */
#define B_VAL 1000000
#define M_VAL 993353399LL

static long long mod_pow(long long base, long long exp, long long mod) {
    long long result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    long long A = A_VAL;
    int B = B_VAL;
    long long M = M_VAL;
    int L = (int)sqrt((double)(A + B)) + 1;

    /* Sieve primes up to L */
    char *is_prime = (char *)calloc(L + 1, 1);
    memset(is_prime, 1, L + 1);
    is_prime[0] = is_prime[1] = 0;
    int sq = (int)sqrt((double)L);
    for (int i = 2; i <= sq; i++) {
        if (is_prime[i]) {
            for (int j = i*i; j <= L; j += i)
                is_prime[j] = 0;
        }
    }

    /* Collect primes */
    int nprimes = 0;
    for (int i = 2; i <= L; i++)
        if (is_prime[i]) nprimes++;
    int *primes = (int *)malloc(nprimes * sizeof(int));
    int idx = 0;
    for (int i = 2; i <= L; i++)
        if (is_prime[i]) primes[idx++] = i;
    free(is_prime);

    /* For each index in [0, B], store list of small prime factors. */
    int *factor_count = (int *)calloc(B + 1, sizeof(int));
    for (int pi = 0; pi < nprimes; pi++) {
        int p = primes[pi];
        long long rem = A % p;
        int start = (rem == 0) ? 0 : (int)(p - rem);
        for (int i = start; i <= B; i += p) {
            if ((long long)p < A + i)
                factor_count[i]++;
        }
    }

    /* Allocate factor storage */
    int *factor_offsets = (int *)malloc((B + 2) * sizeof(int));
    factor_offsets[0] = 0;
    for (int i = 0; i <= B; i++)
        factor_offsets[i + 1] = factor_offsets[i] + factor_count[i];
    int total_factors = factor_offsets[B + 1];
    int *factors = (int *)malloc(total_factors * sizeof(int));
    int *cur_pos = (int *)calloc(B + 1, sizeof(int));

    /* Second pass: fill factors */
    for (int pi = 0; pi < nprimes; pi++) {
        int p = primes[pi];
        long long rem = A % p;
        int start = (rem == 0) ? 0 : (int)(p - rem);
        for (int i = start; i <= B; i += p) {
            if ((long long)p < A + i) {
                int off = factor_offsets[i] + cur_pos[i];
                factors[off] = p;
                cur_pos[i]++;
            }
        }
    }
    free(cur_pos);

    long long ans = 0;

    for (int i = 1; i <= B; i++) {
        /* Check if A+i is prime: it's prime if it has no small factors */
        if (factor_count[i] == 0) {
            long long n = A + i - 1;

            /* Factorize n using factors of index i-1 */
            long long temp = n;
            long long res = 1;

            int nf = factor_count[i - 1];
            int off = factor_offsets[i - 1];
            for (int j = 0; j < nf; j++) {
                int p = factors[off + j];
                if (temp % p != 0) continue;
                int e = 0;
                while (temp % p == 0) {
                    temp /= p;
                    e++;
                }
                long long term = (mod_pow(p, 3*e, M) + mod_pow(p, 3*e - 1, M)
                                  - mod_pow(p, 2*e - 1, M) + M) % M;
                res = res * term % M;
            }
            /* If temp > 1, there's a remaining prime factor */
            if (temp > 1) {
                long long p = temp;
                long long term = (mod_pow(p, 3, M) + mod_pow(p, 2, M)
                                  - mod_pow(p, 1, M) + M) % M;
                res = res * term % M;
            }

            ans = (ans + mod_pow(n, 2, M) + res) % M;
        }
    }

    printf("%lld\n", ans);

    free(primes);
    free(factor_count);
    free(factor_offsets);
    free(factors);
    return 0;
}
