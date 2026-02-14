#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/*
 * Badugi counting using DP on ranks.
 *
 * 4 suits, 13 ranks, 52 cards. A Badugi = 4 cards, distinct ranks, distinct suits.
 * A hand has a Badugi iff its suit-rank bipartite graph has a perfect matching.
 * By Hall's theorem: perfect matching exists iff for all S subset {0,1,2,3},
 * |N(S)| >= |S| where N(S) = ranks that have at least one card in some suit in S.
 *
 * State is the frequency vector (g(V))_{V} with sum = 13.
 * The number of hands = 13! / product g(V)!
 * The number of cards = sum g(V)*|V|.
 * Hall condition: for all nonempty S, sum_{V & S != 0} g(V) >= |S|.
 */

#define NSUITS 4
#define NRANKS 13
#define NPATTERNS 16  /* 2^4 subsets of suits */

static long long fact[NRANKS + 1];
static long long C52[53];
static int pattern_size[NPATTERNS];

static void init(void) {
    fact[0] = 1;
    for (int i = 1; i <= NRANKS; i++) fact[i] = fact[i-1] * i;

    C52[0] = 1;
    for (int n = 1; n <= 52; n++)
        C52[n] = C52[n-1] * (52 - n + 1) / n;

    for (int v = 0; v < NPATTERNS; v++)
        pattern_size[v] = __builtin_popcount(v);
}

static int check_hall(int g[NPATTERNS]) {
    for (int S = 1; S < 16; S++) {
        int ssize = __builtin_popcount(S);
        int NS = 0;
        for (int V = 1; V < 16; V++) {
            if ((V & S) && g[V] > 0) NS += g[V];
        }
        if (NS < ssize) return 0;
    }
    return 1;
}

static long long multinomial(int g[NPATTERNS]) {
    long long denom = 1;
    for (int v = 0; v < NPATTERNS; v++)
        denom *= fact[g[v]];
    return fact[NRANKS] / denom;
}

static long long f_count[53];
static int g_vec[NPATTERNS];

static void enumerate(int idx, int rem) {
    if (idx == NPATTERNS - 1) {
        g_vec[idx] = rem;

        int n = 0;
        for (int v = 0; v < NPATTERNS; v++)
            n += g_vec[v] * pattern_size[v];

        if (n < 4 || n > 13) {
            g_vec[idx] = 0;
            return;
        }

        if (check_hall(g_vec)) {
            long long coeff = multinomial(g_vec);
            f_count[n] += coeff;
        }

        g_vec[idx] = 0;
        return;
    }

    for (int k = 0; k <= rem; k++) {
        g_vec[idx] = k;
        enumerate(idx + 1, rem - k);
    }
    g_vec[idx] = 0;
}

int main(void) {
    init();
    memset(g_vec, 0, sizeof(g_vec));
    memset(f_count, 0, sizeof(f_count));

    enumerate(0, NRANKS);

    long long total = 0;
    for (int n = 4; n <= 13; n++) {
        total += f_count[n];
    }
    printf("%lld\n", total);
    return 0;
}
