/* Project Euler Problem 757: Stealthy Numbers.
 * Translated from python/757.py
 *
 * A stealthy number is n = x(x+1)*g(g+1) for some x,g >= 1.
 * Count distinct stealthy numbers <= N.
 */
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;

#define MAX_SIZE 50000000

static ll *stealthies;
static int size;

int cmp_ll(const void *a, const void *b) {
    ll va = *(const ll *)a;
    ll vb = *(const ll *)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

int main() {
    ll N = 100000000000000LL; /* 10^14 */

    stealthies = (ll *)malloc(MAX_SIZE * sizeof(ll));
    size = 0;

    for (ll x = 1; x * (x + 1) * x * (x + 1) <= N; x++) {
        for (ll g = x; x * (x + 1) * g * (g + 1) <= N; g++) {
            if (size < MAX_SIZE) {
                stealthies[size++] = x * (x + 1) * g * (g + 1);
            }
        }
    }

    qsort(stealthies, size, sizeof(ll), cmp_ll);

    int count = 0;
    for (int i = 0; i < size; i++) {
        if (i == 0 || stealthies[i] != stealthies[i - 1])
            count++;
    }

    printf("%d\n", count);

    free(stealthies);
    return 0;
}
