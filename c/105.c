/* Project Euler Problem 105: Special Sum Sets - Testing */
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

static bool is_special_sum_set(int *set_list, int n) {
    int total_subsets = 1 << n;

    /* Precompute subset sums and sizes */
    int *sums = (int *)malloc(total_subsets * sizeof(int));
    int *sizes = (int *)malloc(total_subsets * sizeof(int));

    for (int mask = 0; mask < total_subsets; mask++) {
        int s = 0, sz = 0;
        for (int i = 0; i < n; i++) {
            if (mask & (1 << i)) {
                s += set_list[i];
                sz++;
            }
        }
        sums[mask] = s;
        sizes[mask] = sz;
    }

    bool result = true;
    for (int i = 1; i < total_subsets && result; i++) {
        for (int j = i + 1; j < total_subsets && result; j++) {
            if (i & j) continue; /* not disjoint */

            if (sums[i] == sums[j]) { result = false; break; }
            if (sizes[i] > sizes[j] && sums[i] <= sums[j]) { result = false; break; }
            if (sizes[j] > sizes[i] && sums[j] <= sums[i]) { result = false; break; }
        }
    }

    free(sums);
    free(sizes);
    return result;
}

int main(void) {
    FILE *f = fopen("solutions/sets.txt", "r");
    if (!f) f = fopen("data/sets.txt", "r");
    if (!f) f = fopen("../data/sets.txt", "r");
    if (!f) f = fopen("../solutions/sets.txt", "r");
    if (!f) { printf("0\n"); return 1; }

    int total_sum = 0;
    char line[1024];
    while (fgets(line, sizeof(line), f)) {
        int nums[20], n = 0;
        char *tok = strtok(line, ",\n\r ");
        while (tok && n < 20) {
            nums[n++] = atoi(tok);
            tok = strtok(NULL, ",\n\r ");
        }
        if (n == 0) continue;

        /* Sort */
        for (int i = 0; i < n - 1; i++)
            for (int j = i + 1; j < n; j++)
                if (nums[i] > nums[j]) { int t = nums[i]; nums[i] = nums[j]; nums[j] = t; }

        if (is_special_sum_set(nums, n)) {
            int s = 0;
            for (int i = 0; i < n; i++) s += nums[i];
            total_sum += s;
        }
    }
    fclose(f);

    printf("%d\n", total_sum);
    return 0;
}
