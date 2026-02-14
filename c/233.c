/*
 * Project Euler Problem 233: Lattice points on a circle
 *
 * Find sum of all n <= 10^11 such that the circle has exactly 420 lattice points.
 * The number of lattice points = 4 * prod(2*a_i + 1) for primes p ≡ 1 mod 4.
 * We need prod(2*a_i + 1) = 105.
 * Valid exponent tuples: (1,2,3), (2,10), (3,7)
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define PLIMIT 5000000

static int primes_1mod4[500000];
static int p1_count = 0;

/* Sieve primes up to limit */
static char is_prime_arr[PLIMIT + 1];

static void sieve_primes(void) {
    memset(is_prime_arr, 1, sizeof(is_prime_arr));
    is_prime_arr[0] = is_prime_arr[1] = 0;
    for (int i = 2; (long long)i * i <= PLIMIT; i++) {
        if (is_prime_arr[i]) {
            for (int j = i * i; j <= PLIMIT; j += i)
                is_prime_arr[j] = 0;
        }
    }
    for (int i = 2; i <= PLIMIT; i++) {
        if (is_prime_arr[i] && i % 4 == 1)
            primes_1mod4[p1_count++] = i;
    }
}

/*
 * Build prefix sums of valid multipliers (numbers with no prime factor ≡ 1 mod 4).
 * A valid multiplier m has all prime factors ≡ 2 or 3 mod 4 (or = 2).
 */
static long long N_VAL = 100000000000LL;
/* MAX_MULT = N / (5^3 * 13^2 * 17) = ~2.77M, but let's be safe */
#define MAX_MULT 3000000

static long long valid_sum[MAX_MULT + 1];
static char is_valid[MAX_MULT + 1];

static void build_valid_sums(void) {
    memset(is_valid, 1, sizeof(is_valid));
    is_valid[0] = 0;

    /* Sieve out numbers with prime factor ≡ 1 mod 4 */
    for (int i = 2; i <= PLIMIT && i <= MAX_MULT; i++) {
        if (is_prime_arr[i] && i % 4 == 1) {
            for (int j = i; j <= MAX_MULT; j += i)
                is_valid[j] = 0;
        }
    }

    long long v = 0;
    valid_sum[0] = 0;
    for (int m = 1; m <= MAX_MULT; m++) {
        if (is_valid[m])
            v += m;
        valid_sum[m] = v;
    }
}

static long long S(long long L) {
    if (L <= 0) return 0;
    if (L > MAX_MULT) L = MAX_MULT;
    return valid_sum[L];
}

int main(void) {
    sieve_primes();
    build_valid_sums();

    long long total = 0;

    /* Pattern (1,2,3): three distinct primes ≡ 1 mod 4, all 6 permutations */
    int perms3[][3] = {{1,2,3},{1,3,2},{2,1,3},{2,3,1},{3,1,2},{3,2,1}};
    for (int pi = 0; pi < 6; pi++) {
        int ep = perms3[pi][0], eq = perms3[pi][1], er = perms3[pi][2];
        for (int ip = 0; ip < p1_count; ip++) {
            long long p = primes_1mod4[ip];
            long long pp = 1;
            for (int e = 0; e < ep; e++) pp *= p;
            if (pp >= N_VAL) break;
            long long remain_p = N_VAL / pp;

            for (int iq = ip + 1; iq < p1_count; iq++) {
                long long q = primes_1mod4[iq];
                long long qq = 1;
                for (int e = 0; e < eq; e++) qq *= q;
                if (qq > remain_p) break;
                long long remain_pq = remain_p / qq;

                long long max_r;
                if (er == 1) max_r = remain_pq;
                else if (er == 2) max_r = (long long)sqrt((double)remain_pq);
                else { /* er == 3 */
                    max_r = (long long)cbrt((double)remain_pq);
                    while ((max_r + 1) * (max_r + 1) * (max_r + 1) <= remain_pq) max_r++;
                    while (max_r * max_r * max_r > remain_pq) max_r--;
                }

                if (max_r <= (long long)primes_1mod4[iq]) continue;

                /* Binary search for end index */
                int lo = iq + 1, hi = p1_count - 1;
                while (lo <= hi) {
                    int mid = lo + (hi - lo) / 2;
                    if (primes_1mod4[mid] <= max_r) lo = mid + 1;
                    else hi = mid - 1;
                }
                int end_ir = lo; /* first index > max_r */

                for (int ir = iq + 1; ir < end_ir; ir++) {
                    long long r = primes_1mod4[ir];
                    long long rr = 1;
                    for (int e = 0; e < er; e++) rr *= r;
                    long long core = pp * qq * rr;
                    long long L = N_VAL / core;
                    if (L <= 0) break;
                    total += core * S(L);
                }
            }
        }
    }

    /* Pattern (2,10): two distinct primes, both permutations */
    int perms2a[][2] = {{2,10},{10,2}};
    for (int pi = 0; pi < 2; pi++) {
        int ep = perms2a[pi][0], eq = perms2a[pi][1];
        for (int ip = 0; ip < p1_count; ip++) {
            long long p = primes_1mod4[ip];
            long long pp = 1;
            for (int e = 0; e < ep; e++) { pp *= p; if (pp > N_VAL) break; }
            if (pp >= N_VAL) break;
            long long remain = N_VAL / pp;

            for (int iq = ip + 1; iq < p1_count; iq++) {
                long long q = primes_1mod4[iq];
                long long qq = 1;
                for (int e = 0; e < eq; e++) { qq *= q; if (qq > remain) break; }
                if (qq > remain) break;
                long long core = pp * qq;
                total += core * S(N_VAL / core);
            }
        }
    }

    /* Pattern (3,7): two distinct primes, both permutations */
    int perms2b[][2] = {{3,7},{7,3}};
    for (int pi = 0; pi < 2; pi++) {
        int ep = perms2b[pi][0], eq = perms2b[pi][1];
        for (int ip = 0; ip < p1_count; ip++) {
            long long p = primes_1mod4[ip];
            long long pp = 1;
            for (int e = 0; e < ep; e++) { pp *= p; if (pp > N_VAL) break; }
            if (pp >= N_VAL) break;
            long long remain = N_VAL / pp;

            for (int iq = ip + 1; iq < p1_count; iq++) {
                long long q = primes_1mod4[iq];
                long long qq = 1;
                for (int e = 0; e < eq; e++) { qq *= q; if (qq > remain) break; }
                if (qq > remain) break;
                long long core = pp * qq;
                total += core * S(N_VAL / core);
            }
        }
    }

    printf("%lld\n", total);
    return 0;
}
