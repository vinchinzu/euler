/*
 * Project Euler Problem 163: Cross-hatched triangles
 *
 * Count all triangles in a cross-hatched triangular grid of size N=36.
 * Represent each line by two intersection points on the outer triangle perimeter.
 * Three lines form a triangle if they pairwise intersect inside.
 * Subtract over-counted intersection points.
 */
#include <stdio.h>
#include <stdlib.h>

#define N 36
#define MAX_LINES 600

typedef struct {
    int s, e; /* start < end on perimeter 0..6N-1 */
} Line;

static int intersect(Line l1, Line l2) {
    int s1 = l1.s, e1 = l1.e, s2 = l2.s, e2 = l2.e;
    return (s1 <= s2 && s2 <= e1 && e1 <= e2) ||
           (s2 <= s1 && s1 <= e2 && e2 <= e1);
}

int main(void) {
    int P = 6 * N;
    Line lines[MAX_LINES];
    int nlines = 0;

    for (int base = 0; base < 3; base++) {
        for (int i = 0; i <= N; i++) {
            int s = (2 * N * base + 2 * i) % P;
            int e = (2 * N * base - 2 * i % P + P) % P;
            if (s > e) { int t = s; s = e; e = t; }
            lines[nlines].s = s;
            lines[nlines].e = e;
            nlines++;
        }
        for (int i = 0; i <= 2 * N; i++) {
            int s = (2 * N * base + 2 * i) % P;
            int e = ((2 * N * base - i) % P + P) % P;
            if (s > e) { int t = s; s = e; e = t; }
            lines[nlines].s = s;
            lines[nlines].e = e;
            nlines++;
        }
    }

    /* Count triplets of mutually intersecting lines */
    long long ans = 0;

    /* For each pair (i,j), count how many k > j intersect both */
    /* Precompute adjacency: for each line i, store list of lines it intersects */
    /* Actually, let's precompute a bitset for each line */
    /* nlines is about 3*(N+1) + 3*(2N+1) = 3*37 + 3*73 = 111 + 219 = 330 */
    /* So we can use a bitset of 330 bits ~ 6 u64s */

    int nw = (nlines + 63) / 64;
    typedef unsigned long long u64;
    u64 *adj = calloc(nlines * nw, sizeof(u64));

    for (int i = 0; i < nlines; i++) {
        for (int j = i + 1; j < nlines; j++) {
            if (intersect(lines[i], lines[j])) {
                adj[i * nw + j / 64] |= (1ULL << (j % 64));
                adj[j * nw + i / 64] |= (1ULL << (i % 64));
            }
        }
    }

    for (int i = 0; i < nlines; i++) {
        for (int j = i + 1; j < nlines; j++) {
            if (!(adj[i * nw + j / 64] & (1ULL << (j % 64)))) continue;
            /* Count k > j that intersect both i and j */
            for (int w = j / 64; w < nw; w++) {
                u64 common = adj[i * nw + w] & adj[j * nw + w];
                if (w == j / 64) {
                    /* Mask out bits <= j */
                    int bit = j % 64;
                    if (bit < 63)
                        common &= ~((1ULL << (bit + 1)) - 1);
                    else
                        common = 0;
                }
                ans += __builtin_popcountll(common);
            }
        }
    }

    /* Subtract over-counted intersection points */
    long long tr = (long long)(N + 1) * (N + 2) / 2;
    ans -= 20 * tr + (long long)N * N;

    printf("%lld\n", ans);
    free(adj);
    return 0;
}
