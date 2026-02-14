/*
 * Project Euler 143 - Torricelli point of a triangle
 *
 * Find sum of distinct p+q+r <= 120000 for Torricelli triangles.
 * Uses parametric generation of (p,q) pairs where p^2+pq+q^2 = perfect square.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdbool.h>

#define L 120000
#define MAX_PAIRS 100

static int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

/* For each value v, store list of partners q > v such that (v, q) is a valid pair */
typedef struct {
    int *data;
    int count;
    int cap;
} IntVec;

static void vec_init(IntVec *v) {
    v->data = NULL;
    v->count = 0;
    v->cap = 0;
}

static void vec_push(IntVec *v, int val) {
    if (v->count >= v->cap) {
        v->cap = v->cap ? v->cap * 2 : 4;
        v->data = realloc(v->data, v->cap * sizeof(int));
    }
    v->data[v->count++] = val;
}

/* Bit set for checking if a value is present as a key in pairs_map */
static unsigned char has_partners[(L + 8) / 8];
#define SET_BIT(arr, i) ((arr)[(i)/8] |= (1 << ((i)%8)))
#define GET_BIT(arr, i) (((arr)[(i)/8] >> ((i)%8)) & 1)

static IntVec pairs_map[L + 1];

int main(void) {
    memset(has_partners, 0, sizeof(has_partners));
    for (int i = 0; i <= L; i++) vec_init(&pairs_map[i]);

    int max_m = (int)sqrt((double)L) + 10;

    for (int m = 2; m <= max_m; m++) {
        for (int n = 1; n < m; n++) {
            if (gcd(m, n) != 1) continue;
            if ((m - n) % 3 == 0) continue;

            int a = 2 * m * n + n * n;
            int b = m * m - n * n;

            for (int k = 1; ; k++) {
                int pk = k * a;
                int qk = k * b;
                if (pk + qk > L) break;

                int mn = pk < qk ? pk : qk;
                int mx = pk < qk ? qk : pk;

                vec_push(&pairs_map[mn], mx);
                SET_BIT(has_partners, mn);
                SET_BIT(has_partners, mx);
            }
        }
    }

    /* For quick lookup: sort each partner list and use binary search or linear scan */
    /* We need to check if q is in pairs_map[p] (i.e., (p, q) is a pair with q > p) */
    /* Actually we need: for given p, the set of all partners > p */
    /* And for given q, also its partner set */

    /* Build a hash set for all sums */
    #define SUM_HASH_SIZE (1 << 20)
    #define SUM_HASH_MASK (SUM_HASH_SIZE - 1)
    typedef struct snode { int val; struct snode *next; } snode;
    static snode *sum_table[SUM_HASH_SIZE];
    memset(sum_table, 0, sizeof(sum_table));

    /* Also need: for each value, a quick set of all partners (both directions) */
    /* Build sorted partner arrays for intersection */

    /* For intersection, we need: for value v, all values u such that (min(v,u), max(v,u)) is a pair */
    /* pairs_map[v] already has all u > v. We also need u < v where (u, v) is a pair. */
    /* Build full adjacency: adj[v] = set of all partners */

    /* Actually, looking at the Python: pairs_map[p] contains q > p.
     * For the triple check: for p in pairs_map, for q in pairs_map[p],
     * if q also in pairs_map, then find common = pairs_map[p] & pairs_map[q],
     * for r in common where r > q.
     * So pairs_map[p] is "all values that form a valid pair with p, that are > p".
     * The intersection pairs_map[p] & pairs_map[q] finds values that pair with both p and q. */

    /* Sort each partner list for binary search intersection */
    for (int i = 0; i <= L; i++) {
        if (pairs_map[i].count > 1) {
            /* Simple insertion sort for small lists */
            int *d = pairs_map[i].data;
            int n = pairs_map[i].count;
            for (int a = 1; a < n; a++) {
                int key = d[a];
                int b = a - 1;
                while (b >= 0 && d[b] > key) {
                    d[b+1] = d[b];
                    b--;
                }
                d[b+1] = key;
            }
        }
    }

    long long total_sum = 0;

    for (int p = 1; p <= L; p++) {
        if (pairs_map[p].count == 0) continue;
        int *qs = pairs_map[p].data;
        int nq = pairs_map[p].count;

        for (int qi = 0; qi < nq; qi++) {
            int q = qs[qi];
            if (pairs_map[q].count == 0) continue;

            /* Find intersection of pairs_map[p] and pairs_map[q], elements > q */
            int *rs_p = pairs_map[p].data;
            int np = pairs_map[p].count;
            int *rs_q = pairs_map[q].data;
            int nqq = pairs_map[q].count;

            int ip = qi + 1; /* Start after q in p's list (since list is sorted and we want r > q) */
            int iq = 0;

            while (ip < np && iq < nqq) {
                if (rs_p[ip] == rs_q[iq]) {
                    int r = rs_p[ip];
                    int s = p + q + r;
                    if (s <= L) {
                        /* Add to set of sums */
                        unsigned int h = (unsigned int)((unsigned)s * 2654435761U) & SUM_HASH_MASK;
                        bool found = false;
                        for (snode *nd = sum_table[h]; nd; nd = nd->next) {
                            if (nd->val == s) { found = true; break; }
                        }
                        if (!found) {
                            snode *nd = malloc(sizeof(snode));
                            nd->val = s;
                            nd->next = sum_table[h];
                            sum_table[h] = nd;
                            total_sum += s;
                        }
                    }
                    ip++; iq++;
                } else if (rs_p[ip] < rs_q[iq]) {
                    ip++;
                } else {
                    iq++;
                }
            }
        }
    }

    printf("%lld\n", total_sum);
    return 0;
}
