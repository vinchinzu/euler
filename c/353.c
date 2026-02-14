/* Project Euler 353 - Minimal risk paths on a spherical grid.
 *
 * Compute M(r) for r = 2^k - 1, k=1..15, using sum-of-three-squares
 * representation, spatial region neighbor pruning, and Dijkstra.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define MAX_STATIONS 500000
#define HEAP_SIZE 2000000
#define L_BOX 300

typedef struct { int x, y, z; } Point;
typedef struct { double risk; int idx; } HeapNode;

/* Global station storage */
static Point stations[MAX_STATIONS];
static int nstations;

/* Spatial region hash map */
#define REGION_HASH_SIZE (1 << 18)
#define REGION_MASK (REGION_HASH_SIZE - 1)

typedef struct RegionNode {
    int key[3];
    int *indices;
    int count;
    int capacity;
    struct RegionNode *next;
} RegionNode;

static RegionNode *region_table[REGION_HASH_SIZE];
static RegionNode region_pool[MAX_STATIONS];
static int region_pool_idx;
static int idx_pool[MAX_STATIONS * 2];
static int idx_pool_used;

unsigned int region_hash(int rx, int ry, int rz) {
    unsigned int h = (unsigned int)(rx * 73856093 ^ ry * 19349663 ^ rz * 83492791);
    return h & REGION_MASK;
}

void region_add(int rx, int ry, int rz, int idx) {
    unsigned int h = region_hash(rx, ry, rz);
    for (RegionNode *n = region_table[h]; n; n = n->next) {
        if (n->key[0] == rx && n->key[1] == ry && n->key[2] == rz) {
            if (n->count >= n->capacity) {
                int new_cap = n->capacity * 2;
                int *new_arr = &idx_pool[idx_pool_used];
                idx_pool_used += new_cap;
                memcpy(new_arr, n->indices, n->count * sizeof(int));
                n->indices = new_arr;
                n->capacity = new_cap;
            }
            n->indices[n->count++] = idx;
            return;
        }
    }
    RegionNode *n = &region_pool[region_pool_idx++];
    n->key[0] = rx; n->key[1] = ry; n->key[2] = rz;
    n->indices = &idx_pool[idx_pool_used];
    n->capacity = 8;
    idx_pool_used += 8;
    n->count = 1;
    n->indices[0] = idx;
    n->next = region_table[h];
    region_table[h] = n;
}

/* Heap for Dijkstra */
static HeapNode heap[HEAP_SIZE];
static int heap_size;

void heap_push(double risk, int idx) {
    int i = heap_size++;
    heap[i].risk = risk;
    heap[i].idx = idx;
    while (i > 0) {
        int parent = (i - 1) / 2;
        if (heap[parent].risk <= heap[i].risk) break;
        HeapNode tmp = heap[parent];
        heap[parent] = heap[i];
        heap[i] = tmp;
        i = parent;
    }
}

HeapNode heap_pop(void) {
    HeapNode top = heap[0];
    heap_size--;
    if (heap_size > 0) {
        heap[0] = heap[heap_size];
        int i = 0;
        while (1) {
            int left = 2 * i + 1, right = 2 * i + 2, smallest = i;
            if (left < heap_size && heap[left].risk < heap[smallest].risk) smallest = left;
            if (right < heap_size && heap[right].risk < heap[smallest].risk) smallest = right;
            if (smallest == i) break;
            HeapNode tmp = heap[smallest];
            heap[smallest] = heap[i];
            heap[i] = tmp;
            i = smallest;
        }
    }
    return top;
}

/* Sum of two squares representations */
typedef struct { int a, b; } Pair;
static Pair sq_reps[1000];
static int n_sq_reps;

int sqrt_neg1_mod(long long p) {
    for (int g = 2; g < p; g++) {
        long long r = 1;
        long long exp = (p - 1) / 4;
        long long base = g % p;
        while (exp > 0) {
            if (exp & 1) r = (__int128)r * base % p;
            base = (__int128)base * base % p;
            exp >>= 1;
        }
        if ((__int128)r * r % p == p - 1) return (int)r;
    }
    return -1;
}

void find_sq_rep(int p, int *a, int *b) {
    int r = sqrt_neg1_mod(p);
    int aa = p, bb = r;
    int limit = (int)sqrt((double)p);
    while (bb > limit) {
        int t = aa % bb;
        aa = bb;
        bb = t;
    }
    *a = bb;
    *b = (int)sqrt((double)(p - bb * bb));
}

/* Gaussian integer multiplication */
typedef struct { long long re, im; } Gauss;

Gauss gauss_mul(Gauss a, Gauss b) {
    Gauss c;
    c.re = a.re * b.re - a.im * b.im;
    c.im = a.re * b.im + a.im * b.re;
    return c;
}

/* Find all (y,z) with y^2+z^2 = n, y>=0, z>=0 */
void sum_of_two_squares(long long n) {
    n_sq_reps = 0;
    if (n < 0) return;
    if (n == 0) { sq_reps[n_sq_reps].a = 0; sq_reps[n_sq_reps].b = 0; n_sq_reps++; return; }

    /* Factorize n */
    typedef struct { long long p; int e; } Factor;
    Factor factors[30];
    int nfactors = 0;
    long long tmp = n;
    for (long long d = 2; d * d <= tmp; d++) {
        if (tmp % d == 0) {
            int e = 0;
            while (tmp % d == 0) { tmp /= d; e++; }
            factors[nfactors].p = d;
            factors[nfactors].e = e;
            nfactors++;
        }
    }
    if (tmp > 1) { factors[nfactors].p = tmp; factors[nfactors].e = 1; nfactors++; }

    /* Check feasibility */
    for (int i = 0; i < nfactors; i++) {
        if (factors[i].p % 4 == 3 && factors[i].e % 2 == 1) return;
    }

    /* Build representations using Gaussian integers */
    Gauss *reps = (Gauss*)malloc(1000 * sizeof(Gauss));
    int nreps = 1;
    reps[0].re = 1; reps[0].im = 0;

    for (int fi = 0; fi < nfactors; fi++) {
        long long p = factors[fi].p;
        int e = factors[fi].e;

        if (p == 2) {
            for (int ri = 0; ri < nreps; ri++) {
                for (int j = 0; j < e; j++) {
                    long long ca = reps[ri].re, cb = reps[ri].im;
                    reps[ri].re = ca - cb;
                    reps[ri].im = ca + cb;
                }
            }
        } else if (p % 4 == 1) {
            int a, b;
            find_sq_rep((int)p, &a, &b);

            /* Compute powers of (a+bi) and (a-bi) */
            Gauss *pow_plus = (Gauss*)malloc((e + 1) * sizeof(Gauss));
            Gauss *pow_minus = (Gauss*)malloc((e + 1) * sizeof(Gauss));
            pow_plus[0].re = 1; pow_plus[0].im = 0;
            pow_minus[0].re = 1; pow_minus[0].im = 0;

            Gauss gp = {a, b}, gm = {a, -b};
            for (int j = 1; j <= e; j++) {
                pow_plus[j] = gauss_mul(pow_plus[j-1], gp);
                pow_minus[j] = gauss_mul(pow_minus[j-1], gm);
            }

            int old_nreps = nreps;
            Gauss *new_reps = (Gauss*)malloc(old_nreps * (e + 1) * sizeof(Gauss));
            int new_cnt = 0;

            for (int ri = 0; ri < old_nreps; ri++) {
                for (int j = 0; j <= e; j++) {
                    Gauss g = gauss_mul(pow_plus[j], pow_minus[e - j]);
                    new_reps[new_cnt++] = gauss_mul(reps[ri], g);
                }
            }

            free(reps);
            reps = new_reps;
            nreps = new_cnt;
            free(pow_plus);
            free(pow_minus);
        } else {
            /* p % 4 == 3, e even */
            long long factor = 1;
            for (int j = 0; j < e / 2; j++) factor *= p;
            for (int ri = 0; ri < nreps; ri++) {
                reps[ri].re *= factor;
                reps[ri].im *= factor;
            }
        }
    }

    /* Collect unique (|a|, |b|) with a^2+b^2 == n, a>=0, b>=0 */
    /* Use a simple dedup */
    for (int ri = 0; ri < nreps; ri++) {
        long long a = reps[ri].re < 0 ? -reps[ri].re : reps[ri].re;
        long long b = reps[ri].im < 0 ? -reps[ri].im : reps[ri].im;
        if (a * a + b * b != n) continue;
        /* Check if already added */
        int dup = 0;
        for (int k = 0; k < n_sq_reps; k++) {
            if (sq_reps[k].a == (int)a && sq_reps[k].b == (int)b) { dup = 1; break; }
        }
        if (!dup) {
            sq_reps[n_sq_reps].a = (int)a;
            sq_reps[n_sq_reps].b = (int)b;
            n_sq_reps++;
        }
        if (a != b) {
            dup = 0;
            for (int k = 0; k < n_sq_reps; k++) {
                if (sq_reps[k].a == (int)b && sq_reps[k].b == (int)a) { dup = 1; break; }
            }
            if (!dup) {
                sq_reps[n_sq_reps].a = (int)b;
                sq_reps[n_sq_reps].b = (int)a;
                n_sq_reps++;
            }
        }
    }

    free(reps);
}

double compute_mr(int r) {
    long long r2 = (long long)r * r;

    /* Enumerate stations: for each x, find (y,z) with y^2+z^2 = r^2-x^2, y>=0, z>=0 */
    nstations = 0;
    for (int x = 0; x <= r; x++) {
        long long remainder = r2 - (long long)x * x;
        sum_of_two_squares(remainder);
        for (int k = 0; k < n_sq_reps; k++) {
            int y = sq_reps[k].a, z = sq_reps[k].b;
            if (y >= 0 && z >= 0) {
                stations[nstations].x = x;
                stations[nstations].y = y;
                stations[nstations].z = z;
                nstations++;
                if (x != 0) {
                    stations[nstations].x = -x;
                    stations[nstations].y = y;
                    stations[nstations].z = z;
                    nstations++;
                }
            }
        }
    }

    /* Sort by x descending */
    for (int i = 0; i < nstations - 1; i++) {
        for (int j = i + 1; j < nstations; j++) {
            if (stations[j].x > stations[i].x) {
                Point tmp = stations[i]; stations[i] = stations[j]; stations[j] = tmp;
            }
        }
    }

    int n = nstations;

    /* Build spatial regions */
    memset(region_table, 0, sizeof(region_table));
    region_pool_idx = 0;
    idx_pool_used = 0;

    for (int i = 0; i < n; i++) {
        int rx = (stations[i].x + r) / L_BOX + 1;
        int ry = (stations[i].y + r) / L_BOX + 1;
        int rz = (stations[i].z + r) / L_BOX + 1;
        region_add(rx, ry, rz, i);
    }

    /* Dijkstra from index 0 to index n-1 */
    double *risks = (double*)malloc(n * sizeof(double));
    char *visited = (char*)calloc(n, 1);
    for (int i = 0; i < n; i++) risks[i] = 1e30;
    risks[0] = 0.0;

    heap_size = 0;
    heap_push(0.0, 0);

    double pi_val = acos(-1.0);

    while (heap_size > 0) {
        HeapNode top = heap_pop();
        int i = top.idx;
        if (visited[i]) continue;
        visited[i] = 1;

        if (i == n - 1) {
            double result = top.risk;
            free(risks);
            free(visited);
            return result;
        }

        int sx = stations[i].x, sy = stations[i].y, sz = stations[i].z;
        int rx = (sx + r) / L_BOX + 1;
        int ry = (sy + r) / L_BOX + 1;
        int rz = (sz + r) / L_BOX + 1;

        for (int dx = -1; dx <= 1; dx++) {
            for (int dy = -1; dy <= 1; dy++) {
                for (int dz = -1; dz <= 1; dz++) {
                    unsigned int h = region_hash(rx+dx, ry+dy, rz+dz);
                    for (RegionNode *rn = region_table[h]; rn; rn = rn->next) {
                        if (rn->key[0] != rx+dx || rn->key[1] != ry+dy || rn->key[2] != rz+dz)
                            continue;
                        for (int ki = 0; ki < rn->count; ki++) {
                            int j = rn->indices[ki];
                            if (visited[j]) continue;
                            long long dot = (long long)sx * stations[j].x
                                          + (long long)sy * stations[j].y
                                          + (long long)sz * stations[j].z;
                            double cos_theta = (double)dot / (double)r2;
                            if (cos_theta > 1.0) cos_theta = 1.0;
                            if (cos_theta < -1.0) cos_theta = -1.0;
                            double theta = acos(cos_theta);
                            double edge_risk = (theta / pi_val) * (theta / pi_val);
                            double new_risk = top.risk + edge_risk;
                            if (new_risk < risks[j]) {
                                risks[j] = new_risk;
                                heap_push(new_risk, j);
                            }
                        }
                    }
                }
            }
        }
    }

    double result = risks[n - 1];
    free(risks);
    free(visited);
    return result;
}

int main(void) {
    double total = 0.0;
    for (int k = 1; k <= 15; k++) {
        int r = (1 << k) - 1;
        double mr = compute_mr(r);
        total += mr;
    }
    printf("%.10f\n", total);
    return 0;
}
