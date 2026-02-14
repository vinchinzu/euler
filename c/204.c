/*
 * Project Euler 204: Generalised Hamming Numbers
 *
 * Count integers up to 10^9 with no prime factor > 100.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    long long N = 1000000000LL;
    int K = 100;

    /* Sieve primes up to K */
    int is_prime[101];
    memset(is_prime, 1, sizeof(is_prime));
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; i * i <= K; i++)
        if (is_prime[i])
            for (int j = i*i; j <= K; j += i)
                is_prime[j] = 0;

    int primes[30], np = 0;
    for (int i = 2; i <= K; i++)
        if (is_prime[i]) primes[np++] = i;

    /* Generate all K-smooth numbers up to N */
    /* Use a dynamic array */
    long long *nums = (long long *)malloc(4000000 * sizeof(long long));
    nums[0] = 1;
    int count = 1;

    for (int pi = 0; pi < np; pi++) {
        int p = primes[pi];
        int sz = count;
        for (int i = 0; i < sz; i++) {
            long long prod = nums[i];
            while (1) {
                prod *= p;
                if (prod > N) break;
                nums[count++] = prod;
            }
        }
    }

    printf("%d\n", count);
    free(nums);
    return 0;
}
