/*
 * Project Euler 266: Pseudo Square Root
 *
 * Let n = product of all primes < 190. Find the largest factor of n
 * that does not exceed sqrt(n). Answer mod 10^16.
 *
 * Meet-in-the-middle: split primes into two halves, enumerate all subset
 * products (as logs) for each half, then for each B-subset find the best
 * matching A-subset using binary search.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>
#include <string.h>

typedef long long ll;
typedef __int128 i128;

static int primes[50];
static int num_primes = 0;

static void sieve_primes(int limit) {
    char is_prime[200];
    memset(is_prime, 1, sizeof(is_prime));
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; i * i <= limit; i++)
        if (is_prime[i])
            for (int j = i*i; j <= limit; j += i)
                is_prime[j] = 0;
    for (int i = 2; i <= limit; i++)
        if (is_prime[i])
            primes[num_primes++] = i;
}

typedef struct {
    double log_val;
    int mask;
} Entry;

static int cmp_entry(const void *a, const void *b) {
    double da = ((const Entry *)a)->log_val;
    double db = ((const Entry *)b)->log_val;
    if (da < db) return -1;
    if (da > db) return 1;
    return 0;
}

int main(void) {
    ll M = 10000000000000000LL; /* 10^16 */

    sieve_primes(189);
    /* num_primes = 42 primes below 190 */

    int mid = num_primes / 2;
    int na = mid;
    int nb = num_primes - mid;

    double *plogs = (double *)malloc(num_primes * sizeof(double));
    for (int i = 0; i < num_primes; i++)
        plogs[i] = log((double)primes[i]);

    /* Build A subsets */
    int size_a = 1 << na;
    Entry *pa = (Entry *)malloc(size_a * sizeof(Entry));
    for (int s = 0; s < size_a; s++) {
        double lv = 0.0;
        for (int i = 0; i < na; i++)
            if (s & (1 << i))
                lv += plogs[i];
        pa[s].log_val = lv;
        pa[s].mask = s;
    }
    qsort(pa, size_a, sizeof(Entry), cmp_entry);

    /* Compute log(sqrt(n)) */
    double log_sqrt = 0.0;
    for (int i = 0; i < num_primes; i++)
        log_sqrt += plogs[i];
    log_sqrt /= 2.0;

    double best_log = -1.0;
    int best_a_mask = 0, best_b_mask = 0;

    /* For each B subset, binary search in A */
    for (int sb = 0; sb < (1 << nb); sb++) {
        double b_log = 0.0;
        for (int i = 0; i < nb; i++)
            if (sb & (1 << i))
                b_log += plogs[mid + i];

        double target = log_sqrt - b_log;

        /* Binary search for largest pa[idx].log_val <= target */
        int lo = 0, hi = size_a - 1, idx = -1;
        while (lo <= hi) {
            int m2 = lo + (hi - lo) / 2;
            if (pa[m2].log_val <= target) {
                idx = m2;
                lo = m2 + 1;
            } else {
                hi = m2 - 1;
            }
        }

        if (idx >= 0) {
            double cand = b_log + pa[idx].log_val;
            if (cand > best_log) {
                best_log = cand;
                best_a_mask = pa[idx].mask;
                best_b_mask = sb;
            }
        }
    }

    /* Compute answer mod M using the selected primes */
    i128 ans = 1;
    for (int i = 0; i < na; i++) {
        if (best_a_mask & (1 << i))
            ans = (ans * primes[i]) % M;
    }
    for (int i = 0; i < nb; i++) {
        if (best_b_mask & (1 << i))
            ans = (ans * primes[mid + i]) % M;
    }

    printf("%lld\n", (ll)ans);

    free(plogs);
    free(pa);
    return 0;
}
