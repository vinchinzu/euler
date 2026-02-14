"""Project Euler Problem 311: Biclinic Integral Quadrilaterals

Find count of quadrilaterals ABCD with 1 <= AB < BC < CD < AD,
where AB^2 + BC^2 + CD^2 + AD^2 <= 10^10.

Algorithm:
For each n that can be expressed as sum of two squares, count r2(n)
representations, then sum C(r2(n)/2, 3) weighted by 3mod4 square products.

Uses embedded C for performance.
"""
import subprocess, os, tempfile


def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

/*
 * Problem 311 - Biclinic Integral Quadrilaterals
 *
 * N = 10^10, L = N/4, L2 = L / (5*5*13)
 *
 * 1) Sieve primes up to L/(5*5) = 10^8 for primes ≡ 1 (mod 4)
 * 2) Sieve primes up to sqrt(L2) ≈ 2774 for primes ≡ 3 (mod 4)
 * 3) helper1: generate all products of squares of 3mod4 primes <= L2
 * 4) Build prefix sum array C over num3mod4_prods
 * 5) helper2: enumerate products of 1mod4 primes * powers of 2,
 *    accumulate C(r2_contrib, 3) * C[L/n]
 */

#define SIEVE1_LIMIT 100000000   /* 10^8 */
#define SIEVE3_LIMIT 2775        /* ceil(sqrt(L2)) + 1 */

static long long L;
static int L2;

/* Bit-packed sieve for primes up to SIEVE1_LIMIT */
static unsigned char *sieve_bits;  /* bit i represents number i */

static int get_bit(int i) {
    return (sieve_bits[i >> 3] >> (i & 7)) & 1;
}
static void clear_bit(int i) {
    sieve_bits[i >> 3] &= ~(1 << (i & 7));
}

/* Primes arrays */
static int *primes1mod4;
static int n_primes1;

static int *primes3mod4;
static int n_primes3;

/* num3mod4_prods and prefix sum C */
static int *num3mod4_prods;
static int *cumC;

/* Global answer */
static long long ans;

void sieve_and_extract(void) {
    int limit1 = SIEVE1_LIMIT;
    int bytes = (limit1 >> 3) + 1;
    sieve_bits = (unsigned char *)malloc(bytes);
    if (!sieve_bits) { fprintf(stderr, "sieve alloc fail\n"); exit(1); }
    memset(sieve_bits, 0xFF, bytes);
    /* 0 and 1 are not prime */
    clear_bit(0);
    clear_bit(1);

    int sq = (int)sqrt((double)limit1);
    for (int i = 2; i <= sq; i++) {
        if (get_bit(i)) {
            for (long long j = (long long)i * i; j <= limit1; j += i) {
                clear_bit((int)j);
            }
        }
    }

    /* Count primes ≡ 1 mod 4 up to limit1 */
    /* First pass: count */
    int count1 = 0;
    for (int i = 5; i <= limit1; i += 4) {
        if (get_bit(i)) count1++;
    }
    primes1mod4 = (int *)malloc(count1 * sizeof(int));
    n_primes1 = 0;
    for (int i = 5; i <= limit1; i += 4) {
        if (get_bit(i)) primes1mod4[n_primes1++] = i;
    }

    /* Primes ≡ 3 mod 4 up to SIEVE3_LIMIT */
    int count3 = 0;
    for (int i = 3; i <= SIEVE3_LIMIT; i += 4) {
        if (i <= limit1 && get_bit(i)) count3++;
    }
    primes3mod4 = (int *)malloc(count3 * sizeof(int));
    n_primes3 = 0;
    for (int i = 3; i <= SIEVE3_LIMIT; i += 4) {
        if (i <= limit1 && get_bit(i)) primes3mod4[n_primes3++] = i;
    }

    /* Free sieve */
    free(sieve_bits);
    sieve_bits = NULL;
}

/* helper1: generate all products of squares of primes3mod4 <= L2 */
void helper1(int min_idx, long long n) {
    num3mod4_prods[(int)n] += 1;
    for (int idx = min_idx; idx < n_primes3; idx++) {
        long long p = primes3mod4[idx];
        long long p2 = p * p;
        if (n * p2 > (long long)L2) return;
        long long new_n = n;
        while (new_n * p2 <= (long long)L2) {
            new_n *= p2;
            helper1(idx + 1, new_n);
        }
    }
}

/* helper2: enumerate products of primes1mod4, accumulate answer */
void helper2(int min_idx, long long n, int a0, int b) {
    /* b = product of (exponent_i + 1) for primes ≡ 1 mod 4 included so far */
    if (b >= 5) {
        int r2_contrib = (b + (a0 % 2 == 1 ? 1 : 0)) / 2;
        if (r2_contrib >= 3) {
            long long ways = (long long)r2_contrib * (r2_contrib - 1) * (r2_contrib - 2) / 6;
            int idx = (int)(L / n);
            if (idx <= L2) {
                ans += (long long)cumC[idx] * ways;
            }
        }
    }

    if (n > L) return;

    /* Calculate limit for next prime */
    double dlimit = (double)L / (double)n;
    if (b == 1) {
        /* Need at least 3 more prime factors (b needs to reach 5+),
           so minimum is p^4 (giving b*5), hence p <= (L/n)^(1/4)...
           Actually the original uses (L/n)^(1/3) */
        dlimit = cbrt(dlimit);
    } else if (b == 2) {
        dlimit = sqrt(dlimit);
    }
    /* Add small epsilon to handle floating point edge cases */
    long long ilimit = (long long)(dlimit + 0.5);

    for (int idx = min_idx; idx < n_primes1; idx++) {
        long long p = primes1mod4[idx];
        if (p > ilimit) return;

        int e = 1;
        long long new_n = n;
        while (new_n <= L / p) {  /* safe: avoids overflow by checking new_n <= L/p */
            new_n *= p;
            helper2(idx + 1, new_n, a0, b * (e + 1));
            e++;
        }
    }
}

int main(void) {
    long long N = 10000000000LL;  /* 10^10 */
    L = N / 4;                     /* 2500000000 */
    L2 = (int)(L / (5LL * 5 * 13)); /* 7692307 */

    sieve_and_extract();

    /* Allocate arrays for helper1 */
    num3mod4_prods = (int *)calloc(L2 + 1, sizeof(int));
    cumC = (int *)calloc(L2 + 1, sizeof(int));
    if (!num3mod4_prods || !cumC) {
        fprintf(stderr, "array alloc fail\n");
        return 1;
    }

    /* Generate all products of squares of 3mod4 primes */
    helper1(0, 1);

    /* Build prefix sum */
    cumC[0] = num3mod4_prods[0];
    for (int i = 1; i <= L2; i++) {
        cumC[i] = cumC[i - 1] + num3mod4_prods[i];
    }

    /* Free num3mod4_prods - no longer needed */
    free(num3mod4_prods);
    num3mod4_prods = NULL;

    /* Enumerate powers of 2 and call helper2 */
    ans = 0;
    int a0 = 0;
    long long prod = 1;
    while (prod <= L) {
        helper2(0, prod, a0, 1);
        a0++;
        prod *= 2;
    }

    printf("%lld\n", ans);

    free(cumC);
    free(primes1mod4);
    free(primes3mod4);
    return 0;
}
"""
    tmpdir = tempfile.mkdtemp()
    src = os.path.join(tmpdir, "sol311.c")
    exe = os.path.join(tmpdir, "sol311")
    with open(src, 'w') as f:
        f.write(c_code)
    subprocess.run(["gcc", "-O2", "-o", exe, src, "-lm"], check=True, capture_output=True)
    result = subprocess.run([exe], capture_output=True, text=True, check=True, timeout=280)
    print(result.stdout.strip())


if __name__ == "__main__":
    solve()
