/*
 * Project Euler 646 - Bounded Divisors
 * Sum of lambda(d)*d over divisors d of N! where L <= d <= H.
 * Meet-in-the-middle with binary search.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define MOD 1000000007LL

static long long pow_mod(long long base, long long exp, long long mod) {
    long long result = 1;
    base = ((base % mod) + mod) % mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

static int primes[] = {2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67};
static int num_primes = 19;
static int exponents[19];

static int num_factors_in_factorial(int n, int p) {
    int count = 0;
    long long power = p;
    while (power <= n) {
        count += n / (int)power;
        power *= p;
    }
    return count;
}

typedef struct {
    long double log_val;
    long long lio;
} Divisor;

static int cmp_divisor(const void *a, const void *b) {
    long double da = ((const Divisor *)a)->log_val;
    long double db = ((const Divisor *)b)->log_val;
    if (da < db) return -1;
    if (da > db) return 1;
    return 0;
}

static Divisor *gen_divisors(int start, int end, int *out_count) {
    long long total = 1;
    for (int i = start; i < end; i++) total *= (exponents[i] + 1);
    *out_count = (int)total;
    Divisor *divs = (Divisor *)malloc(total * sizeof(Divisor));
    if (!divs) { fprintf(stderr, "malloc fail\n"); exit(1); }
    divs[0].log_val = 0.0L;
    divs[0].lio = 1;
    int count = 1;
    for (int fi = start; fi < end; fi++) {
        int p = primes[fi];
        int e_max = exponents[fi];
        long double log_p = logl((long double)p);
        int old_count = count;
        for (int e = 1; e <= e_max; e++) {
            long long neg_p_e = pow_mod(MOD - p, e, MOD);
            long double log_pe = e * log_p;
            for (int j = 0; j < old_count; j++) {
                divs[count].log_val = divs[j].log_val + log_pe;
                divs[count].lio = divs[j].lio * neg_p_e % MOD;
                count++;
            }
        }
    }
    return divs;
}

int main(void) {
    int N = 70;
    long double log_L = 20.0L * logl(10.0L);
    long double log_H = 60.0L * logl(10.0L);

    for (int i = 0; i < num_primes; i++) {
        exponents[i] = num_factors_in_factorial(N, primes[i]);
    }

    long long total_factors = 1;
    for (int i = 0; i < num_primes; i++) total_factors *= (exponents[i] + 1);

    int half_index = 0;
    long long nf = 1;
    while (nf * nf < total_factors) {
        nf *= (exponents[half_index] + 1);
        half_index++;
    }

    int left_count, right_count;
    Divisor *left = gen_divisors(0, half_index, &left_count);
    Divisor *right = gen_divisors(half_index, num_primes, &right_count);

    qsort(left, left_count, sizeof(Divisor), cmp_divisor);

    long long *prefix = (long long *)malloc((left_count + 1) * sizeof(long long));
    prefix[0] = 0;
    for (int i = 0; i < left_count; i++) {
        prefix[i + 1] = (prefix[i] + left[i].lio) % MOD;
    }

    long double *left_logs = (long double *)malloc(left_count * sizeof(long double));
    for (int i = 0; i < left_count; i++) left_logs[i] = left[i].log_val;

    long double EPS_VAL = 1e-14L;
    long long ans = 0;

    for (int ri = 0; ri < right_count; ri++) {
        long double log_r = right[ri].log_val;
        long long lio_r = right[ri].lio;
        long double lo = log_L - log_r - EPS_VAL;
        long double hi = log_H - log_r + EPS_VAL;

        int a = 0, b_lo = left_count;
        while (a < b_lo) {
            int mid = (a + b_lo) / 2;
            if (left_logs[mid] <= lo) a = mid + 1;
            else b_lo = mid;
        }

        int b = a, b_hi = left_count;
        while (b < b_hi) {
            int mid = (b + b_hi) / 2;
            if (left_logs[mid] <= hi) b = mid + 1;
            else b_hi = mid;
        }

        long long range_sum = (prefix[b] - prefix[a] + MOD) % MOD;
        ans = (ans + lio_r * range_sum) % MOD;
    }

    ans = (ans % MOD + MOD) % MOD;
    printf("%lld\n", ans);

    free(left); free(right); free(prefix); free(left_logs);
    return 0;
}
