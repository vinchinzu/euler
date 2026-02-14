/*
 * Project Euler 524 - First Sort II
 *
 * Find the minimum Q(n, N) over all n, where Q(n, k) is the
 * lexicographic index of the first permutation of n elements
 * requiring exactly k steps. N = 12^12.
 *
 * Key insight: the number of steps for a permutation relates
 * to the positions of elements via binary representation of N.
 */
#include <stdio.h>
#include <stdint.h>
#include <string.h>

typedef unsigned long long ull;
typedef __int128 u128;

/* N = 12^12 */
static ull compute_N(void) {
    ull r = 1;
    for (int i = 0; i < 12; i++) r *= 12;
    return r;
}

static int ilog2(ull n) {
    int b = 0;
    while ((1ULL << (b + 1)) <= n) b++;
    return b;
}

/* Factorial (fits in ull for small values) */
static ull factorial(int n) {
    ull r = 1;
    for (int i = 2; i <= n; i++) r *= i;
    return r;
}

#define MAXL 50

static ull answer;
static int L_val;

/* ranks: sorted list of set bit positions in N */
static int ranks_store[MAXL];
static int ranks_len;

/* Helper: DFS to find first valid permutation */
static void helper(int *remaining, int rem_len, int *rks, int rks_len, ull order_index) {
    if (answer != 0) return;

    if (rem_len == 0) {
        if (rks_len == 0) {
            answer = order_index;
        }
        return;
    }

    /* Pruning */
    int check = rem_len < rks_len ? rem_len : rks_len;
    for (int i = 0; i < check; i++) {
        if (remaining[i] > rks[i] + i) return;
    }

    for (int i = 0; i < rem_len; i++) {
        if (answer != 0) break;

        int el = remaining[i];
        int rank = el - i;

        /* Find rank in rks */
        int bit_index = -1;
        for (int j = 0; j < rks_len; j++) {
            if (rks[j] == rank) { bit_index = j; break; }
        }

        int target_rank = L_val - (rem_len - 1);
        if (rank != target_rank) {
            if (bit_index < 0) continue;
            /* Remove rks[bit_index] */
            int saved_rk = rks[bit_index];
            for (int j = bit_index; j < rks_len - 1; j++) rks[j] = rks[j + 1];
            /* Remove remaining[i] */
            for (int j = i; j < rem_len - 1; j++) remaining[j] = remaining[j + 1];

            helper(remaining, rem_len - 1, rks, rks_len - 1,
                   order_index + (ull)i * factorial(rem_len - 1));

            /* Restore remaining */
            for (int j = rem_len - 1; j > i; j--) remaining[j] = remaining[j - 1];
            remaining[i] = el;
            /* Restore rks */
            for (int j = rks_len - 1; j > bit_index; j--) rks[j] = rks[j - 1];
            rks[bit_index] = saved_rk;
        } else {
            /* Remove remaining[i] */
            for (int j = i; j < rem_len - 1; j++) remaining[j] = remaining[j + 1];

            helper(remaining, rem_len - 1, rks, rks_len,
                   order_index + (ull)i * factorial(rem_len - 1));

            /* Restore remaining */
            for (int j = rem_len - 1; j > i; j--) remaining[j] = remaining[j - 1];
            remaining[i] = el;
        }
    }
}

int main(void) {
    ull N = compute_N();
    L_val = ilog2(N) + 1;

    /* Collect set bit positions */
    ranks_len = 0;
    for (int i = 0; i <= L_val; i++) {
        if (N & (1ULL << i)) {
            ranks_store[ranks_len++] = i;
        }
    }

    int remaining[MAXL];
    for (int i = 0; i <= L_val; i++) remaining[i] = i;

    int rks[MAXL];
    memcpy(rks, ranks_store, ranks_len * sizeof(int));

    answer = 0;
    helper(remaining, L_val + 1, rks, ranks_len, 1);

    printf("%llu\n", (unsigned long long)answer);
    return 0;
}
