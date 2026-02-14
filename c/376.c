/*
 * Project Euler Problem 376: Nontransitive sets of dice
 *
 * Count nontransitive sets of 3 dice, each with K=6 sides, max N=30 pips.
 * Each pair beats the next cyclically (probability > 1/2).
 *
 * Algorithm:
 * Recurse over pip values in increasing order. For each pip value, choose how
 * many sides of each die get that value. Track losing rolls between pairs.
 * At the base case, multiply by C(N, maxPip).
 * Divide by 3 to remove cyclic triple-counting.
 */

#include <stdio.h>
#include <string.h>

typedef long long ll;

#define N_PIPS 30
#define K_SIDES 6
#define HALF_K_SQ 18  /* K*K/2 */

/* Precompute binomial coefficients */
static ll binom[N_PIPS + 2][N_PIPS + 2];

static void init_binom(void) {
    for (int i = 0; i <= N_PIPS + 1; i++) {
        binom[i][0] = 1;
        for (int j = 1; j <= i; j++)
            binom[i][j] = binom[i-1][j-1] + binom[i-1][j];
    }
}

/* Memoization using hash table */
#define MEMO_SIZE (1 << 22)
#define MEMO_MASK (MEMO_SIZE - 1)

typedef struct {
    unsigned long long key;
    ll value;
    int occupied;
} MemoEntry;

static MemoEntry memo[MEMO_SIZE];

static unsigned long long pack_state(int max_pip, int r1, int r2, int r3, int l1, int l2, int l3) {
    return ((unsigned long long)max_pip << 40) |
           ((unsigned long long)r1 << 34) |
           ((unsigned long long)r2 << 28) |
           ((unsigned long long)r3 << 22) |
           ((unsigned long long)l1 << 11) |
           ((unsigned long long)l2 << 5) |
           ((unsigned long long)l3);
}

static int memo_get(unsigned long long key, ll *val) {
    unsigned int idx = (unsigned int)(key * 0x9E3779B97F4A7C15ULL >> 42) & MEMO_MASK;
    for (int probe = 0; probe < 64; probe++) {
        unsigned int i = (idx + probe) & MEMO_MASK;
        if (!memo[i].occupied) return 0;
        if (memo[i].key == key) { *val = memo[i].value; return 1; }
    }
    return 0;
}

static void memo_put(unsigned long long key, ll val) {
    unsigned int idx = (unsigned int)(key * 0x9E3779B97F4A7C15ULL >> 42) & MEMO_MASK;
    for (int probe = 0; probe < 64; probe++) {
        unsigned int i = (idx + probe) & MEMO_MASK;
        if (!memo[i].occupied || memo[i].key == key) {
            memo[i].key = key;
            memo[i].value = val;
            memo[i].occupied = 1;
            return;
        }
    }
    /* Overwrite on collision */
    memo[idx].key = key;
    memo[idx].value = val;
    memo[idx].occupied = 1;
}

static ll helper(int max_pip, int rem1, int rem2, int rem3, int loss1, int loss2, int loss3) {
    if (loss1 >= HALF_K_SQ || loss2 >= HALF_K_SQ || loss3 >= HALF_K_SQ)
        return 0;
    if (rem1 == 0 && rem2 == 0 && rem3 == 0)
        return binom[N_PIPS][max_pip];

    unsigned long long key = pack_state(max_pip, rem1, rem2, rem3, loss1, loss2, loss3);
    ll cached;
    if (memo_get(key, &cached)) return cached;

    ll total = 0;
    for (int s1 = 0; s1 <= rem1; s1++) {
        for (int s2 = 0; s2 <= rem2; s2++) {
            for (int s3 = 0; s3 <= rem3; s3++) {
                if (s1 + s2 + s3 > 0) {
                    total += helper(
                        max_pip + 1,
                        rem1 - s1, rem2 - s2, rem3 - s3,
                        loss1 + rem1 * s2,
                        loss2 + rem2 * s3,
                        loss3 + rem3 * s1
                    );
                }
            }
        }
    }

    memo_put(key, total);
    return total;
}

int main(void) {
    init_binom();
    memset(memo, 0, sizeof(memo));

    ll result = helper(0, K_SIDES, K_SIDES, K_SIDES, 0, 0, 0) / 3;
    printf("%lld\n", result);
    return 0;
}
