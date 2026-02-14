/*
 * Project Euler Problem 303: Multiples with small digits
 *
 * For each n from 1 to 10000, find the smallest positive multiple of n
 * that uses only digits 0, 1, 2. Sum f(n)/n for all n.
 *
 * BFS over remainders mod n.
 */
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/* BFS queue entry: remainder and the actual value (as __int128 for large multiples) */
typedef unsigned __int128 u128;

static u128 find_min_multiple(int n) {
    if (n == 1) return 1;

    /* BFS over remainders mod n */
    /* visited[r] = 1 if remainder r already seen */
    char *visited = calloc(n, 1);
    if (!visited) exit(1);

    /* Queue stores (remainder, value) pairs */
    /* Max queue size is n */
    typedef struct { int rem; u128 val; } entry;
    entry *queue = malloc(n * sizeof(entry));
    if (!queue) exit(1);
    int head = 0, tail = 0;

    /* Start with digits 1 and 2 */
    for (int d = 1; d <= 2; d++) {
        int r = d % n;
        if (r == 0) {
            free(visited);
            free(queue);
            return d;
        }
        if (!visited[r]) {
            visited[r] = 1;
            queue[tail++] = (entry){r, d};
        }
    }

    while (head < tail) {
        entry e = queue[head++];
        for (int d = 0; d <= 2; d++) {
            u128 nv = e.val * 10 + d;
            int nr = (e.rem * 10 + d) % n;
            if (nr == 0) {
                free(visited);
                free(queue);
                return nv;
            }
            if (!visited[nr]) {
                visited[nr] = 1;
                queue[tail++] = (entry){nr, nv};
            }
        }
    }

    free(visited);
    free(queue);
    return 0; /* Should not reach here */
}

int main(void) {
    long long total = 0;
    for (int n = 1; n <= 10000; n++) {
        u128 f = find_min_multiple(n);
        /* f/n is the quotient we need */
        total += (long long)(f / n);
    }
    printf("%lld\n", total);
    return 0;
}
