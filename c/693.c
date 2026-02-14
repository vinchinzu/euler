/*
 * Project Euler 693 - Finite Sequence Generator
 *
 * Compute f(N) = max g(x) over x <= N, where g(x) = max l(x,y) over y < x.
 * Uses divide-and-conquer with pruning.
 *
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define CACHE_SIZE 4000000
int cache[CACHE_SIZE];

int compute_g(int x) {
    if (x <= 2) return 0;
    if (x < CACHE_SIZE && cache[x] != -1) return cache[x];

    int *used = (int*)calloc(2 * x + 100, sizeof(int));
    int *ys = (int*)malloc(x * sizeof(int));
    int *new_ys = (int*)malloc(x * sizeof(int));

    int ys_size = 0;
    for (int i = 2; i < x; i++) {
        ys[ys_size++] = i;
    }

    int z = x;
    while (1) {
        if (ys_size == 0) {
            int result = z - x + 1;
            free(used);
            free(ys);
            free(new_ys);
            if (x < CACHE_SIZE) cache[x] = result;
            return result;
        }

        int new_ys_size = 0;
        for (int i = 0; i < ys_size; i++) {
            long long val = ((long long)ys[i] * ys[i]) % z;
            int new_y = (int)val;
            if (new_y > 1 && used[new_y] != z) {
                new_ys[new_ys_size++] = new_y;
            }
            used[new_y] = z;
        }

        int *temp = ys;
        ys = new_ys;
        ys_size = new_ys_size;
        new_ys = temp;
        z++;
    }
}

int global_best;

void helper(int low, int high, int depth) {
    if (low >= high) return;

    int g_high = compute_g(high);
    if (g_high > global_best) {
        global_best = g_high;
    }

    if (low + 1 == high || depth == 0) return;
    if (global_best >= g_high + high - low) return;

    int mid = (low + high) / 2;
    helper(low, mid, depth - 1);
    helper(mid, high, depth - 1);
}

int main(void) {
    int N = 3000000;
    global_best = 0;

    memset(cache, -1, sizeof(cache));

    for (int depth = 1; (1 << depth) < N; depth++) {
        helper(0, N, depth);
    }

    printf("%d\n", global_best);
    return 0;
}
