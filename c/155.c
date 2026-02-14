/*
 * Project Euler 155 - Counting Capacitor Circuits
 *
 * Count distinct capacitance values achievable with N=18 unit capacitors
 * connected in series/parallel combinations.
 * Uses hash sets of reduced fractions.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define INIT_CAP 1024

typedef struct {
    int n, d;
} Frac;

typedef struct {
    Frac *data;
    int count;
    int cap;
    int *buckets;
    int *next;
    int nbuckets;
} FracSet;

static unsigned int hash_frac(int n, int d, int nbuckets) {
    unsigned int h = (unsigned int)n * 2654435761u ^ (unsigned int)d * 2246822519u;
    return h % (unsigned int)nbuckets;
}

static void fs_init(FracSet *s, int cap) {
    s->cap = cap;
    s->count = 0;
    s->data = (Frac *)malloc(cap * sizeof(Frac));
    s->next = (int *)malloc(cap * sizeof(int));
    s->nbuckets = cap * 2;
    s->buckets = (int *)malloc(s->nbuckets * sizeof(int));
    memset(s->buckets, -1, s->nbuckets * sizeof(int));
}

static void fs_free(FracSet *s) {
    free(s->data);
    free(s->next);
    free(s->buckets);
}

static void fs_rehash(FracSet *s) {
    int new_nbuckets = s->nbuckets * 2;
    int *new_buckets = (int *)malloc(new_nbuckets * sizeof(int));
    memset(new_buckets, -1, new_nbuckets * sizeof(int));
    for (int i = 0; i < s->count; i++) {
        unsigned int h = hash_frac(s->data[i].n, s->data[i].d, new_nbuckets);
        s->next[i] = new_buckets[h];
        new_buckets[h] = i;
    }
    free(s->buckets);
    s->buckets = new_buckets;
    s->nbuckets = new_nbuckets;
}

static int fs_contains(FracSet *s, int n, int d) {
    unsigned int h = hash_frac(n, d, s->nbuckets);
    int idx = s->buckets[h];
    while (idx != -1) {
        if (s->data[idx].n == n && s->data[idx].d == d) return 1;
        idx = s->next[idx];
    }
    return 0;
}

static void fs_insert(FracSet *s, int n, int d) {
    if (fs_contains(s, n, d)) return;
    if (s->count >= s->cap) {
        s->cap *= 2;
        s->data = (Frac *)realloc(s->data, s->cap * sizeof(Frac));
        s->next = (int *)realloc(s->next, s->cap * sizeof(int));
        fs_rehash(s);
    }
    if (s->count * 3 > s->nbuckets * 2) {
        fs_rehash(s);
    }
    int idx = s->count++;
    s->data[idx].n = n;
    s->data[idx].d = d;
    unsigned int h = hash_frac(n, d, s->nbuckets);
    s->next[idx] = s->buckets[h];
    s->buckets[h] = idx;
}

static int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

#define MAXN 18

int main(void) {
    FracSet exact[MAXN + 1];
    for (int i = 0; i <= MAXN; i++) {
        fs_init(&exact[i], (i <= 2) ? 16 : INIT_CAP);
    }

    fs_insert(&exact[1], 1, 1);

    for (int k = 2; k <= MAXN; k++) {
        FracSet *found = &exact[k];

        for (int i = 1; i <= k / 2; i++) {
            int j = k - i;
            FracSet *L1 = &exact[i];
            FracSet *L2 = &exact[j];

            if (i == j) {
                int len = L1->count;
                for (int a = 0; a < len; a++) {
                    int n1 = L1->data[a].n, d1 = L1->data[a].d;
                    for (int b = a; b < len; b++) {
                        int n2 = L1->data[b].n, d2 = L1->data[b].d;
                        long long num = (long long)n1 * d2 + (long long)n2 * d1;
                        long long den = (long long)d1 * d2;
                        int g = gcd((int)(num < 0 ? -num : num), (int)(den < 0 ? -den : den));
                        int sn = (int)(num / g), sd = (int)(den / g);
                        fs_insert(found, sn, sd);
                        fs_insert(found, sd, sn);
                    }
                }
            } else {
                int len1 = L1->count, len2 = L2->count;
                for (int a = 0; a < len1; a++) {
                    int n1 = L1->data[a].n, d1 = L1->data[a].d;
                    for (int b = 0; b < len2; b++) {
                        int n2 = L2->data[b].n, d2 = L2->data[b].d;
                        long long num = (long long)n1 * d2 + (long long)n2 * d1;
                        long long den = (long long)d1 * d2;
                        int g = gcd((int)(num < 0 ? -num : num), (int)(den < 0 ? -den : den));
                        int sn = (int)(num / g), sd = (int)(den / g);
                        fs_insert(found, sn, sd);
                        fs_insert(found, sd, sn);
                    }
                }
            }
        }
    }

    /* Collect all distinct fractions */
    FracSet all;
    fs_init(&all, 1 << 20);
    for (int k = 1; k <= MAXN; k++) {
        for (int i = 0; i < exact[k].count; i++) {
            fs_insert(&all, exact[k].data[i].n, exact[k].data[i].d);
        }
    }

    printf("%d\n", all.count);

    fs_free(&all);
    for (int i = 0; i <= MAXN; i++) fs_free(&exact[i]);

    return 0;
}
