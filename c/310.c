/*
 * Project Euler Problem 310: Nim Square
 *
 * Count losing positions (a,b,c) with 0<=a<=b<=c<=100000
 * where g[a]^g[b]^g[c]==0, g being Grundy numbers for Nim Square
 * (moves are perfect squares).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define MAX_HEAP 100000

static int grundy[MAX_HEAP + 1];

/* Precompute squares */
static int squares[320]; /* sqrt(100000) ~ 316 */
static int nsquares;

static void compute_grundy(void) {
    int sq_limit = (int)sqrt((double)MAX_HEAP) + 1;
    nsquares = 0;
    for (int i = 1; i <= sq_limit; i++)
        squares[nsquares++] = i * i;

    /* Temporary seen array for mex. Grundy values stay small. */
    int max_cand = nsquares + 2;
    char *seen = calloc(max_cand, 1);

    grundy[0] = 0;
    for (int n = 1; n <= MAX_HEAP; n++) {
        for (int i = 0; i < nsquares && squares[i] <= n; i++) {
            int g = grundy[n - squares[i]];
            if (g < max_cand) seen[g] = 1;
        }
        int mex = 0;
        while (mex < max_cand && seen[mex]) mex++;
        grundy[n] = mex;

        /* Reset */
        for (int i = 0; i < nsquares && squares[i] <= n; i++) {
            int g = grundy[n - squares[i]];
            if (g < max_cand) seen[g] = 0;
        }
    }
    free(seen);
}

int main(void) {
    compute_grundy();

    /* Find max Grundy value */
    int max_g = 0;
    for (int i = 0; i <= MAX_HEAP; i++)
        if (grundy[i] > max_g) max_g = grundy[i];

    /* Build position lists by Grundy value */
    int *freq = calloc(max_g + 1, sizeof(int));
    for (int i = 0; i <= MAX_HEAP; i++)
        freq[grundy[i]]++;

    /* Build sorted position arrays */
    int **positions = calloc(max_g + 1, sizeof(int *));
    int *pos_idx = calloc(max_g + 1, sizeof(int));
    for (int g = 0; g <= max_g; g++) {
        if (freq[g] > 0)
            positions[g] = malloc(freq[g] * sizeof(int));
    }
    for (int i = 0; i <= MAX_HEAP; i++) {
        int g = grundy[i];
        positions[g][pos_idx[g]++] = i;
    }

    /* Count triples (a,b,c) with a<=b<=c and g[a]^g[b]^g[c]==0 */
    long long total = 0;

    for (int ga = 0; ga <= max_g; ga++) {
        if (!freq[ga]) continue;
        int *pos_a = positions[ga];
        int na = freq[ga];

        for (int gb = 0; gb <= max_g; gb++) {
            if (!freq[gb]) continue;
            int gc = ga ^ gb;
            if (gc > max_g || !freq[gc]) continue;

            int *pos_b = positions[gb];
            int nb = freq[gb];
            int *pos_c = positions[gc];
            int nc = freq[gc];

            /* Count triples with a in pos_a, b in pos_b, c in pos_c, a<=b<=c */
            /* For each b, count a's <= b in pos_a, count c's >= b in pos_c */
            int ia = 0, ic_start = 0;
            for (int ib = 0; ib < nb; ib++) {
                int b = pos_b[ib];
                /* Count a's <= b: binary search or linear scan */
                while (ia < na && pos_a[ia] <= b) ia++;
                int num_a = ia; /* pos_a[0..ia-1] are <= b */

                /* Count c's >= b: find first index in pos_c >= b */
                while (ic_start < nc && pos_c[ic_start] < b) ic_start++;
                int num_c = nc - ic_start;

                if (num_a > 0 && num_c > 0)
                    total += (long long)num_a * num_c;
            }
        }
    }

    printf("%lld\n", total);

    for (int g = 0; g <= max_g; g++)
        free(positions[g]);
    free(positions);
    free(pos_idx);
    free(freq);

    return 0;
}
