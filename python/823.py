"""Project Euler Problem 823: Factor Shuffle.

A list starts with [2,3,...,n]. Each round: divide each number by its smallest
prime factor, collect all those SPFs into a product (new number), remove 1s.
Find S(10^4, 10^16) mod 1234567891.

Key insight: Track each prime factor individually with a tiebreak value.
After 2*num_factors rounds, compute periods analytically.
Each factor's period = position_in_number + numbers_with_more_factors.

Uses embedded C for performance.
"""
import subprocess
import os
import tempfile


def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 10000
#define MOD 1234567891LL
#define MAX_FACTORS 40000   /* ~36000 total prime factors for 2..10000 */
#define MAX_NUMBERS 40000   /* max number of "numbers" at any time */

typedef struct {
    int prime;
    int tiebreak;
} Factor;

typedef struct {
    long long k_mod;
    int tiebreak;
    int prime;
} FactorInfo;

/* Compare factors descending by (prime, tiebreak) */
static int cmp_desc(const void *a, const void *b) {
    const Factor *fa = (const Factor *)a;
    const Factor *fb = (const Factor *)b;
    if (fa->prime != fb->prime) return fb->prime - fa->prime;
    return fb->tiebreak - fa->tiebreak;
}

/* Compare FactorInfo ascending by k_mod */
static int cmp_finfo(const void *a, const void *b) {
    const FactorInfo *fa = (const FactorInfo *)a;
    const FactorInfo *fb = (const FactorInfo *)b;
    if (fa->k_mod < fb->k_mod) return -1;
    if (fa->k_mod > fb->k_mod) return 1;
    return 0;
}

/* Each "number" in the list owns a contiguous block of factors */
typedef struct {
    Factor *factors;  /* dynamically allocated array */
    int len;
    int cap;
} Number;

static int spf[N + 1];

static void compute_spf(void) {
    for (int i = 0; i <= N; i++) spf[i] = i;
    for (int i = 2; (long long)i * i <= N; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j <= N; j += i) {
                if (spf[j] == j) spf[j] = i;
            }
        }
    }
}

/* Global arrays for numbers */
static Number numbers[MAX_NUMBERS];
static int num_numbers = 0;

/* Temporary buffer for collected factors during shuffle */
static Factor collected[MAX_NUMBERS];

/* Temporary buffer for new numbers array */
static Number new_numbers[MAX_NUMBERS];

static void do_shuffle(void) {
    int collected_count = 0;
    int new_count = 0;

    for (int i = 0; i < num_numbers; i++) {
        Number *num = &numbers[i];
        if (num->len > 0) {
            /* Pop the last factor (smallest in descending order) */
            collected[collected_count++] = num->factors[num->len - 1];
            num->len--;
            if (num->len > 0) {
                new_numbers[new_count++] = *num;
            } else {
                /* Free empty number */
                free(num->factors);
            }
        }
    }

    /* Sort collected factors descending */
    qsort(collected, collected_count, sizeof(Factor), cmp_desc);

    /* Create new number from collected factors */
    /* Shift new_numbers right by 1 to insert at front */
    memmove(&new_numbers[1], &new_numbers[0], new_count * sizeof(Number));

    /* Allocate new number */
    new_numbers[0].factors = (Factor *)malloc(collected_count * sizeof(Factor));
    memcpy(new_numbers[0].factors, collected, collected_count * sizeof(Factor));
    new_numbers[0].len = collected_count;
    new_numbers[0].cap = collected_count;
    new_count++;

    /* Copy back */
    memcpy(numbers, new_numbers, new_count * sizeof(Number));
    num_numbers = new_count;
}

int main(void) {
    long long K = 10000000000000000LL; /* 10^16 */

    compute_spf();

    /* Initialize: factor each number 2..N */
    int total_factors = 0;
    int temp_factors[20]; /* max factors for a number <= 10000 is ~13 */

    for (int n = 2; n <= N; n++) {
        int fc = 0;
        int x = n;
        while (x > 1) {
            int p = spf[x];
            temp_factors[fc++] = p;
            x /= p;
        }

        Number *num = &numbers[num_numbers];
        num->factors = (Factor *)malloc(fc * sizeof(Factor));
        num->len = fc;
        num->cap = fc;

        for (int i = 0; i < fc; i++) {
            num->factors[i].prime = temp_factors[i];
            num->factors[i].tiebreak = i * N + n;
        }

        /* Sort descending by (prime, tiebreak) */
        qsort(num->factors, fc, sizeof(Factor), cmp_desc);

        num_numbers++;
        total_factors += fc;
    }

    /* Shuffle for 2 * total_factors rounds */
    long long shuffle_rounds = 2LL * total_factors;
    for (long long r = 0; r < shuffle_rounds; r++) {
        do_shuffle();
    }

    /* Compute period for each factor and determine (K - shuffle_rounds) % period */
    /* We need to find the max k_mod value to know how many additional shuffles */

    /* First pass: compute all k_mod values and find max */
    /* Store factor info: for each factor, record its current position, its index
       within its number, and its period */

    /* We'll store: factor_id (index in flat enumeration), k_mod, current number index, prime */
    /* But we need to match factors after additional shuffles. Instead, follow the
       Python approach: for each k_mod, record which factors (by identity) need to be
       snapshotted at that shuffle step. */

    /* Approach: assign each factor a unique ID based on tiebreak.
       After computing k_mod for each factor, group by k_mod.
       Then simulate additional shuffles, and at each step k, snapshot factors
       whose k_mod == k. */

    /* Find max k_mod and allocate per-k_mod lists */
    long long max_k_mod = 0;

    /* Store (k_mod, tiebreak, prime) for each factor */
    FactorInfo *finfo = (FactorInfo *)malloc(total_factors * sizeof(FactorInfo));
    int fi_count = 0;

    for (int pos = 0; pos < num_numbers; pos++) {
        Number *num = &numbers[pos];
        for (int j = 0; j < num->len; j++) {
            long long period = (long long)pos + num->len - j;
            long long remaining = K - shuffle_rounds;
            long long km = remaining % period;
            if (km < 0) km += period; /* shouldn't happen since K > shuffle_rounds */

            finfo[fi_count].k_mod = km;
            finfo[fi_count].tiebreak = num->factors[j].tiebreak;
            finfo[fi_count].prime = num->factors[j].prime;
            fi_count++;

            if (km > max_k_mod) max_k_mod = km;
        }
    }

    /* Build a lookup: for each k_mod value, list of (tiebreak, prime) */
    /* Use a hash map or sorted array approach */
    /* Since max_k_mod could be large, use a sorted list and step through */

    /* Sort finfo by k_mod */
    qsort(finfo, fi_count, sizeof(FactorInfo), cmp_finfo);

    /* Now simulate additional shuffles from k=0 to max_k_mod */
    /* At each step k, check if any factors have k_mod == k */
    /* For those factors, find them in the current state and record their number's position */

    /* For the final answer: we need to know which prime factors end up in which number.
       Each factor with k_mod == k: at step k, it's in some number at some position.
       We record which number (by position index at step k) it belongs to, and its prime value. */

    /* Use a map from number_position -> product of primes */
    /* Since positions can be up to MAX_NUMBERS, use a simple array */
    long long *products = (long long *)calloc(MAX_NUMBERS, sizeof(long long));
    /* Initialize to 1 for positions that get any factor, 0 for empty */
    /* We'll use 0 to mean "no factors assigned yet" and handle the product manually */

    /* We also need a way to find a factor by tiebreak in the current state.
       Build a hash map: tiebreak -> (number_index, factor_index_in_number) */
    /* tiebreak values range from 0*N+2 to 13*N+N = ~130000 */
    /* Use a direct array indexed by tiebreak */
    int max_tiebreak = 15 * N + N + 1;
    int *tb_to_numidx = (int *)malloc(max_tiebreak * sizeof(int));
    memset(tb_to_numidx, -1, max_tiebreak * sizeof(int));

    /* Build initial tiebreak -> number_index map */
    for (int i = 0; i < num_numbers; i++) {
        for (int j = 0; j < numbers[i].len; j++) {
            tb_to_numidx[numbers[i].factors[j].tiebreak] = i;
        }
    }

    /* Process additional shuffles */
    int fi_ptr = 0; /* pointer into sorted finfo array */
    long long answer = 0;

    for (long long k = 0; k <= max_k_mod; k++) {
        /* Snapshot factors whose k_mod == k */
        while (fi_ptr < fi_count && finfo[fi_ptr].k_mod == k) {
            int tb = finfo[fi_ptr].tiebreak;
            int prime = finfo[fi_ptr].prime;
            int numidx = tb_to_numidx[tb];

            if (numidx >= 0) {
                if (products[numidx] == 0) {
                    products[numidx] = prime;
                } else {
                    products[numidx] = (products[numidx] * prime) % MOD;
                }
            }
            fi_ptr++;
        }

        /* If we've processed all factors, we can stop early */
        if (fi_ptr >= fi_count) break;

        /* Do one more shuffle if k < max_k_mod */
        if (k < max_k_mod) {
            /* Update tiebreak map before shuffle */
            /* Clear old mappings for factors that will move */
            /* Actually, we need to rebuild after shuffle */

            do_shuffle();

            /* Rebuild tiebreak -> number_index map */
            memset(tb_to_numidx, -1, max_tiebreak * sizeof(int));
            for (int i = 0; i < num_numbers; i++) {
                for (int j = 0; j < numbers[i].len; j++) {
                    tb_to_numidx[numbers[i].factors[j].tiebreak] = i;
                }
            }
        }
    }

    /* Sum all products */
    for (int i = 0; i < MAX_NUMBERS; i++) {
        if (products[i] > 0) {
            answer = (answer + products[i]) % MOD;
        }
    }

    printf("%lld\n", answer);

    /* Cleanup */
    free(finfo);
    free(products);
    free(tb_to_numidx);
    for (int i = 0; i < num_numbers; i++) {
        free(numbers[i].factors);
    }

    return 0;
}
"""
    tmpdir = tempfile.mkdtemp()
    src = os.path.join(tmpdir, "sol823.c")
    exe = os.path.join(tmpdir, "sol823")
    with open(src, 'w') as f:
        f.write(c_code)
    subprocess.run(
        ["gcc", "-O2", "-o", exe, src, "-lm"],
        check=True, capture_output=True
    )
    result = subprocess.run(
        [exe], capture_output=True, text=True, check=True, timeout=280
    )
    print(result.stdout.strip())


if __name__ == "__main__":
    solve()
