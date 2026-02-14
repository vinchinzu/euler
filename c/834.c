/*
 * Project Euler 834: Add and Divide
 *
 * f(m) = n(m+1) + m(m+1)/2 = 1/2 ((n+m)(n+m+1) - n(n-1))
 * So (n+m) | n(n-1) => iterate over divisors d of n(n-1), d > n.
 * Then m = d-n, check parity condition.
 *
 * Uses SPF sieve for factorization.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define NMAX 1234567

static int spf[NMAX + 1];

static void sieve_spf(void) {
    for (int i = 0; i <= NMAX; i++) spf[i] = i;
    spf[0] = spf[1] = 0;
    for (int i = 2; (long long)i * i <= NMAX; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j <= NMAX; j += i) {
                if (spf[j] == j) spf[j] = i;
            }
        }
    }
}

/* Get prime factorization of n using spf */
typedef struct { int p; int e; } PFactor;

static int factorize(long long n, PFactor *factors) {
    int nf = 0;
    /* For n up to ~1.5e12, we need to handle primes > NMAX */
    /* But n = k * (k-1) where k <= NMAX, so all prime factors <= NMAX */
    /* Actually n(n-1) can have factors up to n-1 < NMAX, so spf works for each */

    /* Factor using trial division with known small primes from spf */
    for (int p = 2; (long long)p * p <= n && p <= NMAX; p++) {
        if (spf[p] != p) continue; /* not prime */
        if (n % p == 0) {
            factors[nf].p = p;
            factors[nf].e = 0;
            while (n % p == 0) { n /= p; factors[nf].e++; }
            nf++;
        }
    }
    if (n > 1) {
        factors[nf].p = (int)n;
        factors[nf].e = 1;
        nf++;
    }
    return nf;
}

/* Generate all divisors of n given its factorization */
static int gen_divisors(PFactor *factors, int nf, long long *divs) {
    int nd = 1;
    divs[0] = 1;
    for (int i = 0; i < nf; i++) {
        int cur = nd;
        long long pk = 1;
        for (int j = 0; j < factors[i].e; j++) {
            pk *= factors[i].p;
            for (int k = 0; k < cur; k++) {
                divs[nd++] = divs[k] * pk;
            }
        }
    }
    return nd;
}

int main(void) {
    sieve_spf();

    long long ans = 0;

    for (int n = 3; n <= NMAX; n++) {
        long long nn1 = (long long)n * (n - 1);

        /* Get prime factors of n and n-1 separately, then combine */
        PFactor factors[40];
        int nf = factorize(nn1, factors);

        long long divs[10000];
        int nd = gen_divisors(factors, nf, divs);

        for (int i = 0; i < nd; i++) {
            long long d = divs[i];
            if (d > n) {
                long long m = d - n;
                /* Check: (d + 1 - nn1/d) % 2 == 0 */
                long long val = d + 1 - nn1 / d;
                if (val % 2 == 0) {
                    ans += m;
                }
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
