/*
 * Project Euler 665 - Proportionate Nim
 *
 * Find losing positions in a game with two piles.
 * Uses union-find-like "next free" data structure.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

typedef struct {
    int *next_free;
    int size;
} NextFree;

static void nf_init(NextFree *nf, int n) {
    nf->size = n;
    nf->next_free = (int *)malloc(n * sizeof(int));
    for (int i = 0; i < n; i++) nf->next_free[i] = i;
}

static int nf_get(NextFree *nf, int n) {
    if (n < 0 || n >= nf->size) return n;
    if (nf->next_free[n] == n) return n;
    nf->next_free[n] = nf_get(nf, nf->next_free[n]);
    return nf->next_free[n];
}

static void nf_use(NextFree *nf, int n, int jump) {
    if (n >= 0 && n < nf->size) {
        nf->next_free[n] = nf_get(nf, nf->next_free[n + jump]);
    }
}

static void nf_free(NextFree *nf) {
    free(nf->next_free);
}

int main() {
    int N = 10000000;

    NextFree nf1, nf2, nf3, nf4;
    nf_init(&nf1, 3 * N);
    nf_init(&nf2, 2 * N);
    nf_init(&nf3, 2 * N);
    nf_init(&nf4, 4 * N);

    ll ans = 0;
    for (int n = 0; n < N; n++) {
        if (nf_get(&nf1, n) != n) continue;
        int m = n;
        while (1) {
            int old_m = m;
            m = nf_get(&nf1, m);
            m = nf_get(&nf2, m - n) + n;
            m = nf_get(&nf3, m - 2 * n + N) + 2 * n - N;
            m = (nf_get(&nf4, 2 * m - n) + n) / 2;
            if (m == old_m) break;
        }

        if (n + m <= N) {
            ans += n + m;
        }

        nf_use(&nf1, m, 1);
        nf_use(&nf2, m - n, 1);
        nf_use(&nf3, m - 2 * n + N, 1);
        nf_use(&nf3, n - 2 * m + N, 1);
        nf_use(&nf4, 2 * m - n, 2);
        nf_use(&nf4, 2 * n - m, 2);
    }

    printf("%lld\n", ans);

    nf_free(&nf1);
    nf_free(&nf2);
    nf_free(&nf3);
    nf_free(&nf4);
    return 0;
}
