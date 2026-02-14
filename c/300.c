/*
 * Project Euler Problem 300: Protein Folding
 *
 * Find average max H-H contacts in optimal folding of random 15-mer protein.
 * Enumerate SAWs, compute contact bitsets, maximize per protein mask.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N_LEN 15
#define HALF 7
#define GRID_SIZE (2 * N_LEN + 1)
#define OFFSET N_LEN

static int grid[GRID_SIZE][GRID_SIZE];
static const int DX[] = {1, -1, 0, 0};
static const int DY[] = {0, 0, 1, -1};

/* Contact bitsets (up to 56 bits: 8 * 7 = 56) */
#define MAX_BITSETS 500000
static long long bitsets[MAX_BITSETS];
static int nbitsets;

/* Deduplicated bitsets stored in hash set */
#define BS_HT_SIZE (1 << 20)
#define BS_HT_MASK (BS_HT_SIZE - 1)
static long long bs_ht[BS_HT_SIZE]; /* 0 = empty (valid since empty bitset never added) */

static int bs_add(long long bs) {
    if (bs == 0) return 0;
    unsigned long long h = (unsigned long long)bs;
    h = (h ^ (h >> 30)) * 0xbf58476d1ce4e5b9ULL;
    int idx = (int)((h ^ (h >> 27)) & BS_HT_MASK);
    while (1) {
        if (bs_ht[idx] == 0) {
            bs_ht[idx] = bs;
            return 1; /* new */
        }
        if (bs_ht[idx] == bs) return 0; /* already exists */
        idx = (idx + 1) & BS_HT_MASK;
    }
}

static void dfs(int step, int x, int y, long long bitset) {
    if (step == N_LEN) {
        if (bs_add(bitset)) {
            bitsets[nbitsets++] = bitset;
        }
        return;
    }

    int dstart = 0, dend = 4;
    if (step == 1) { dstart = 0; dend = 1; } /* first step: east only */
    else if (step == 2) { dstart = 0; dend = 3; } /* second step: east and north (not south) */
    /* For step==2: east(0), north(2) -> indices 0 and 2 */

    for (int d = 0; d < 4; d++) {
        if (step == 1 && d != 0) continue; /* only east */
        if (step == 2 && d == 1) continue; /* not west */
        if (step == 2 && d == 3) continue; /* not south */

        int nx = x + DX[d];
        int ny = y + DY[d];
        int gx = nx + OFFSET;
        int gy = ny + OFFSET;

        if (gx < 0 || gx >= GRID_SIZE || gy < 0 || gy >= GRID_SIZE) continue;
        if (grid[gy][gx] >= 0) continue;

        long long new_bits = 0;
        for (int dd = 0; dd < 4; dd++) {
            int nnx = nx + DX[dd];
            int nny = ny + DY[dd];
            int ggx = nnx + OFFSET;
            int ggy = nny + OFFSET;
            if (ggx < 0 || ggx >= GRID_SIZE || ggy < 0 || ggy >= GRID_SIZE) continue;
            int prev = grid[ggy][ggx];
            if (prev < 0) continue;
            if (step % 2 == 0 && prev % 2 == 1) {
                new_bits |= 1LL << (HALF * (step / 2) + prev / 2);
            } else if (step % 2 == 1 && prev % 2 == 0) {
                new_bits |= 1LL << (HALF * (prev / 2) + step / 2);
            }
        }

        grid[gy][gx] = step;
        dfs(step + 1, nx, ny, bitset | new_bits);
        grid[gy][gx] = -1;
    }
}

static int popcount64(long long x) {
    return __builtin_popcountll(x);
}

int main(void) {
    /* Initialize grid */
    memset(grid, -1, sizeof(grid));
    memset(bs_ht, 0, sizeof(bs_ht));
    nbitsets = 0;

    grid[OFFSET][OFFSET] = 0;
    dfs(1, 0, 0, 0);

    /* Prune dominated bitsets */
    /* Sort ascending */
    /* Simple O(n^2) prune - bitsets list should be manageable */
    int *keep = (int *)calloc(nbitsets, sizeof(int));
    for (int i = 0; i < nbitsets; i++) keep[i] = 1;

    for (int i = 0; i < nbitsets; i++) {
        if (!keep[i]) continue;
        for (int j = 0; j < nbitsets; j++) {
            if (i == j || !keep[j]) continue;
            if ((bitsets[j] & bitsets[i]) == bitsets[j] && bitsets[j] != bitsets[i]) {
                keep[j] = 0; /* j is dominated by i */
            }
        }
    }

    int npruned = 0;
    for (int i = 0; i < nbitsets; i++) {
        if (keep[i]) bitsets[npruned++] = bitsets[i];
    }
    free(keep);

    /* For each protein mask (2^15 = 32768), compute max contacts */
    long long sum_contacts = 0;
    int total_proteins = 1 << N_LEN;

    for (int protein = 0; protein < total_proteins; protein++) {
        /* Compute protein contact bitset */
        int even_bits = 0;
        for (int i = 0; i < 8; i++) {
            if (protein & (1 << (2 * i))) even_bits |= 1 << i;
        }
        int odd_bits = 0;
        for (int j = 0; j < 7; j++) {
            if (protein & (1 << (2 * j + 1))) odd_bits |= 1 << j;
        }
        long long pbs = 0;
        for (int i = 0; i < 8; i++) {
            if (even_bits & (1 << i)) {
                pbs |= (long long)odd_bits << (7 * i);
            }
        }

        int max_contacts = 0;
        for (int f = 0; f < npruned; f++) {
            int c = popcount64(pbs & bitsets[f]);
            if (c > max_contacts) max_contacts = c;
        }
        sum_contacts += max_contacts;
    }

    /* Print average: sum_contacts / 2^15 */
    double avg = (double)sum_contacts / total_proteins;
    /* The answer should have limited precision */
    /* Expected: 8.0540771484375 */
    /* Print with enough precision */
    printf("%.13f\n", avg);
    return 0;
}
