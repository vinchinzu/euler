#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

/*
 * Project Euler 898: Claire Voyant coin problem.
 *
 * Meet-in-the-middle: split 25 pairs into two halves,
 * enumerate all 3^half (value, prob) combos for each,
 * then use sorted merge to compute P(sum > 0) + 0.5 * P(sum == 0).
 */

typedef struct {
    double val;
    double prob;
} VP;

int cmp_vp(const void *a, const void *b) {
    double da = ((const VP *)a)->val;
    double db = ((const VP *)b)->val;
    if (da < db) return -1;
    if (da > db) return 1;
    return 0;
}

/* Get distribution for pair with lie probability p:
   Values: 2w, 0, -2w where w = log((1-p)/p)
   Probs: (1-p)^2, 2p(1-p), p^2 */

int main(void) {
    double pairs[25];
    for (int k = 0; k < 25; k++) {
        pairs[k] = (25 + k) / 100.0;
    }

    int n = 25;
    int mid = n / 2;  /* 12 */
    int nA = mid;     /* 12 */
    int nB = n - mid; /* 13 */

    /* Generate all sums for group A */
    int szA = 1;
    for (int i = 0; i < nA; i++) szA *= 3;

    VP *sums_A = (VP *)malloc(szA * sizeof(VP));
    sums_A[0].val = 0.0;
    sums_A[0].prob = 1.0;
    int cur_sz = 1;

    for (int i = 0; i < nA; i++) {
        double p = pairs[i];
        double w = log((1.0 - p) / p);
        double vals[3] = {2 * w, 0.0, -2 * w};
        double probs[3] = {(1.0 - p) * (1.0 - p), 2 * p * (1.0 - p), p * p};

        int new_sz = cur_sz * 3;
        VP *tmp = (VP *)malloc(new_sz * sizeof(VP));
        int idx = 0;
        for (int j = 0; j < cur_sz; j++) {
            for (int k = 0; k < 3; k++) {
                tmp[idx].val = sums_A[j].val + vals[k];
                tmp[idx].prob = sums_A[j].prob * probs[k];
                idx++;
            }
        }
        free(sums_A);
        sums_A = tmp;
        cur_sz = new_sz;
    }

    /* Generate all sums for group B */
    int szB = 1;
    for (int i = 0; i < nB; i++) szB *= 3;

    VP *sums_B = (VP *)malloc(szB * sizeof(VP));
    sums_B[0].val = 0.0;
    sums_B[0].prob = 1.0;
    cur_sz = 1;

    for (int i = 0; i < nB; i++) {
        double p = pairs[nA + i];
        double w = log((1.0 - p) / p);
        double vals[3] = {2 * w, 0.0, -2 * w};
        double probs[3] = {(1.0 - p) * (1.0 - p), 2 * p * (1.0 - p), p * p};

        int new_sz = cur_sz * 3;
        VP *tmp = (VP *)malloc(new_sz * sizeof(VP));
        int idx = 0;
        for (int j = 0; j < cur_sz; j++) {
            for (int k = 0; k < 3; k++) {
                tmp[idx].val = sums_B[j].val + vals[k];
                tmp[idx].prob = sums_B[j].prob * probs[k];
                idx++;
            }
        }
        free(sums_B);
        sums_B = tmp;
        cur_sz = new_sz;
    }

    /* Sort B by value */
    qsort(sums_B, szB, sizeof(VP), cmp_vp);

    /* Compute suffix probability sums for B */
    double *suffix_probs = (double *)malloc((szB + 1) * sizeof(double));
    suffix_probs[szB] = 0.0;
    for (int i = szB - 1; i >= 0; i--) {
        suffix_probs[i] = suffix_probs[i + 1] + sums_B[i].prob;
    }

    double total_prob = 0.0;
    double EPS = 1e-9;

    for (int i = 0; i < szA; i++) {
        double val_A = sums_A[i].val;
        double prob_A = sums_A[i].prob;
        double target = -val_A;

        /* Binary search for idx_start: first element >= target - EPS */
        int lo = 0, hi = szB;
        while (lo < hi) {
            int m = (lo + hi) / 2;
            if (sums_B[m].val < target - EPS)
                lo = m + 1;
            else
                hi = m;
        }
        int idx_start = lo;

        /* Binary search for idx_end: first element > target + EPS */
        lo = 0; hi = szB;
        while (lo < hi) {
            int m = (lo + hi) / 2;
            if (sums_B[m].val <= target + EPS)
                lo = m + 1;
            else
                hi = m;
        }
        int idx_end = lo;

        /* Strictly greater: from idx_end onwards */
        double prob_strict = suffix_probs[idx_end];
        total_prob += prob_A * prob_strict;

        /* Equal */
        if (idx_end > idx_start) {
            double prob_equal = suffix_probs[idx_start] - suffix_probs[idx_end];
            total_prob += prob_A * prob_equal * 0.5;
        }
    }

    printf("%.10f\n", total_prob);

    free(sums_A);
    free(sums_B);
    free(suffix_probs);
    return 0;
}
