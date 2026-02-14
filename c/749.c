/* Project Euler Problem 749: Near Power Sums.
 * Translated from python/749.py
 *
 * Find the sum of near power sums (sum of kth powers of digits is 1 away
 * from the number itself) with up to N digits.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;

#define MAXN 16
#define BASE 10
#define MAX_NUMS 100000

static ull nums[MAX_NUMS];
static int num_count = 0;

/* Precomputed kth powers of digits */
static ull pows[BASE];

static ull pow_limit; /* 10^N */

/* Check if candidate is a near power sum for the given k */
static int check_candidate(ull candidate, int k) {
    ull sum = 0;
    ull num = candidate;
    while (num > 0) {
        int d = num % BASE;
        sum += pows[d];
        num /= BASE;
    }
    return sum == candidate + 1 || sum == candidate - 1;
    (void)k;
}

static void helper(int d, int num_digits, ull sum_powers, int k) {
    /* Check sum_powers - 1 and sum_powers + 1 */
    for (int delta = -1; delta <= 1; delta += 2) {
        ull candidate = sum_powers + delta;
        if (candidate > 0 && candidate < pow_limit) {
            /* Verify: sum of kth powers of digits of candidate == sum_powers */
            ull actual_sum = 0;
            ull num = candidate;
            while (num > 0) {
                int dig = num % BASE;
                actual_sum += pows[dig];
                num /= BASE;
            }
            if (actual_sum == sum_powers) {
                /* Add to set (check for duplicates later) */
                if (num_count < MAX_NUMS) {
                    nums[num_count++] = candidate;
                }
            }
        }
    }

    if (num_digits < MAXN && num_digits <= k + 1) {
        for (int new_d = d; new_d < BASE; new_d++) {
            ull new_sum = sum_powers + pows[new_d];
            if (new_sum < pow_limit) {
                helper(new_d, num_digits + 1, new_sum, k);
            }
        }
    }
}

int cmp_ull(const void *a, const void *b) {
    ull va = *(const ull *)a;
    ull vb = *(const ull *)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

int main() {
    pow_limit = 1;
    for (int i = 0; i < MAXN; i++) pow_limit *= BASE;

    for (int k = 2; k <= MAXN + 2; k += 2) {
        /* Compute kth powers of digits */
        for (int d = 0; d < BASE; d++) {
            ull p = 1;
            for (int j = 0; j < k; j++) {
                p *= d;
                if (p >= pow_limit) { p = pow_limit; break; }
            }
            pows[d] = p;
        }
        helper(1, 0, 0, k);
    }

    /* Sort and deduplicate */
    qsort(nums, num_count, sizeof(ull), cmp_ull);

    ull total = 0;
    for (int i = 0; i < num_count; i++) {
        if (i == 0 || nums[i] != nums[i - 1]) {
            total += nums[i];
        }
    }

    printf("%llu\n", total);
    return 0;
}
