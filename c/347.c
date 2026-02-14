/* Project Euler 347 - Largest integer divisible by two primes
 *
 * S(N): sum of all distinct M(p,q,N) for prime pairs p<q with p*q<=N.
 * M(p,q,N) = largest m<=N with exactly prime factors {p,q}.
 *
 * For each pair (p,q), enumerate p^a * q^b <= N with a>=1, b>=1.
 */

#include <stdio.h>
#include <string.h>

#define N 10000000
#define SQRT_N 3163

static char is_composite[N + 1];
static int primes[700000];
static int nprimes;

void sieve(void) {
    memset(is_composite, 0, sizeof(is_composite));
    nprimes = 0;
    for (int i = 2; i <= N; i++) {
        if (!is_composite[i]) {
            primes[nprimes++] = i;
            if ((long long)i * i <= N) {
                for (long long j = (long long)i * i; j <= N; j += i)
                    is_composite[j] = 1;
            }
        }
    }
}

int main(void) {
    sieve();

    long long total = 0;

    for (int i = 0; i < nprimes; i++) {
        long long p = primes[i];
        if (p * p > N) break; /* q > p, so p*q > N */

        for (int j = i + 1; j < nprimes; j++) {
            long long q = primes[j];
            if (p * q > N) break;

            /* Find largest p^a * q^b <= N with a>=1, b>=1 */
            long long best = 0;
            long long pa = p;
            while (pa * q <= N) {
                /* For this pa, find largest q^b such that pa * q^b <= N */
                long long val = pa;
                while (val * q <= N) val *= q;
                if (val > best) best = val;
                pa *= p;
            }

            total += best;
        }
    }

    printf("%lld\n", total);
    return 0;
}
