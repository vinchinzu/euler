/*
 * Project Euler 850 - Fractional parts sum S(N)
 *
 * S(N) = sum_{k odd} sum_{n=1}^{N} f_k(n) where f_k(n) = sum_{i=1}^{n} {i^k/n}
 * Find floor(S(33557799775533)) mod 977676779.
 *
 * Algorithm: Uses multiplicative function properties and DFS over prime powers.
 * For each divisor d of n with structure, computes contribution via
 * "tail sum" and accumulates.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define MOD 977676779LL
#define MOD2 (2LL * MOD)

static long long N_val = 33557799775533LL;

/* Sieve for primes up to SQRT_N */
#define SQRT_N_MAX 5900000  /* sqrt(33557799775533) ~ 5792000 */
static char is_prime[SQRT_N_MAX + 1];
static long long p_sum[SQRT_N_MAX + 1]; /* Prefix sum of (p-1) for primes */

#define SMALL_PRIME_LIMIT 32000
static int small_primes[4000];
static int num_small_primes = 0;

#define MAX_SMALL_K 45
/* Small odd ks: 3, 5, 7, ..., 45 => 22 values */
static int small_odd_ks[22];
static int num_small_odd_ks = 0;

/* Total sums for each K and 'inf' */
static long long total_sums_k[22]; /* indexed by small_odd_ks index */
static long long total_sums_inf;

static void sieve_primes(void) {
    memset(is_prime, 1, sizeof(is_prime));
    is_prime[0] = is_prime[1] = 0;
    int limit = (int)sqrt((double)SQRT_N_MAX) + 1;
    for (int i = 2; i <= limit; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= SQRT_N_MAX; j += i)
                is_prime[j] = 0;
        }
    }

    long long cs = 0;
    for (int i = 2; i <= SQRT_N_MAX; i++) {
        if (is_prime[i]) {
            cs += (i - 1);
            if (i <= SMALL_PRIME_LIMIT)
                small_primes[num_small_primes++] = i;
        }
        p_sum[i] = cs;
    }
}

/* Compute p^exp mod nothing -- just integer power */
static long long ipow(long long p, int e) {
    long long r = 1;
    for (int i = 0; i < e; i++) r *= p;
    return r;
}

/* C_K(p^e) = p^(e - ceil(e/k)) for k; for 'inf', ceil(e/k) = 1 */
static long long get_c_k_val(int p, int e, int k) {
    int ceil_val = (e + k - 1) / k;
    int exp = e - ceil_val;
    return ipow((long long)p, exp);
}

static long long get_c_k_inf(int p, int e) {
    int exp = e - 1;
    return ipow((long long)p, exp);
}

static long long calc_tail_sum(long long M) {
    long long res = M;

    if ((long long)SMALL_PRIME_LIMIT * SMALL_PRIME_LIMIT >= M)
        return res;

    int min_p_bound = SMALL_PRIME_LIMIT;

    long long k = 1;
    while (1) {
        long long upper_bound_val = M / k;
        long long lower_bound_val = M / (k + 1);

        int upper_p = (int)sqrt((double)upper_bound_val);
        /* Correct sqrt */
        while ((long long)(upper_p + 1) * (upper_p + 1) <= upper_bound_val)
            upper_p++;
        while ((long long)upper_p * upper_p > upper_bound_val)
            upper_p--;

        int lower_p = (int)sqrt((double)lower_bound_val);
        while ((long long)(lower_p + 1) * (lower_p + 1) <= lower_bound_val)
            lower_p++;
        while ((long long)lower_p * lower_p > lower_bound_val)
            lower_p--;

        int eff_upper = upper_p;
        int eff_lower = lower_p > min_p_bound ? lower_p : min_p_bound;

        if (eff_upper > eff_lower && eff_upper <= SQRT_N_MAX) {
            long long term_sum = p_sum[eff_upper] - p_sum[eff_lower];
            res += term_sum * k;
        }

        if (eff_upper <= min_p_bound)
            break;

        k++;
    }

    /* Part 2: p^3 terms */
    int limit_p3 = (int)cbrt((double)M);
    while ((long long)(limit_p3 + 1) * (limit_p3 + 1) * (limit_p3 + 1) <= M)
        limit_p3++;
    while ((long long)limit_p3 * limit_p3 * limit_p3 > M)
        limit_p3--;

    if (limit_p3 > SMALL_PRIME_LIMIT && limit_p3 <= SQRT_N_MAX) {
        for (int p = SMALL_PRIME_LIMIT + 1; p <= limit_p3; p++) {
            if (is_prime[p]) {
                long long val = (long long)p * p - p;
                long long term = M / ((long long)p * p * p);
                res += val * term;
            }
        }
    }

    return res;
}

/* DFS over small primes */
/* current_vals: one value per small_odd_k plus one for 'inf' */
static long long curr_vals_k[22];
static long long curr_vals_inf;

static void dfs(int idx, long long current_d) {
    long long M = N_val / current_d;
    long long tail_mult = calc_tail_sum(M);

    for (int ki = 0; ki < num_small_odd_ks; ki++)
        total_sums_k[ki] = (total_sums_k[ki] + curr_vals_k[ki] % MOD2 * (tail_mult % MOD2)) % MOD2;
    total_sums_inf = (total_sums_inf + curr_vals_inf % MOD2 * (tail_mult % MOD2)) % MOD2;

    for (int i = idx; i < num_small_primes; i++) {
        int p = small_primes[i];
        if (current_d > N_val / ((long long)p * p))
            break;

        long long pe = (long long)p * p;
        int e = 2;

        while (1) {
            long long new_d = current_d * pe;
            if (new_d > N_val)
                break;

            /* Save old vals */
            long long old_k[22], old_inf;
            memcpy(old_k, curr_vals_k, sizeof(long long) * num_small_odd_ks);
            old_inf = curr_vals_inf;

            for (int ki = 0; ki < num_small_odd_ks; ki++) {
                long long term = get_c_k_val(p, e, small_odd_ks[ki])
                               - get_c_k_val(p, e - 1, small_odd_ks[ki]);
                curr_vals_k[ki] = (old_k[ki] * (term % MOD2)) % MOD2;
            }
            long long term_inf = get_c_k_inf(p, e) - get_c_k_inf(p, e - 1);
            curr_vals_inf = (old_inf * (term_inf % MOD2)) % MOD2;

            dfs(i + 1, new_d);

            /* Restore */
            memcpy(curr_vals_k, old_k, sizeof(long long) * num_small_odd_ks);
            curr_vals_inf = old_inf;

            pe *= p;
            e++;
        }
    }
}

int main(void) {
    sieve_primes();

    /* Build small odd ks: 3, 5, ..., 45 */
    for (int k = 3; k <= MAX_SMALL_K + 1; k += 2)
        small_odd_ks[num_small_odd_ks++] = k;

    /* Initialize totals */
    memset(total_sums_k, 0, sizeof(total_sums_k));
    total_sums_inf = 0;

    /* Initial vals */
    for (int ki = 0; ki < num_small_odd_ks; ki++)
        curr_vals_k[ki] = 1;
    curr_vals_inf = 1;

    dfs(0, 1);

    /* Calculate final S */
    long long num_odd = (N_val + 1) / 2;
    /* 2 * Term1 = num_odd * N*(N+1)/2 */
    long long N_mod = N_val % MOD2;
    long long Np1_mod = (N_val + 1) % MOD2;
    long long half_NN = (N_mod * Np1_mod / 2) % MOD2;
    long long term_doubled = (num_odd % MOD2 * half_NN) % MOD2;

    /* Sum of Sigma C_K */
    long long sum_sigma_ck = N_val % MOD2;

    for (int ki = 0; ki < num_small_odd_ks; ki++)
        sum_sigma_ck = (sum_sigma_ck + total_sums_k[ki]) % MOD2;

    long long num_small = num_small_odd_ks + 1; /* +1 for K=1 */
    long long num_large = num_odd - num_small;
    sum_sigma_ck = (sum_sigma_ck + (num_large % MOD2) * (total_sums_inf % MOD2)) % MOD2;

    long long two_S = ((term_doubled - sum_sigma_ck) % MOD2 + MOD2) % MOD2;
    long long ans = (two_S / 2) % MOD;

    printf("%lld\n", ans);
    return 0;
}
