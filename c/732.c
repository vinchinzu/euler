/* Project Euler 732: Standing on the Shoulders of Trolls.
 * Knapsack DP with left/right splitting.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define NTROLLS 1000
#define MOD_VAL 1000000007LL

typedef struct { int h, l, q; } Troll;

static Troll trolls[NTROLLS];
static int D;
static int total_iq;

static int imod(long long a, long long m) {
    return (int)(((a % m) + m) % m);
}

static void generate_trolls(void) {
    long long r = 1;
    long long M = MOD_VAL;
    for (int i = 0; i < NTROLLS; i++) {
        trolls[i].h = imod(r, 101) + 50;
        r = (r * 5) % M;
        trolls[i].l = imod(r, 101) + 50;
        r = (r * 5) % M;
        trolls[i].q = imod(r, 101) + 50;
        r = (r * 5) % M;
    }
}

static const int INF = 1000000000;

static int *right_all;

static inline int* rdp_row(int k) {
    return right_all + (long long)k * (D + 1);
}

int main(void) {
    generate_trolls();

    int total_h = 0;
    total_iq = 0;
    for (int i = 0; i < NTROLLS; i++) {
        total_h += trolls[i].h;
        total_iq += trolls[i].q;
    }
    D = (int)ceil((double)total_h / sqrt(2.0));

    right_all = (int *)malloc((long long)NTROLLS * (D + 1) * sizeof(int));
    if (!right_all) { fprintf(stderr, "malloc fail\n"); return 1; }

    {
        int *row = rdp_row(0);
        row[0] = 0;
        for (int j = 1; j <= D; j++) row[j] = INF;
    }

    for (int k = 1; k < NTROLLS; k++) {
        int *prev = rdp_row(k - 1);
        int *curr = rdp_row(k);
        memcpy(curr, prev, (D + 1) * sizeof(int));

        int h = trolls[NTROLLS - k].h;
        int q = trolls[NTROLLS - k].q;

        for (int j = D; j >= h; j--) {
            int val = curr[j - h];
            if (val < INF && val + q < curr[j]) {
                curr[j] = val + q;
            }
        }

        for (int j = D - 1; j >= 0; j--) {
            if (curr[j + 1] < curr[j]) curr[j] = curr[j + 1];
        }
    }

    int *ldp = (int *)malloc((D + 1) * sizeof(int));
    ldp[0] = 0;
    for (int j = 1; j <= D; j++) ldp[j] = INF;

    int ans = 0;

    for (int i = 0; i < NTROLLS; i++) {
        int dist = D - trolls[i].h - trolls[i].l;
        if (dist >= 0) {
            int *rrow = rdp_row(NTROLLS - 1 - i);
            for (int j = 0; j <= dist; j++) {
                int lv = ldp[j];
                int rv = rrow[dist - j];
                if (lv < INF && rv < INF) {
                    int iq_used = lv + rv;
                    int remaining = total_iq - iq_used;
                    if (remaining > ans) ans = remaining;
                }
            }
        }

        int h = trolls[i].h;
        int q = trolls[i].q;
        for (int j = D; j >= h; j--) {
            int val = ldp[j - h];
            if (val < INF && val + q < ldp[j]) {
                ldp[j] = val + q;
            }
        }
        for (int j = D - 1; j >= 0; j--) {
            if (ldp[j + 1] < ldp[j]) ldp[j] = ldp[j + 1];
        }
    }

    printf("%d\n", ans);

    free(right_all);
    free(ldp);
    return 0;
}
