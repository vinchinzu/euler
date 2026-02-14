"""Project Euler Problem 644: Squares on the Line.

Uses embedded C for performance. Ports both the nimber computation (heap-based)
and the probability computation (range pairs) phases.
"""
import subprocess, os, tempfile


def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

#define A_PARAM 200
#define B_PARAM 500

static const double SQRT2 = 1.41421356237309504880168872420969807856967;
static const double EPS = 1e-10;

/* ---- Min-heap for events ---- */
typedef struct {
    double pos;
    int is_remove;  /* 0=add, 1=remove */
    int counter;
    int value;
} Event;

static Event *heap1;
static int heap1_size, heap1_cap;

static void heap1_push(double pos, int is_remove, int counter, int value) {
    if (heap1_size >= heap1_cap) {
        heap1_cap = heap1_cap * 2 + 1;
        heap1 = (Event *)realloc(heap1, heap1_cap * sizeof(Event));
    }
    int i = heap1_size++;
    heap1[i].pos = pos;
    heap1[i].is_remove = is_remove;
    heap1[i].counter = counter;
    heap1[i].value = value;
    /* Sift up */
    while (i > 0) {
        int parent = (i - 1) / 2;
        int swap = 0;
        if (heap1[parent].pos > heap1[i].pos) swap = 1;
        else if (heap1[parent].pos == heap1[i].pos) {
            if (heap1[parent].is_remove > heap1[i].is_remove) swap = 1;
            else if (heap1[parent].is_remove == heap1[i].is_remove) {
                if (heap1[parent].counter > heap1[i].counter) swap = 1;
            }
        }
        if (swap) {
            Event tmp = heap1[parent];
            heap1[parent] = heap1[i];
            heap1[i] = tmp;
            i = parent;
        } else break;
    }
}

static Event heap1_pop(void) {
    Event result = heap1[0];
    heap1[0] = heap1[--heap1_size];
    /* Sift down */
    int i = 0;
    while (1) {
        int smallest = i;
        int left = 2*i + 1, right = 2*i + 2;
        for (int c = left; c <= right && c < heap1_size; c++) {
            int better = 0;
            if (heap1[c].pos < heap1[smallest].pos) better = 1;
            else if (heap1[c].pos == heap1[smallest].pos) {
                if (heap1[c].is_remove < heap1[smallest].is_remove) better = 1;
                else if (heap1[c].is_remove == heap1[smallest].is_remove) {
                    if (heap1[c].counter < heap1[smallest].counter) better = 1;
                }
            }
            if (better) smallest = c;
        }
        if (smallest != i) {
            Event tmp = heap1[smallest];
            heap1[smallest] = heap1[i];
            heap1[i] = tmp;
            i = smallest;
        } else break;
    }
    return result;
}

/* ---- Multiset (nimber counts) ---- */
#define MS_MAX 4096
static int ms_counts[MS_MAX];

static void ms_add(int val) { if (val < MS_MAX) ms_counts[val]++; }
static void ms_remove(int val) { if (val < MS_MAX && ms_counts[val] > 0) ms_counts[val]--; }
static int ms_contains(int val) { return val < MS_MAX && ms_counts[val] > 0; }

/* ---- Dynamic arrays for nimber positions/values ---- */
static double *nim_pos;
static int *nim_val;
static int nim_len, nim_cap;

static void nim_push(double p, int v) {
    if (nim_len >= nim_cap) {
        nim_cap = nim_cap * 2 + 1;
        nim_pos = (double *)realloc(nim_pos, nim_cap * sizeof(double));
        nim_val = (int *)realloc(nim_val, nim_cap * sizeof(int));
    }
    nim_pos[nim_len] = p;
    nim_val[nim_len] = v;
    nim_len++;
}

/* ---- Sizes ---- */
static double *sizes;
static int sizes_len, sizes_cap;

static void sizes_push(double s) {
    if (sizes_len >= sizes_cap) {
        sizes_cap = sizes_cap * 2 + 1;
        sizes = (double *)realloc(sizes, sizes_cap * sizeof(double));
    }
    sizes[sizes_len++] = s;
}

static int cmp_double(const void *a, const void *b) {
    double da = *(const double *)a, db = *(const double *)b;
    if (da < db) return -1;
    if (da > db) return 1;
    return 0;
}

/* ---- Range pair events for phase 4 ---- */
typedef struct {
    double pos;
    int is_remove;  /* 0=add, 1=remove */
    int pid;
} Event2;

static Event2 *heap2;
static int heap2_size, heap2_cap;

static void heap2_push(double pos, int is_remove, int pid) {
    if (heap2_size >= heap2_cap) {
        heap2_cap = heap2_cap * 2 + 1;
        heap2 = (Event2 *)realloc(heap2, heap2_cap * sizeof(Event2));
    }
    int i = heap2_size++;
    heap2[i].pos = pos;
    heap2[i].is_remove = is_remove;
    heap2[i].pid = pid;
    while (i > 0) {
        int parent = (i - 1) / 2;
        if (heap2[parent].pos > heap2[i].pos ||
            (heap2[parent].pos == heap2[i].pos && heap2[parent].is_remove > heap2[i].is_remove)) {
            Event2 tmp = heap2[parent];
            heap2[parent] = heap2[i];
            heap2[i] = tmp;
            i = parent;
        } else break;
    }
}

static Event2 heap2_pop(void) {
    Event2 result = heap2[0];
    heap2[0] = heap2[--heap2_size];
    int i = 0;
    while (1) {
        int smallest = i;
        int left = 2*i + 1, right = 2*i + 2;
        for (int c = left; c <= right && c < heap2_size; c++) {
            if (heap2[c].pos < heap2[smallest].pos ||
                (heap2[c].pos == heap2[smallest].pos && heap2[c].is_remove < heap2[smallest].is_remove))
                smallest = c;
        }
        if (smallest != i) {
            Event2 tmp = heap2[smallest];
            heap2[smallest] = heap2[i];
            heap2[i] = tmp;
            i = smallest;
        } else break;
    }
    return result;
}

/* ---- Range pairs storage ---- */
typedef struct {
    double r1_lo, r1_hi, r2_lo, r2_hi;
} RangePair;

static RangePair *pairs;
static int pairs_len, pairs_cap;

static int pairs_add(double r1_lo, double r1_hi, double r2_lo, double r2_hi) {
    if (pairs_len >= pairs_cap) {
        pairs_cap = pairs_cap * 2 + 1;
        pairs = (RangePair *)realloc(pairs, pairs_cap * sizeof(RangePair));
    }
    int id = pairs_len++;
    pairs[id].r1_lo = r1_lo;
    pairs[id].r1_hi = r1_hi;
    pairs[id].r2_lo = r2_lo;
    pairs[id].r2_hi = r2_hi;
    return id;
}

/* ---- Active set (bitset) ---- */
static unsigned char *active_bits;
static int active_bits_len;

/* Active pairs linked list for iteration */
static int *active_list;
static int active_list_len, active_list_cap;

static void active_add(int pid) {
    active_bits[pid] = 1;
    if (active_list_len >= active_list_cap) {
        active_list_cap = active_list_cap * 2 + 1;
        active_list = (int *)realloc(active_list, active_list_cap * sizeof(int));
    }
    active_list[active_list_len++] = pid;
}

static void active_remove(int pid) {
    active_bits[pid] = 0;
}

/* ---- Probability map (for sizes -> prob) using hash table ---- */
#define HASH_SIZE (1 << 18)
#define HASH_MASK (HASH_SIZE - 1)

typedef struct HashEntry {
    long long key;
    double val;
    int used;
} HashEntry;

static HashEntry *prob_hash;

static void hash_init(void) {
    prob_hash = (HashEntry *)calloc(HASH_SIZE, sizeof(HashEntry));
}

static void hash_set(long long key, double val) {
    int h = (int)((unsigned long long)key * 2654435761ULL >> 14) & HASH_MASK;
    while (prob_hash[h].used && prob_hash[h].key != key)
        h = (h + 1) & HASH_MASK;
    prob_hash[h].key = key;
    prob_hash[h].val = val;
    prob_hash[h].used = 1;
}

static double hash_get(long long key) {
    int h = (int)((unsigned long long)key * 2654435761ULL >> 14) & HASH_MASK;
    while (prob_hash[h].used) {
        if (prob_hash[h].key == key) return prob_hash[h].val;
        h = (h + 1) & HASH_MASK;
    }
    return 0.0;
}

int main(void) {
    int counter = 0;

    /* Phase 1: Compute nimbers */
    memset(ms_counts, 0, sizeof(ms_counts));

    nim_cap = 1024;
    nim_len = 0;
    nim_pos = (double *)malloc(nim_cap * sizeof(double));
    nim_val = (int *)malloc(nim_cap * sizeof(int));
    nim_push(0.0, 0);

    heap1_size = 0;
    heap1_cap = 1024;
    heap1 = (Event *)malloc(heap1_cap * sizeof(Event));

    heap1_push(1.0, 0, counter++, 0);

    while (heap1_size > 0) {
        Event ev = heap1_pop();
        if (ev.pos > B_PARAM) break;

        if (ev.is_remove == 0) ms_add(ev.value);
        else ms_remove(ev.value);

        if (heap1_size > 0 && fabs(heap1[0].pos - ev.pos) < EPS)
            continue;

        int nimber = 1;
        while (ms_contains(nimber)) nimber++;

        if (nimber == nim_val[nim_len - 1]) continue;

        nim_push(ev.pos, nimber);

        int n_entries = nim_len;
        for (int i = 0; i < n_entries; i++) {
            double pos = nim_pos[i];
            int xor_val = nimber ^ nim_val[i];
            heap1_push(ev.pos + pos + 1, 0, counter++, xor_val);
            heap1_push(ev.pos + pos + SQRT2, 0, counter++, xor_val);
            if (pos != 0.0) {
                int prev_last_nimber = nim_val[nim_len - 2];
                int prev_pos_nimber = (i > 0) ? nim_val[i - 1] : 0;
                int new_nimber = prev_last_nimber ^ prev_pos_nimber;
                heap1_push(ev.pos + pos + 1, 1, counter++, new_nimber);
                heap1_push(ev.pos + pos + SQRT2, 1, counter++, new_nimber);
            }
        }
    }

    /* Phase 2: Compute sizes */
    sizes_cap = 1024;
    sizes_len = 0;
    sizes = (double *)malloc(sizes_cap * sizeof(double));

    for (int a = 1; a <= B_PARAM; a++) {
        for (int b = 1; ; b++) {
            double size = a + b * SQRT2;
            if (size > B_PARAM) break;
            if (size >= A_PARAM) sizes_push(size);
        }
    }
    qsort(sizes, sizes_len, sizeof(double), cmp_double);

    /* Phase 3: Build ranges grouped by nimber value */
    /* First, find max nimber value */
    int max_nimber = 0;
    for (int i = 0; i < nim_len; i++)
        if (nim_val[i] > max_nimber) max_nimber = nim_val[i];

    /* For each nimber value, collect ranges */
    /* ranges[nim_val] = list of (lo, hi) */
    /* Use simple linked structure */
    typedef struct { double lo, hi; } Range;
    /* Count ranges per nimber */
    int *range_counts = (int *)calloc(max_nimber + 1, sizeof(int));
    for (int i = 0; i < nim_len - 1; i++)
        range_counts[nim_val[i]]++;

    /* Allocate */
    Range **range_lists = (Range **)calloc(max_nimber + 1, sizeof(Range *));
    for (int v = 0; v <= max_nimber; v++) {
        if (range_counts[v] > 0)
            range_lists[v] = (Range *)malloc(range_counts[v] * sizeof(Range));
    }
    int *range_idx = (int *)calloc(max_nimber + 1, sizeof(int));
    for (int i = 0; i < nim_len - 1; i++) {
        int v = nim_val[i];
        range_lists[v][range_idx[v]].lo = nim_pos[i];
        range_lists[v][range_idx[v]].hi = nim_pos[i + 1];
        range_idx[v]++;
    }

    /* Phase 4: Build RangePair events */
    pairs_cap = 1024;
    pairs_len = 0;
    pairs = (RangePair *)malloc(pairs_cap * sizeof(RangePair));

    heap2_size = 0;
    heap2_cap = 1024;
    heap2 = (Event2 *)malloc(heap2_cap * sizeof(Event2));

    for (int v = 0; v <= max_nimber; v++) {
        int nc = range_counts[v];
        for (int j = 0; j < nc; j++) {
            for (int k = 0; k < nc; k++) {
                Range *r1 = &range_lists[v][j];
                Range *r2 = &range_lists[v][k];
                int pid = pairs_add(r1->lo, r1->hi, r2->lo, r2->hi);
                heap2_push(r1->lo + r2->lo, 0, pid);
                heap2_push(r1->hi + r2->hi, 1, pid);
            }
        }
    }

    /* Phase 5: For each size, compute probability */
    active_bits_len = pairs_len;
    active_bits = (unsigned char *)calloc(pairs_len, 1);
    active_list_cap = 1024;
    active_list_len = 0;
    active_list = (int *)malloc(active_list_cap * sizeof(int));

    hash_init();

    double *probs = (double *)calloc(sizes_len, sizeof(double));

    for (int si = 0; si < sizes_len; si++) {
        double size = sizes[si];

        while (heap2_size > 0 && heap2[0].pos < size) {
            Event2 ev = heap2_pop();
            if (ev.is_remove == 0) {
                active_add(ev.pid);
            } else {
                active_remove(ev.pid);
            }
        }

        double prob = 0.0;
        for (int ai = 0; ai < active_list_len; ai++) {
            int pid = active_list[ai];
            if (!active_bits[pid]) continue;
            RangePair *rp = &pairs[pid];
            double lo = rp->r1_lo;
            if (size - rp->r2_hi > lo) lo = size - rp->r2_hi;
            double hi = rp->r1_hi;
            if (size - rp->r2_lo < hi) hi = size - rp->r2_lo;
            double intersection = hi - lo;
            if (intersection > 0)
                prob += intersection / size;
        }
        probs[si] = prob;
        long long key = llround(size * 1e8);
        hash_set(key, prob);
    }

    /* Phase 6: Find maximum L * f(L) */
    double ans = 0.0;
    for (int si = 0; si < sizes_len; si++) {
        double size = sizes[si];
        double s1 = size - 1.0;
        double s2 = size - SQRT2;
        long long k1 = llround(s1 * 1e8);
        long long k2 = llround(s2 * 1e8);
        double p1 = hash_get(k1);
        double p2 = hash_get(k2);
        double L_val = size * (p1 + p2) / 2.0;
        if (L_val > ans) ans = L_val;
    }

    printf("%.8f\n", ans);

    /* Cleanup */
    free(heap1);
    free(nim_pos);
    free(nim_val);
    free(sizes);
    for (int v = 0; v <= max_nimber; v++)
        if (range_lists[v]) free(range_lists[v]);
    free(range_lists);
    free(range_counts);
    free(range_idx);
    free(pairs);
    free(heap2);
    free(active_bits);
    free(active_list);
    free(prob_hash);
    free(probs);

    return 0;
}
"""
    tmp = tempfile.NamedTemporaryFile(suffix='.c', delete=False, mode='w')
    tmp.write(c_code)
    tmp.close()
    exe = tmp.name.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', exe, tmp.name, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.run([exe], capture_output=True, text=True, timeout=280)
        print(result.stdout.strip())
    finally:
        os.unlink(tmp.name)
        if os.path.exists(exe):
            os.unlink(exe)


if __name__ == "__main__":
    solve()
