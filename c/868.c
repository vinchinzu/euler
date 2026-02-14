#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/*
 * Project Euler 868 - Bell Ringing (SJT Permutation Rank)
 *
 * Compute the SJT rank of the permutation "NOWPICKBELFRYMATHS"
 * starting from alphabetical order.
 */

typedef long long ll;
typedef __int128 lll;

static ll sjt_rank(int *perm, int n) {
    if (n <= 1) return 0;

    /* Find position of largest element (n-1) */
    int largest = n - 1;
    int pos = -1;
    for (int i = 0; i < n; i++) {
        if (perm[i] == largest) {
            pos = i;
            break;
        }
    }

    /* Create sub-permutation without largest */
    int sub_perm[20];
    int idx = 0;
    for (int i = 0; i < n; i++) {
        if (perm[i] != largest) {
            sub_perm[idx++] = perm[i];
        }
    }

    ll r_sub = sjt_rank(sub_perm, n - 1);

    int local_index;
    if (r_sub % 2 == 0) {
        /* Even rank: Right-to-Left */
        local_index = (n - 1) - pos;
    } else {
        /* Odd rank: Left-to-Right */
        local_index = pos;
    }

    return r_sub * n + local_index;
}

int main(void) {
    const char *target = "NOWPICKBELFRYMATHS";
    int n = (int)strlen(target);

    /* Sort characters to get alphabetical order */
    char sorted[20];
    memcpy(sorted, target, n + 1);
    for (int i = 0; i < n - 1; i++) {
        for (int j = i + 1; j < n; j++) {
            if (sorted[j] < sorted[i]) {
                char tmp = sorted[i];
                sorted[i] = sorted[j];
                sorted[j] = tmp;
            }
        }
    }

    /* Map characters to 0..n-1 based on sorted order */
    int char_map[256];
    for (int i = 0; i < n; i++) {
        char_map[(unsigned char)sorted[i]] = i;
    }

    /* Convert target string to permutation */
    int perm[20];
    for (int i = 0; i < n; i++) {
        perm[i] = char_map[(unsigned char)target[i]];
    }

    ll result = sjt_rank(perm, n);
    printf("%lld\n", result);
    return 0;
}
