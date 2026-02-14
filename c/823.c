/*
 * Project Euler 823: Factor Shuffle
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 10000
#define MOD 1234567891LL
#define MAX_FACTORS 40000
#define MAX_NUMBERS 40000

typedef struct {
    int prime;
    int tiebreak;
} Factor;

typedef struct {
    long long k_mod;
    int tiebreak;
    int prime;
} FactorInfo;

static int cmp_desc(const void *a, const void *b) {
    const Factor *fa = (const Factor *)a;
    const Factor *fb = (const Factor *)b;
    if (fa->prime != fb->prime) return fb->prime - fa->prime;
    return fb->tiebreak - fa->tiebreak;
}

static int cmp_finfo(const void *a, const void *b) {
    const FactorInfo *fa = (const FactorInfo *)a;
    const FactorInfo *fb = (const FactorInfo *)b;
    if (fa->k_mod < fb->k_mod) return -1;
    if (fa->k_mod > fb->k_mod) return 1;
    return 0;
}

typedef struct {
    Factor *factors;
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

static Number numbers[MAX_NUMBERS];
static int num_numbers = 0;

static Factor collected[MAX_NUMBERS];
static Number new_numbers[MAX_NUMBERS];

static void do_shuffle(void) {
    int collected_count = 0;
    int new_count = 0;

    for (int i = 0; i < num_numbers; i++) {
        Number *num = &numbers[i];
        if (num->len > 0) {
            collected[collected_count++] = num->factors[num->len - 1];
            num->len--;
            if (num->len > 0) {
                new_numbers[new_count++] = *num;
            } else {
                free(num->factors);
            }
        }
    }

    qsort(collected, collected_count, sizeof(Factor), cmp_desc);

    memmove(&new_numbers[1], &new_numbers[0], new_count * sizeof(Number));

    new_numbers[0].factors = (Factor *)malloc(collected_count * sizeof(Factor));
    memcpy(new_numbers[0].factors, collected, collected_count * sizeof(Factor));
    new_numbers[0].len = collected_count;
    new_numbers[0].cap = collected_count;
    new_count++;

    memcpy(numbers, new_numbers, new_count * sizeof(Number));
    num_numbers = new_count;
}

int main(void) {
    long long K = 10000000000000000LL;

    compute_spf();

    int total_factors = 0;
    int temp_factors[20];

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

        qsort(num->factors, fc, sizeof(Factor), cmp_desc);

        num_numbers++;
        total_factors += fc;
    }

    long long shuffle_rounds = 2LL * total_factors;
    for (long long r = 0; r < shuffle_rounds; r++) {
        do_shuffle();
    }

    long long max_k_mod = 0;

    FactorInfo *finfo = (FactorInfo *)malloc(total_factors * sizeof(FactorInfo));
    int fi_count = 0;

    for (int pos = 0; pos < num_numbers; pos++) {
        Number *num = &numbers[pos];
        for (int j = 0; j < num->len; j++) {
            long long period = (long long)pos + num->len - j;
            long long remaining = K - shuffle_rounds;
            long long km = remaining % period;
            if (km < 0) km += period;

            finfo[fi_count].k_mod = km;
            finfo[fi_count].tiebreak = num->factors[j].tiebreak;
            finfo[fi_count].prime = num->factors[j].prime;
            fi_count++;

            if (km > max_k_mod) max_k_mod = km;
        }
    }

    qsort(finfo, fi_count, sizeof(FactorInfo), cmp_finfo);

    long long *products = (long long *)calloc(MAX_NUMBERS, sizeof(long long));
    int max_tiebreak = 15 * N + N + 1;
    int *tb_to_numidx = (int *)malloc(max_tiebreak * sizeof(int));
    memset(tb_to_numidx, -1, max_tiebreak * sizeof(int));

    for (int i = 0; i < num_numbers; i++) {
        for (int j = 0; j < numbers[i].len; j++) {
            tb_to_numidx[numbers[i].factors[j].tiebreak] = i;
        }
    }

    int fi_ptr = 0;
    long long answer = 0;

    for (long long k = 0; k <= max_k_mod; k++) {
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

        if (fi_ptr >= fi_count) break;

        if (k < max_k_mod) {
            do_shuffle();

            memset(tb_to_numidx, -1, max_tiebreak * sizeof(int));
            for (int i = 0; i < num_numbers; i++) {
                for (int j = 0; j < numbers[i].len; j++) {
                    tb_to_numidx[numbers[i].factors[j].tiebreak] = i;
                }
            }
        }
    }

    for (int i = 0; i < MAX_NUMBERS; i++) {
        if (products[i] > 0) {
            answer = (answer + products[i]) % MOD;
        }
    }

    printf("%lld\n", answer);

    free(finfo);
    free(products);
    free(tb_to_numidx);
    for (int i = 0; i < num_numbers; i++) {
        free(numbers[i].factors);
    }

    return 0;
}
