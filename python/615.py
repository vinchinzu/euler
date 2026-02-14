"""Project Euler Problem 615: The millionth number with at least one million
prime factors.

Uses embedded C for performance.
"""
import subprocess, os, tempfile


def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

#define N_VAL 1000000
#define MOD_VAL 123454321LL

static int *primes;
static int nprimes;
static double *log_primes;

/* Results: stored as parallel arrays of (log_val, mod_val) */
static double *res_log;
static long long *res_mod;
static int res_count;
static int res_cap;

static void ensure_cap(void) {
    if (res_count >= res_cap) {
        res_cap = res_cap * 2 + 1;
        res_log = (double *)realloc(res_log, res_cap * sizeof(double));
        res_mod = (long long *)realloc(res_mod, res_cap * sizeof(long long));
    }
}

static void add_result(double lv, long long mv) {
    ensure_cap();
    res_log[res_count] = lv;
    res_mod[res_count] = mv;
    res_count++;
}

static void sieve_primes(int limit) {
    char *is_prime = (char *)calloc(limit + 1, 1);
    memset(is_prime, 1, limit + 1);
    is_prime[0] = is_prime[1] = 0;
    int sq = (int)sqrt((double)limit);
    for (int i = 2; i <= sq; i++) {
        if (is_prime[i]) {
            for (int j = i*i; j <= limit; j += i)
                is_prime[j] = 0;
        }
    }
    /* Count primes */
    nprimes = 0;
    for (int i = 2; i <= limit; i++)
        if (is_prime[i]) nprimes++;
    primes = (int *)malloc(nprimes * sizeof(int));
    log_primes = (double *)malloc(nprimes * sizeof(double));
    int idx = 0;
    for (int i = 2; i <= limit; i++) {
        if (is_prime[i]) {
            primes[idx] = i;
            log_primes[idx] = log((double)i);
            idx++;
        }
    }
    free(is_prime);
}

static double limit_val;

static void helper(int min_index, int num_primes, double log_val, long long mod_val) {
    if (num_primes >= N_VAL) {
        add_result(log_val, mod_val);
    }
    for (int index = min_index; index < nprimes; index++) {
        double lp = log_primes[index];
        int remaining = N_VAL - num_primes;
        if (remaining < 1) remaining = 1;
        if (log_val + remaining * lp > limit_val)
            break;
        long long new_mod = mod_val * primes[index] % MOD_VAL;
        int e = 1;
        while (log_val + e * lp < limit_val) {
            helper(index + 1, num_primes + e, log_val + e * lp, new_mod);
            e++;
            new_mod = new_mod * primes[index] % MOD_VAL;
        }
    }
}

/* Compare function for sorting by log value */
static int cmp_idx(const void *a, const void *b) {
    int ia = *(const int *)a;
    int ib = *(const int *)b;
    if (res_log[ia] < res_log[ib]) return -1;
    if (res_log[ia] > res_log[ib]) return 1;
    return 0;
}

int main(void) {
    sieve_primes(N_VAL + 2);

    res_cap = 1024 * 1024;
    res_count = 0;
    res_log = (double *)malloc(res_cap * sizeof(double));
    res_mod = (long long *)malloc(res_cap * sizeof(long long));

    limit_val = N_VAL * log(2.0);
    while (1) {
        res_count = 0;
        helper(0, 0, 0.0, 1);
        if (res_count >= N_VAL) {
            /* Sort by log value using index sort */
            int *indices = (int *)malloc(res_count * sizeof(int));
            for (int i = 0; i < res_count; i++) indices[i] = i;
            qsort(indices, res_count, sizeof(int), cmp_idx);
            printf("%lld\n", res_mod[indices[N_VAL - 1]]);
            free(indices);
            break;
        }
        limit_val += 1.0;
    }

    free(res_log);
    free(res_mod);
    free(primes);
    free(log_primes);
    return 0;
}
"""
    tmp = tempfile.NamedTemporaryFile(suffix='.c', delete=False, mode='w')
    tmp.write(c_code)
    tmp.close()
    exe = tmp.name.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', exe, tmp.name, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.run([exe], capture_output=True, text=True, timeout=280)
        print(result.stdout.strip())
    finally:
        os.unlink(tmp.name)
        if os.path.exists(exe):
            os.unlink(exe)


if __name__ == "__main__":
    solve()
