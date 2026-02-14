/* Project Euler Problem 140: Modified Fibonacci golden nuggets.
 *
 * Pell equation X^2 - 5Y^2 = 44, need X = 2 (mod 5) for positive integer N.
 * N = (X - 7) / 5.
 * Base solutions and recurrence via fundamental solution (9,4):
 *   X_{n+1} = 9*X_n + 20*Y_n
 *   Y_{n+1} = 4*X_n + 9*Y_n
 *
 * Numbers can get very large, need __int128.
 */
#include <stdio.h>
#include <stdlib.h>

typedef __int128 i128;

/* Print __int128 */
static void print_i128(i128 v) {
    if (v == 0) { printf("0"); return; }
    if (v < 0) { printf("-"); v = -v; }
    char buf[50];
    int pos = 0;
    while (v > 0) {
        buf[pos++] = '0' + (int)(v % 10);
        v /= 10;
    }
    for (int i = pos - 1; i >= 0; i--) putchar(buf[i]);
}

int cmp_i128(const void *a, const void *b) {
    i128 va = *(const i128 *)a;
    i128 vb = *(const i128 *)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

int main(void) {
    /* Base solutions to X^2 - 5Y^2 = 44 */
    long long base_x[] = {7, 8, 13, 17, 32, 43, 83};
    long long base_y[] = {1, 2, 5, 7, 14, 19, 37};
    int nbases = 7;

    i128 nuggets[200];
    int nnuggets = 0;

    for (int b = 0; b < nbases; b++) {
        i128 x0 = base_x[b];
        i128 y0 = base_y[b];

        /* Compute x1, y1 using fundamental solution (9, 4) */
        i128 x1 = 9 * x0 + 20 * y0;
        i128 y1 = 4 * x0 + 9 * y0;

        /* Check x0 */
        if (x0 > 7 && (x0 - 7) % 5 == 0) {
            nuggets[nnuggets++] = (x0 - 7) / 5;
        }

        /* Generate using recurrence: X_{n+1} = 18*X_n - X_{n-1} */
        i128 xprev = 18 * x0 - x1; /* x_{-1} */
        i128 xcurr = x0;

        for (int iter = 0; iter < 15; iter++) {
            i128 xnext = 18 * xcurr - xprev;
            if (xnext > 7 && (xnext - 7) % 5 == 0) {
                nuggets[nnuggets++] = (xnext - 7) / 5;
            }
            xprev = xcurr;
            xcurr = xnext;
        }
    }

    /* Sort, deduplicate, take first 30 */
    qsort(nuggets, nnuggets, sizeof(i128), cmp_i128);

    /* Deduplicate */
    int unique = 0;
    for (int i = 0; i < nnuggets; i++) {
        if (i == 0 || nuggets[i] != nuggets[i - 1]) {
            nuggets[unique++] = nuggets[i];
        }
    }

    /* Filter positive only and sum first 30 */
    i128 total = 0;
    int count = 0;
    for (int i = 0; i < unique && count < 30; i++) {
        if (nuggets[i] > 0) {
            total += nuggets[i];
            count++;
        }
    }

    print_i128(total);
    printf("\n");
    return 0;
}
