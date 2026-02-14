/*
 * Project Euler Problem 293: Pseudo-Fortunate Numbers
 *
 * An admissible integer has consecutive prime factors starting from 2.
 * The pseudo-Fortunate number for admissible n is the smallest m>=2
 * such that n+m is prime. Find the sum of distinct pseudo-Fortunate numbers.
 */
#include <stdio.h>
#include <string.h>
#include <math.h>

#define N 1000000000LL
#define SIEVE_SIZE 2000000

static char sieve[SIEVE_SIZE];

static int is_prime_check(long long n) {
    if (n < 2) return 0;
    if (n < SIEVE_SIZE) return sieve[n];
    if (n % 2 == 0) return 0;
    if (n % 3 == 0) return 0;
    for (long long i = 5; i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0) return 0;
    }
    return 1;
}

/* Small primes for admissible generation */
static const int primes[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29};
static const int nprimes = 10;

/* Store fortunate numbers */
#define MAX_FORTUNATE 100000
static long long fortunates[MAX_FORTUNATE];
static int nfortunates = 0;

/* Check if value is already in fortunates */
static int contains(long long val) {
    for (int i = 0; i < nfortunates; i++) {
        if (fortunates[i] == val) return 1;
    }
    return 0;
}

static void add_fortunate(long long val) {
    if (!contains(val)) {
        fortunates[nfortunates++] = val;
    }
}

/* Generate admissible numbers and find pseudo-Fortunate for each */
static void generate(int pi, long long val) {
    /* val is an admissible number (product of consecutive primes from 2) */
    /* Find pseudo-Fortunate: smallest m >= 2 such that val + m is prime */
    long long m = 2;
    while (!is_prime_check(val + m)) m++;
    add_fortunate(m);

    /* Try higher powers of primes[pi-1] */
    if (pi > 0) {
        long long next = val * primes[pi - 1];
        while (next < N) {
            m = 2;
            while (!is_prime_check(next + m)) m++;
            add_fortunate(m);
            next *= primes[pi - 1];
        }
    }

    /* Try next prime */
    if (pi < nprimes) {
        long long next = val * primes[pi];
        if (next < N) {
            generate(pi + 1, next);
            /* Also try higher powers */
            long long powered = next * primes[pi];
            while (powered < N) {
                generate(pi + 1, powered);
                powered *= primes[pi];
            }
        }
    }
}

int main(void) {
    /* Initialize prime sieve */
    memset(sieve, 1, sizeof(sieve));
    sieve[0] = sieve[1] = 0;
    for (int i = 2; i * i < SIEVE_SIZE; i++) {
        if (sieve[i]) {
            for (int j = i * i; j < SIEVE_SIZE; j += i)
                sieve[j] = 0;
        }
    }

    /* Generate all admissible numbers < N using a different approach:
     * An admissible number must use all primes from 2 up to some prime p,
     * each raised to power >= 1. */
    /* Recursive generation: start with 2^a1 * 3^a2 * 5^a3 * ... */

    /* Actually, let's enumerate directly */
    nfortunates = 0;

    /* DFS: at each level, we decide the exponent for primes[level] */
    /* We must use primes[0], primes[1], ..., primes[level] each at least once */
    /* Actually: admissible = 2^a * 3^b * 5^c * ... with a>=1, b>=1, etc., using consecutive primes */

    /* Generate all admissible < N */
    long long stack_val[100000];
    int stack_pi[100000];
    int sp = 0;

    /* Start: must include prime 2 */
    long long v = 2;
    while (v < N) {
        /* v is admissible (product of 2^k for some k) */
        long long m = 2;
        while (!is_prime_check(v + m)) m++;
        add_fortunate(m);

        /* Extend with prime 3 */
        long long w = v * 3;
        while (w < N) {
            /* w uses 2 and 3 */
            m = 2;
            while (!is_prime_check(w + m)) m++;
            add_fortunate(m);

            long long x = w * 5;
            while (x < N) {
                m = 2;
                while (!is_prime_check(x + m)) m++;
                add_fortunate(m);

                long long y = x * 7;
                while (y < N) {
                    m = 2;
                    while (!is_prime_check(y + m)) m++;
                    add_fortunate(m);

                    long long z = y * 11;
                    while (z < N) {
                        m = 2;
                        while (!is_prime_check(z + m)) m++;
                        add_fortunate(m);

                        long long a = z * 13;
                        while (a < N) {
                            m = 2;
                            while (!is_prime_check(a + m)) m++;
                            add_fortunate(m);

                            long long b = a * 17;
                            while (b < N) {
                                m = 2;
                                while (!is_prime_check(b + m)) m++;
                                add_fortunate(m);

                                long long c = b * 19;
                                while (c < N) {
                                    m = 2;
                                    while (!is_prime_check(c + m)) m++;
                                    add_fortunate(m);

                                    long long d = c * 23;
                                    while (d < N) {
                                        m = 2;
                                        while (!is_prime_check(d + m)) m++;
                                        add_fortunate(m);
                                        d *= 23;
                                    }
                                    c *= 19;
                                }
                                b *= 17;
                            }
                            a *= 13;
                        }
                        z *= 11;
                    }
                    y *= 7;
                }
                x *= 5;
            }
            w *= 3;
        }
        v *= 2;
    }

    long long sum = 0;
    for (int i = 0; i < nfortunates; i++) {
        sum += fortunates[i];
    }
    printf("%lld\n", sum);
    return 0;
}
