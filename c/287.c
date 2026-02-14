/*
 * Project Euler 287 - Quadtree Encoding
 *
 * Extracted from embedded C in the Python solution.
 * Find the minimum number of bits for quadtree encoding of a 2^N x 2^N bitmap
 * where pixel (x,y) is black iff (x-L)^2 + (y-L)^2 <= L^2, with N=24, L=2^23.
 */
#include <stdio.h>
#include <stdlib.h>

#define N 24
#define L (1 << (N - 1))

static inline int black(long long x, long long y) {
    return (x - L) * (x - L) + (y - L) * (y - L) <= (long long)L * L;
}

static int len_enc(int x, int y, int side) {
    typedef struct { int x, y, side; } Item;
    Item *stack = malloc(sizeof(Item) * 100000);
    int sp = 0;
    int bits = 0;
    stack[sp].x = x; stack[sp].y = y; stack[sp].side = side; sp++;
    while (sp > 0) {
        sp--;
        int cx = stack[sp].x, cy = stack[sp].y, cs = stack[sp].side;
        int b00 = black(cx, cy);
        int b11 = black(cx + cs - 1, cy + cs - 1);
        int b10 = black(cx + cs - 1, cy);
        int b01 = black(cx, cy + cs - 1);
        if (b00 == b11 && b10 == b01) {
            bits += 2;
        } else {
            int half = cs >> 1;
            bits += 1;
            stack[sp].x = cx;        stack[sp].y = cy;        stack[sp].side = half; sp++;
            stack[sp].x = cx + half; stack[sp].y = cy;        stack[sp].side = half; sp++;
            stack[sp].x = cx;        stack[sp].y = cy + half; stack[sp].side = half; sp++;
            stack[sp].x = cx + half; stack[sp].y = cy + half; stack[sp].side = half; sp++;
        }
    }
    free(stack);
    return bits;
}

int main() {
    long long ans = 1 + len_enc(0, 0, L) + 2 * len_enc(L, 0, L) + len_enc(L, L, L);
    printf("%lld\n", ans);
    return 0;
}
