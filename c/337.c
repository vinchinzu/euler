/*
 * Project Euler Problem 337 - Totient Stairstep Sequences
 * Extracted from embedded C in Python solution.
 *
 * S(N) counts valid sequences {a_1, ..., a_n} with a_1=6,
 * phi(a_i) < phi(a_{i+1}) < a_i < a_{i+1}, a_n <= N.
 *
 * Uses sieve for phi, sort by (phi, value), DP with Fenwick tree.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define TARGET_N 20000000
#define MOD 100000000
#define START 6

static int phi_arr[TARGET_N + 1];
static int primes[2000000];
static int nprimes = 0;
static char is_composite[TARGET_N + 1];

/* Fenwick tree */
static long long bit[TARGET_N + 2];

void bit_update(int idx, long long val) {
    idx++; /* 1-indexed */
    while (idx <= TARGET_N + 1) {
        bit[idx] = (bit[idx] + val) % MOD;
        idx += idx & (-idx);
    }
}

long long bit_query(int idx) {
    idx++; /* 1-indexed */
    long long s = 0;
    while (idx > 0) {
        s += bit[idx];
        idx -= idx & (-idx);
    }
    return s % MOD;
}

long long bit_range(int l, int r) {
    if (l > r) return 0;
    long long res = bit_query(r);
    if (l > 0) res -= bit_query(l - 1);
    return ((res % MOD) + MOD) % MOD;
}

void sieve_phi(void) {
    for (int i = 0; i <= TARGET_N; i++) phi_arr[i] = i;
    memset(is_composite, 0, sizeof(is_composite));
    /* Linear sieve for phi */
    phi_arr[0] = 0;
    phi_arr[1] = 1;
    for (int i = 2; i <= TARGET_N; i++) {
        if (!is_composite[i]) {
            primes[nprimes++] = i;
            phi_arr[i] = i - 1;
        }
        for (int j = 0; j < nprimes; j++) {
            long long x = (long long)i * primes[j];
            if (x > TARGET_N) break;
            is_composite[x] = 1;
            if (i % primes[j] == 0) {
                phi_arr[x] = phi_arr[i] * primes[j];
                break;
            }
            phi_arr[x] = phi_arr[i] * (primes[j] - 1);
        }
    }
}

/* Pair (phi, index) for sorting */
typedef struct {
    int phi_val;
    int idx;
} Pair;

static Pair *pairs;
static long long *dp;

int pair_cmp(const void *a, const void *b) {
    const Pair *pa = (const Pair *)a;
    const Pair *pb = (const Pair *)b;
    if (pa->phi_val != pb->phi_val) return pa->phi_val - pb->phi_val;
    return pa->idx - pb->idx;
}

int main(void) {
    sieve_phi();

    int count = TARGET_N - START + 1;
    pairs = (Pair *)malloc(count * sizeof(Pair));
    dp = (long long *)calloc(TARGET_N + 1, sizeof(long long));

    for (int i = START; i <= TARGET_N; i++) {
        pairs[i - START].phi_val = phi_arr[i];
        pairs[i - START].idx = i;
    }

    qsort(pairs, count, sizeof(Pair), pair_cmp);

    memset(bit, 0, sizeof(bit));

    long long total = 0;
    int pos = 0;

    /* Process groups with the same phi value */
    while (pos < count) {
        int cur_phi = pairs[pos].phi_val;

        /* Collect all indices with this phi */
        int group_start = pos;
        while (pos < count && pairs[pos].phi_val == cur_phi) pos++;
        int group_end = pos;

        /* Compute dp for each element in the group */
        for (int g = group_start; g < group_end; g++) {
            int j = pairs[g].idx;
            int left = START;
            if (cur_phi + 1 > left) left = cur_phi + 1;
            int right = j - 1;

            long long sum_prev = bit_range(left, right);
            long long base = (j == START) ? 1 : 0;
            long long value = (base + sum_prev) % MOD;
            dp[j] = value;
            total = (total + value) % MOD;
        }

        /* Update BIT with the computed dp values */
        for (int g = group_start; g < group_end; g++) {
            int j = pairs[g].idx;
            bit_update(j, dp[j]);
        }
    }

    printf("%lld\n", total);

    free(pairs);
    free(dp);
    return 0;
}
