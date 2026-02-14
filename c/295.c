/*
 * Project Euler Problem 295: Lenticular Holes
 *
 * Count lenticular pairs using Stern-Brocot tree and inclusion-exclusion.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N_MAX 100000
#define N2 ((long long)N_MAX * N_MAX)
#define MAX_GROUPS 20000
#define MAX_TOTAL_RADII 7000000

typedef struct { long long rv; int gi; } RVPair;
static RVPair *all_pairs;
static int npairs;
static int group_size[MAX_GROUPS];
static int ngroups;

typedef struct { int p1, q1, p2, q2; } SB;

/* Chord data from Stern-Brocot tree */
typedef struct { long long len2; long long min_r2; } ChordInfo;

static int rv_cmp(const void *a, const void *b) {
    long long ra = ((const RVPair *)a)->rv, rb = ((const RVPair *)b)->rv;
    if (ra != rb) return (ra > rb) - (ra < rb);
    return ((const RVPair *)a)->gi - ((const RVPair *)b)->gi;
}

/* Inter hash for inclusion-exclusion */
#define IHT_SIZE (1 << 20)
#define IHT_MASK (IHT_SIZE - 1)
typedef struct { long long key; int count; int used; } IHTEntry;

static void iht_add(IHTEntry *t, long long key) {
    unsigned long long h = (unsigned long long)key;
    h = (h ^ (h >> 30)) * 0xbf58476d1ce4e5b9ULL;
    h = (h ^ (h >> 27)) * 0x94d049bb133111ebULL;
    unsigned int idx = (unsigned int)(h ^ (h >> 31)) & IHT_MASK;
    for (;;) {
        if (!t[idx].used) { t[idx].key = key; t[idx].count = 1; t[idx].used = 1; return; }
        if (t[idx].key == key) { t[idx].count++; return; }
        idx = (idx + 1) & IHT_MASK;
    }
}

static long long iht_correction(IHTEntry *t) {
    long long total = 0;
    for (int i = 0; i < IHT_SIZE; i++) {
        if (t[i].used) {
            long long n = t[i].count;
            total += n * (n + 1) / 2;
        }
    }
    return total;
}

/* Store chord info per len2 */
#define CHT_SIZE (1 << 18)
#define CHT_MASK (CHT_SIZE - 1)
typedef struct { long long len2; long long min_r2; int used; } CHTEntry;
static CHTEntry cht[CHT_SIZE];

static void cht_add(long long len2, long long min_r2) {
    unsigned long long h = (unsigned long long)len2;
    h = (h ^ (h >> 30)) * 0xbf58476d1ce4e5b9ULL;
    unsigned int idx = (unsigned int)((h >> 27) & CHT_MASK);
    for (;;) {
        if (!cht[idx].used) {
            cht[idx].len2 = len2;
            cht[idx].min_r2 = min_r2;
            cht[idx].used = 1;
            return;
        }
        if (cht[idx].len2 == len2) {
            if (min_r2 < cht[idx].min_r2) cht[idx].min_r2 = min_r2;
            return;
        }
        idx = (idx + 1) & CHT_MASK;
    }
}

int main(void) {
    memset(cht, 0, sizeof(cht));

    /* Stern-Brocot tree: enumerate chords and compute min_r2 per len2 */
    SB *stack = malloc(500000 * sizeof(SB));
    int sp = 0;
    stack[sp++] = (SB){0, 1, 1, 0};

    while (sp > 0) {
        SB e = stack[--sp];
        int p = e.p1 + e.p2, q = e.q1 + e.q2;
        if ((long long)p*p + (long long)q*q > 4LL*N_MAX) continue;

        if ((p + q) % 2 == 0) {
            long long len2 = (long long)p*p + (long long)q*q;
            /* min_r2 = (p1^2+q1^2)*(p2^2+q2^2)*len2 / 4 */
            long long f1 = (long long)e.p1*e.p1 + (long long)e.q1*e.q1;
            long long f2 = (long long)e.p2*e.p2 + (long long)e.q2*e.q2;
            long long min_r2 = f1 * f2 * len2 / 4;
            cht_add(len2, min_r2);
        }

        stack[sp++] = (SB){e.p1, e.q1, p, q};
        stack[sp++] = (SB){p, q, e.p2, e.q2};
    }
    free(stack);

    /* Collect all groups and generate (rv, gi) pairs */
    typedef struct { long long len2; long long min_r2; } GroupInfo;
    GroupInfo *ginfo = malloc(MAX_GROUPS * sizeof(GroupInfo));
    ngroups = 0;

    for (int i = 0; i < CHT_SIZE; i++) {
        if (cht[i].used) {
            ginfo[ngroups].len2 = cht[i].len2;
            ginfo[ngroups].min_r2 = cht[i].min_r2;
            ngroups++;
        }
    }

    all_pairs = malloc(MAX_TOTAL_RADII * sizeof(RVPair));
    npairs = 0;
    memset(group_size, 0, sizeof(group_size));

    for (int gi = 0; gi < ngroups; gi++) {
        long long len2 = ginfo[gi].len2;
        long long min_r2 = ginfo[gi].min_r2;
        long long half_len2 = len2 / 2;

        for (long long k = 0; ; k++) {
            long long rv = (2*k*k + 2*k + 1) * half_len2;
            if (rv > N2) break;
            if (rv >= min_r2) {
                all_pairs[npairs].rv = rv;
                all_pairs[npairs].gi = gi;
                npairs++;
                group_size[gi]++;
            }
        }
    }
    free(ginfo);

    /* Naive count */
    long long naive = 0;
    for (int gi = 0; gi < ngroups; gi++) {
        long long n = group_size[gi];
        naive += n * (n + 1) / 2;
    }

    /* Sort pairs by rv */
    qsort(all_pairs, npairs, sizeof(RVPair), rv_cmp);

    /* Inclusion-exclusion */
    IHTEntry *inter2 = calloc(IHT_SIZE, sizeof(IHTEntry));
    IHTEntry *inter3 = calloc(IHT_SIZE, sizeof(IHTEntry));
    IHTEntry *inter4 = calloc(IHT_SIZE, sizeof(IHTEntry));

    int idx = 0;
    while (idx < npairs) {
        int j = idx;
        while (j < npairs && all_pairs[j].rv == all_pairs[idx].rv) j++;

        if (j - idx >= 2) {
            int gs[20];
            int ugs = 0;
            for (int k = idx; k < j; k++) {
                int gi = all_pairs[k].gi;
                int found = 0;
                for (int g = 0; g < ugs; g++) {
                    if (gs[g] == gi) { found = 1; break; }
                }
                if (!found && ugs < 20) gs[ugs++] = gi;
            }

            if (ugs >= 2) {
                for (int a = 0; a < ugs; a++)
                    for (int b = a + 1; b < ugs; b++)
                        if (gs[a] > gs[b]) { int t = gs[a]; gs[a] = gs[b]; gs[b] = t; }

                for (int a = 0; a < ugs; a++) {
                    for (int b = a + 1; b < ugs; b++) {
                        long long pk = (long long)gs[a] * 20001 + gs[b];
                        iht_add(inter2, pk);
                        for (int c = b + 1; c < ugs; c++) {
                            long long tk = pk * 20001 + gs[c];
                            iht_add(inter3, tk);
                            for (int d = c + 1; d < ugs; d++) {
                                long long qk = tk * 20001 + gs[d];
                                iht_add(inter4, qk);
                            }
                        }
                    }
                }
            }
        }
        idx = j;
    }

    long long corr2 = iht_correction(inter2);
    long long corr3 = iht_correction(inter3);
    long long corr4 = iht_correction(inter4);

    free(inter2); free(inter3); free(inter4);
    free(all_pairs);

    printf("%lld\n", naive - corr2 + corr3 - corr4);
    return 0;
}
