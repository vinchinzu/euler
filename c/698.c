/*
 * Project Euler 698 - 123 Numbers
 *
 * A "123 number" uses only digits 1, 2, 3, and the count of each digit
 * is itself a 123 number (or 0). Find the N-th such number mod 123123123.
 *
 * Valid digit counts per digit are from the set NUMS.
 * For each length, enumerate valid (c1, c2, c3) triples, count
 * permutations, and build digit-by-digit.
 */
#include <stdio.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define M 123123123

/* Valid counts for each digit: 0 or any 123-number */
static int NUMS[] = {0, 1, 2, 3, 11, 12, 13, 21, 22, 23, 31, 32, 33};
static int NUM_COUNT = 13;

/* Multinomial coefficient for counts[0..2] */
static ll multinomial(int c1, int c2, int c3) {
    int total = c1 + c2 + c3;
    ll result = 1;
    /* C(total, c1) * C(total-c1, c2) */
    for (int i = 0; i < c1; i++) {
        result = result * (total - i) / (i + 1);
    }
    for (int i = 0; i < c2; i++) {
        result = result * (total - c1 - i) / (i + 1);
    }
    return result;
}

/* Count permutations of remaining digits with given counts */
static ll count_perms(int counts[3], int remaining) {
    if (remaining == 0) {
        return (counts[0] == 0 && counts[1] == 0 && counts[2] == 0) ? 1 : 0;
    }
    if (counts[0] < 0 || counts[1] < 0 || counts[2] < 0) return 0;
    return multinomial(counts[0], counts[1], counts[2]);
}

int main(void) {
    ll N_target = 111111111111222333LL;
    ll limit = N_target;

    /* Find the length of the N-th number */
    int length = 0;
    while (1) {
        ll total_count = 0;
        for (int i = 0; i < NUM_COUNT; i++) {
            for (int j = 0; j < NUM_COUNT; j++) {
                for (int k = 0; k < NUM_COUNT; k++) {
                    if (NUMS[i] + NUMS[j] + NUMS[k] == length)
                        total_count += multinomial(NUMS[i], NUMS[j], NUMS[k]);
                }
            }
        }
        if (total_count > limit) break;
        limit -= total_count;
        length++;
    }

    /* Build the number digit by digit */
    int digits[200];
    int counts_so_far[3] = {0, 0, 0};

    for (int pos = 0; pos < length; pos++) {
        for (int d = 0; d < 3; d++) {
            ll total_count = 0;
            /* For each valid (c1,c2,c3) triple summing to length */
            for (int i = 0; i < NUM_COUNT; i++) {
                for (int j = 0; j < NUM_COUNT; j++) {
                    for (int k = 0; k < NUM_COUNT; k++) {
                        if (NUMS[i] + NUMS[j] + NUMS[k] != length) continue;
                        int cnts[3] = {NUMS[i], NUMS[j], NUMS[k]};
                        /* Subtract digits already placed */
                        int valid = 1;
                        for (int q = 0; q < 3; q++) {
                            cnts[q] -= counts_so_far[q];
                            if (cnts[q] < 0) { valid = 0; break; }
                        }
                        if (!valid) continue;
                        /* Subtract current digit d */
                        cnts[d]--;
                        if (cnts[d] < 0) continue;
                        int rem = length - pos - 1;
                        if (cnts[0] + cnts[1] + cnts[2] != rem) continue;
                        total_count += multinomial(cnts[0], cnts[1], cnts[2]);
                    }
                }
            }
            if (total_count > limit) {
                digits[pos] = d + 1;
                counts_so_far[d]++;
                break;
            }
            limit -= total_count;
        }
    }

    /* Compute the number mod M */
    ll result = 0;
    for (int i = 0; i < length; i++) {
        result = (result * 10 + digits[i]) % M;
    }

    printf("%lld\n", result);
    return 0;
}
