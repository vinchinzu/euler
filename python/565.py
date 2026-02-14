"""Project Euler Problem 565 — Divisor Sum Divisibility.

Find the sum of all integers n<=N such that sigma(n) is divisible by K.
N=10^11, K=2017.

Uses inclusion-exclusion over prime power bases where sigma(p^e) % K == 0.
Ported to embedded C for speed.
"""
import subprocess, tempfile, os

C_CODE = r'''
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>

#define MAXN 100000000000LL  /* 10^11 */
#define KK 2017

typedef __int128 int128;

static int128 ans;
static int64_t NN = MAXN;

typedef struct {
    int64_t p;
    int e;
    int64_t pe; /* p^e */
} Base;

static Base *bases;
static int nbases;
static int bases_cap;

static int128 tr(int64_t n) {
    /* n*(n+1)/2, can be up to ~5*10^21 for n=10^11 */
    int128 nn = (int128)n;
    return nn * (nn + 1) / 2;
}

static void helper(int min_index, int parity, int64_t n) {
    if (n > 1) {
        int64_t q = NN / n;
        int128 t = tr(q);
        ans += (int128)parity * (int128)n * t;
    }
    int i;
    for (i = min_index; i < nbases; i++) {
        int64_t pe = bases[i].pe;
        /* Check overflow: n * pe > NN */
        if (pe > NN / n) break; /* since bases sorted by pe */
        int64_t npe = n * pe;

        helper(i + 1, -parity, npe);

        /* Also handle p^{e+1} */
        int64_t pe1 = pe * bases[i].p;
        if (pe1 <= NN / n) {
            helper(i + 1, parity, n * pe1);
        }
    }
}

/* Sieve primes up to limit */
static char *is_prime_arr;
static int *primes;
static int nprimes;

static void sieve_primes(int limit) {
    is_prime_arr = (char *)calloc(limit + 1, 1);
    memset(is_prime_arr, 1, limit + 1);
    is_prime_arr[0] = is_prime_arr[1] = 0;
    int i, j;
    for (i = 2; (int64_t)i * i <= limit; i++) {
        if (is_prime_arr[i]) {
            for (j = i * i; j <= limit; j += i)
                is_prime_arr[j] = 0;
        }
    }
    /* Count primes */
    nprimes = 0;
    for (i = 2; i <= limit; i++)
        if (is_prime_arr[i]) nprimes++;
    primes = (int *)malloc(nprimes * sizeof(int));
    int idx = 0;
    for (i = 2; i <= limit; i++)
        if (is_prime_arr[i]) primes[idx++] = i;
}

/* Extended GCD to find modular inverse */
static int64_t mod_inv(int64_t a, int64_t m) {
    int64_t t = 0, new_t = 1;
    int64_t r = m, new_r = a % m;
    while (new_r != 0) {
        int64_t q = r / new_r;
        int64_t tmp;
        tmp = new_t; new_t = t - q * new_t; t = tmp;
        tmp = new_r; new_r = r - q * new_r; r = tmp;
    }
    if (t < 0) t += m;
    return t;
}

/* Compare bases by pe for sorting */
static int cmp_bases(const void *a, const void *b) {
    int64_t pa = ((const Base *)a)->pe;
    int64_t pb = ((const Base *)b)->pe;
    if (pa < pb) return -1;
    if (pa > pb) return 1;
    return 0;
}

int main(void) {
    int sieve_limit = (int)sqrt((double)NN) + 1;
    sieve_primes(sieve_limit);

    /* Sieve to find which i have i*K - 1 prime */
    int64_t sieve_size = NN / KK + 1;
    char *sieve = (char *)malloc(sieve_size);
    memset(sieve, 1, sieve_size);

    int pi;
    for (pi = 0; pi < nprimes; pi++) {
        int64_t p = primes[pi];
        if (p == KK) continue;
        int64_t inv = mod_inv(KK, p);
        int64_t i;
        for (i = inv; i < sieve_size; i += p) {
            if (p != i * KK - 1)
                sieve[i] = 0;
        }
    }

    nbases = 0;
    bases_cap = 3000000;
    bases = (Base *)malloc(bases_cap * sizeof(Base));

    /* Add primes p = i*K - 1 with e=1 */
    int64_t ii;
    for (ii = 1; ii < sieve_size; ii++) {
        if (sieve[ii]) {
            int64_t p = ii * KK - 1;
            bases[nbases].p = p;
            bases[nbases].e = 1;
            bases[nbases].pe = p;
            nbases++;
        }
    }
    free(sieve);

    /* Add prime powers p^e where sigma(p^e) % K == 0 */
    for (pi = 0; pi < nprimes; pi++) {
        int64_t p = primes[pi];
        int64_t sum_div = 1 + p; /* sigma(p) */
        int e = 2;
        int64_t pe = p * p;
        while (pe <= NN) {
            sum_div = sum_div * p + 1; /* sigma(p^e) = 1 + p + ... + p^e */
            if (sum_div % KK == 0) {
                bases[nbases].p = (int64_t)p;
                bases[nbases].e = e;
                bases[nbases].pe = pe;
                nbases++;
            }
            e++;
            if (pe > NN / p) break;
            pe *= p;
        }
    }

    free(is_prime_arr);
    free(primes);

    qsort(bases, nbases, sizeof(Base), cmp_bases);

    ans = 0;
    helper(0, -1, 1);

    /* Print the result - ans fits in int64_t */
    int64_t result = (int64_t)ans;
    printf("%lld\n", result);
    return 0;
}
'''

if __name__ == "__main__":
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(C_CODE.encode())
        c_file = f.name
    exe = c_file[:-2]
    try:
        subprocess.run(['gcc', '-O2', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
        result = subprocess.run([exe], capture_output=True, text=True, timeout=280)
        print(result.stdout.strip())
    finally:
        os.unlink(c_file)
        if os.path.exists(exe):
            os.unlink(exe)
