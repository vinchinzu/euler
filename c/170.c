/*
 * Project Euler Problem 170: Largest 0 to 9 pandigital formed by
 * concatenating products.
 *
 * For a 2-digit base, find multipliers whose products (concatenated)
 * use each digit 0-9 exactly once, forming the largest such number.
 */
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define ALL_DIGITS_MASK 0x3FF

static int popcount10(int mask) {
    int c = 0;
    for (int m = mask; m; m >>= 1) c += m & 1;
    return c;
}

/* Digit info: check if number has all unique digits. Return 1 on success. */
static int digit_info(long long number, int *mask_out, int *len_out) {
    if (number <= 0) return 0;
    int mask = 0, length = 0;
    long long n = number;
    while (n > 0) {
        int d = (int)(n % 10);
        int bit = 1 << d;
        if (mask & bit) return 0;
        mask |= bit;
        n /= 10;
        length++;
    }
    *mask_out = mask;
    *len_out = length;
    return 1;
}

/* Generate all numbers of given length using digits from mask (no repeated digits, no leading zero).
 * Stores results in provided arrays. Returns count. Digits tried in descending order. */
typedef struct { int value; int used_mask; } NR;

static int gen_numbers(int mask, int target_len, NR *out, int max_out) {
    /* Use iterative DFS with a stack */
    struct { int depth; int value; int used; } stk[50000];
    int top = 0;
    int count = 0;

    stk[top].depth = 0;
    stk[top].value = 0;
    stk[top].used = 0;
    top++;

    while (top > 0) {
        top--;
        int depth = stk[top].depth;
        int value = stk[top].value;
        int used = stk[top].used;

        if (depth == target_len) {
            if (count < max_out) {
                out[count].value = value;
                out[count].used_mask = used;
                count++;
            }
            continue;
        }

        /* Try digits 0..9 (ascending so that stack reversal gives descending order) */
        for (int digit = 0; digit <= 9; digit++) {
            int bit = 1 << digit;
            if (!(mask & bit)) continue;
            if (used & bit) continue;
            if (depth == 0 && target_len > 1 && digit == 0) continue;
            stk[top].depth = depth + 1;
            stk[top].value = value * 10 + digit;
            stk[top].used = used | bit;
            top++;
        }
    }

    return count;
}

static void num_to_str(long long n, char *buf) {
    if (n == 0) { buf[0] = '0'; buf[1] = '\0'; return; }
    char tmp[20];
    int len = 0;
    while (n > 0) { tmp[len++] = '0' + (int)(n % 10); n /= 10; }
    for (int i = 0; i < len; i++) buf[i] = tmp[len - 1 - i];
    buf[len] = '\0';
}

int main(void) {
    int base_length = 2;
    char best[12] = "";

    NR *bases = malloc(100 * sizeof(NR));
    int nbases = gen_numbers(ALL_DIGITS_MASK, base_length, bases, 100);

    NR *mults1 = malloc(50000 * sizeof(NR));
    NR *mults2 = malloc(50000 * sizeof(NR));

    for (int bi = 0; bi < nbases; bi++) {
        int base = bases[bi].value;
        int base_mask = bases[bi].used_mask;
        if (base == 0) continue;

        int remaining_mask = ALL_DIGITS_MASK ^ base_mask;
        int total_remaining = popcount10(remaining_mask);
        if (total_remaining != 10 - base_length) continue;

        for (int len1 = 1; len1 < total_remaining; len1++) {
            int len2 = total_remaining - len1;
            int expected_len1 = len1 + 1;
            int expected_len2 = len2 + 1;

            int nmults1 = gen_numbers(remaining_mask, len1, mults1, 50000);

            for (int mi = 0; mi < nmults1; mi++) {
                int mult1 = mults1[mi].value;
                int mask1 = mults1[mi].used_mask;
                int next_remaining = remaining_mask ^ mask1;
                if (popcount10(next_remaining) != len2) continue;

                int mask_prod1, len_prod1;
                if (!digit_info((long long)base * mult1, &mask_prod1, &len_prod1)) continue;
                if (len_prod1 != expected_len1) continue;

                int nmults2 = gen_numbers(next_remaining, len2, mults2, 50000);

                for (int mj = 0; mj < nmults2; mj++) {
                    int mult2 = mults2[mj].value;

                    int mask_prod2, len_prod2;
                    if (!digit_info((long long)base * mult2, &mask_prod2, &len_prod2)) continue;
                    if (len_prod2 != expected_len2) continue;
                    if (mask_prod1 & mask_prod2) continue;
                    if ((mask_prod1 | mask_prod2) != ALL_DIGITS_MASK) continue;

                    char s1[12], s2[12], candidate[22];
                    num_to_str((long long)base * mult1, s1);
                    num_to_str((long long)base * mult2, s2);
                    strcpy(candidate, s1);
                    strcat(candidate, s2);

                    if (best[0] == '\0' || strcmp(candidate, best) > 0)
                        strcpy(best, candidate);
                }
            }
        }
    }

    printf("%s\n", best);

    free(bases);
    free(mults1);
    free(mults2);
    return 0;
}
