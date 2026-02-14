/* Project Euler 061 - Cyclical figurate numbers */
#include <stdio.h>
#include <stdbool.h>

static int polygonal(int type, int n) {
    switch (type) {
        case 3: return n * (n + 1) / 2;
        case 4: return n * n;
        case 5: return n * (3 * n - 1) / 2;
        case 6: return n * (2 * n - 1);
        case 7: return n * (5 * n - 3) / 2;
        case 8: return n * (3 * n - 2);
    }
    return 0;
}

/* For each type (3-8), store all 4-digit numbers */
#define MAX_PER_TYPE 200
static int nums[6][MAX_PER_TYPE];
static int nums_count[6];

/* by_prefix[type][prefix] -> list of numbers with that first-two-digit prefix */
#define MAX_PER_PREFIX 20
static int by_prefix[6][100][MAX_PER_PREFIX];
static int by_prefix_count[6][100];

static int chain[6];      /* the 6 numbers in the cycle */
static int chain_type[6];  /* which polygonal type each is */
static bool type_used[6];

static bool found;
static int answer;

static void search(int depth) {
    if (found) return;
    if (depth == 6) {
        /* Check that last links back to first */
        if (chain[5] % 100 == chain[0] / 100) {
            int sum = 0;
            for (int i = 0; i < 6; i++) sum += chain[i];
            answer = sum;
            found = true;
        }
        return;
    }
    int needed_prefix = chain[depth - 1] % 100;
    if (needed_prefix < 10) return; /* two-digit suffix must be >= 10 */

    for (int t = 0; t < 6; t++) {
        if (type_used[t]) continue;
        type_used[t] = true;
        for (int k = 0; k < by_prefix_count[t][needed_prefix]; k++) {
            chain[depth] = by_prefix[t][needed_prefix][k];
            chain_type[depth] = t;
            search(depth + 1);
            if (found) return;
        }
        type_used[t] = false;
    }
}

int main(void) {
    /* Generate all 4-digit polygonal numbers */
    for (int t = 0; t < 6; t++) {
        nums_count[t] = 0;
        for (int p = 0; p < 100; p++) by_prefix_count[t][p] = 0;
        for (int n = 1; ; n++) {
            int val = polygonal(t + 3, n);
            if (val >= 10000) break;
            if (val >= 1000) {
                nums[t][nums_count[t]++] = val;
                int prefix = val / 100;
                int pc = by_prefix_count[t][prefix];
                by_prefix[t][prefix][pc] = val;
                by_prefix_count[t][prefix]++;
            }
        }
    }

    found = false;
    /* Try each type and each number as the starting point */
    for (int t = 0; t < 6 && !found; t++) {
        for (int i = 0; i < 6; i++) type_used[i] = false;
        type_used[t] = true;
        for (int i = 0; i < nums_count[t] && !found; i++) {
            chain[0] = nums[t][i];
            chain_type[0] = t;
            search(1);
        }
    }

    printf("%d\n", answer);
    return 0;
}
