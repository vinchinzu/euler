/*
 * Project Euler Problem 240: Top Dice
 *
 * 20 twelve-sided dice, top 10 values sum to 70.
 * Count the number of ways.
 *
 * Enumerate which distinct die values appear among the top 10,
 * then count multinomial arrangements and lower dice possibilities.
 */
#include <stdio.h>

#define T 70
#define NN 20
#define K 10
#define S 12

static long long n_cr(int n, int k) {
    if (k < 0 || k > n) return 0;
    if (k == 0 || k == n) return 1;
    if (k > n - k) k = n - k;
    long long r = 1;
    for (int i = 0; i < k; i++)
        r = r * (n - i) / (i + 1);
    return r;
}

static long long pow_int(int base, int exp) {
    long long r = 1;
    for (int i = 0; i < exp; i++) r *= base;
    return r;
}

static long long ans = 0;

/* dice_vals: array of (value, count) pairs for the top K dice */
/* We enumerate in increasing order of die value */
static int dice_vals[K]; /* values */
static int dice_cnts[K]; /* counts */
static int num_distinct;

static void helper(int min_die, int num_dice, int sum_val) {
    if (num_dice == K) {
        if (sum_val != T) return;
        int lowest_die = dice_vals[0];

        /* The remaining NN-K dice each show a value in [1, lowest_die] */
        /* But some might show exactly lowest_die */
        /* For i extra dice showing lowest_die (i = 0..NN-K): */
        for (int i = 0; i <= NN - K; i++) {
            int num_dice_remaining = NN;
            long long num_ways = 1;

            for (int d = 0; d < num_distinct; d++) {
                int count = dice_cnts[d];
                if (d == 0) count += i; /* add extra lowest_die dice */
                num_ways *= n_cr(num_dice_remaining, count);
                num_dice_remaining -= count;
            }

            /* Remaining dice show values in [1, lowest_die - 1] */
            num_ways *= pow_int(lowest_die - 1, num_dice_remaining);
            ans += num_ways;
        }
        return;
    }

    for (int die = min_die; die <= S; die++) {
        for (int count = 1; count <= K - num_dice; count++) {
            if (sum_val + count * die > T) break;
            dice_vals[num_distinct] = die;
            dice_cnts[num_distinct] = count;
            num_distinct++;
            helper(die + 1, num_dice + count, sum_val + count * die);
            num_distinct--;
        }
    }
}

int main(void) {
    num_distinct = 0;
    helper(1, 0, 0);
    printf("%lld\n", ans);
    return 0;
}
