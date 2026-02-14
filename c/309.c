/*
 * Project Euler Problem 309: Integer Ladders
 *
 * Count integer triplets where two ladders lean against opposite walls
 * and intersect at an integer height.
 *
 * Generate Pythagorean triples, group by leg (width), check pairs of heights.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 1000000

/* For each width w, store list of heights */
static int *height_lists[N + 1]; /* pointers to height arrays */
static int height_counts[N + 1];
static int height_caps[N + 1];

static void add_height(int w, int h) {
    if (w > N) return;
    if (height_counts[w] >= height_caps[w]) {
        int newcap = height_caps[w] ? height_caps[w] * 2 : 4;
        height_lists[w] = realloc(height_lists[w], newcap * sizeof(int));
        height_caps[w] = newcap;
    }
    height_lists[w][height_counts[w]++] = h;
}

static int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

int main(void) {
    memset(height_counts, 0, sizeof(height_counts));
    memset(height_caps, 0, sizeof(height_caps));
    memset(height_lists, 0, sizeof(height_lists));

    /* Generate Pythagorean triples with hypotenuse < N */
    for (int m = 2; (long long)m * m + 1 < N; m++) {
        for (int n = 1; n < m; n++) {
            if (gcd(m, n) != 1) continue;
            if ((m % 2) == (n % 2)) continue;

            int a = m * m - n * n;
            int b = 2 * m * n;
            int c = m * m + n * n;

            for (int k = 1; (long long)k * c < N; k++) {
                int ka = k * a, kb = k * b;
                /* (ka, kb) is a pair of legs */
                add_height(ka, kb);
                add_height(kb, ka);
            }
        }
    }

    long long count = 0;

    for (int w = 1; w < N; w++) {
        int nh = height_counts[w];
        if (nh < 2) continue;

        int *hs = height_lists[w];

        for (int i = 0; i < nh; i++) {
            for (int j = i + 1; j < nh; j++) {
                long long h1 = hs[i];
                long long h2 = hs[j];
                if ((h1 * h2) % (h1 + h2) == 0)
                    count++;
            }
        }
    }

    printf("%lld\n", count);

    for (int w = 0; w <= N; w++)
        free(height_lists[w]);

    return 0;
}
