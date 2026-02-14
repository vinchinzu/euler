/* Project Euler Problem 106: Special Sum Sets - Meta-testing */
#include <stdio.h>
#include <stdbool.h>

/* Generate combinations of 'k' items from 'n' items.
   Store as arrays of indices. */

static int combos[1000][12];
static int combo_count;

static void gen_combos(int n, int k, int start, int *current, int depth) {
    if (depth == k) {
        for (int i = 0; i < k; i++)
            combos[combo_count][i] = current[i];
        combo_count++;
        return;
    }
    for (int i = start; i < n; i++) {
        current[depth] = i;
        gen_combos(n, k, i + 1, current, depth + 1);
    }
}

static bool needs_testing(int *a, int *b, int size) {
    bool a_less = true, a_greater = true;
    for (int i = 0; i < size; i++) {
        if (a[i] >= b[i]) a_less = false;
        if (a[i] <= b[i]) a_greater = false;
    }
    return !a_less && !a_greater;
}

int main(void) {
    int n = 12;
    int total = 0;

    for (int subset_size = 2; subset_size <= n / 2; subset_size++) {
        combo_count = 0;
        int current[12];
        gen_combos(n, subset_size, 0, current, 0);

        /* Compute masks for fast disjointness */
        int masks[1000];
        for (int i = 0; i < combo_count; i++) {
            masks[i] = 0;
            for (int j = 0; j < subset_size; j++)
                masks[i] |= (1 << combos[i][j]);
        }

        for (int i = 0; i < combo_count; i++) {
            for (int j = i + 1; j < combo_count; j++) {
                if (masks[i] & masks[j]) continue;
                if (needs_testing(combos[i], combos[j], subset_size))
                    total++;
            }
        }
    }

    printf("%d\n", total);
    return 0;
}
