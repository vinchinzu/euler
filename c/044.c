#include <stdio.h>
#include <stdbool.h>
#include <limits.h>

#define LIMIT 3000

static long long pentagonals[LIMIT];
static bool pen_set_arr[14000000]; /* large enough for max pentagonal */

bool is_in_set(long long val) {
    if (val <= 0 || val >= 14000000) return false;
    return pen_set_arr[val];
}

int main(void) {
    for (int n = 1; n <= LIMIT; n++) {
        pentagonals[n - 1] = (long long)n * (3 * n - 1) / 2;
    }

    /* Build lookup set */
    for (int i = 0; i < LIMIT; i++) {
        if (pentagonals[i] < 14000000) {
            pen_set_arr[pentagonals[i]] = true;
        }
    }

    long long min_D = LLONG_MAX;

    for (int j = 1; j < LIMIT; j++) {
        for (int k = 0; k < j; k++) {
            long long pj = pentagonals[j];
            long long pk = pentagonals[k];
            long long sum_val = pj + pk;
            long long diff = pj - pk;

            if (is_in_set(sum_val) && is_in_set(diff)) {
                if (diff < min_D) {
                    min_D = diff;
                }
            }
        }
    }

    printf("%lld\n", min_D);
    return 0;
}
