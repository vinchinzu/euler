/*
 * Project Euler 839: Beans in Bowls
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>

#define NMAX 10000000

typedef struct {
    long long value;
    int length;
} Block;

static long long S[NMAX];
static Block blocks[NMAX];
static long long T[NMAX];

int main(void) {
    int N = NMAX;
    long long seed = 290797LL;
    long long m = 50515093LL;

    /* Generate BBS sequence */
    S[0] = seed;
    for (int i = 1; i < N; i++) {
        S[i] = (S[i-1] * S[i-1]) % m;
    }

    /* Merge blocks */
    int nb = 0;
    for (int i = 0; i < N; i++) {
        blocks[nb].value = S[i];
        blocks[nb].length = 1;
        nb++;
        while (nb >= 2) {
            Block *b1 = &blocks[nb - 2];
            Block *b2 = &blocks[nb - 1];
            long long avg1 = (b1->value + b1->length - 1) / b1->length;
            long long avg2 = b2->value / b2->length;
            if (avg1 <= avg2) break;
            b1->value += b2->value;
            b1->length += b2->length;
            nb--;
        }
    }

    /* Compute final state T */
    int idx = 0;
    for (int b = 0; b < nb; b++) {
        long long v = blocks[b].value;
        int len = blocks[b].length;
        for (int i = 0; i < len; i++) {
            T[idx++] = (v + i) / len;
        }
    }

    /* Count steps */
    long long ans = 0;
    for (int i = 0; i < N - 1; i++) {
        long long diff = S[i] - T[i];
        S[i + 1] += diff;
        ans += diff;
    }

    printf("%lld\n", ans);
    return 0;
}
