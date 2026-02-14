/*
 * Project Euler 653 - Frictionless Tube
 * Elastic collisions in a tube. Collapsing marbles to points, sorting distances.
 */
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;

#define N_MARBLES 1000001
#define J_VAL 500001
#define D_VAL 20
#define L_VAL 1000000000

static ll distances[N_MARBLES];

static int cmp_ll(const void *a, const void *b) {
    ll da = *(const ll *)a, db = *(const ll *)b;
    if (da < db) return -1;
    if (da > db) return 1;
    return 0;
}

int main(void) {
    int m = 32745673;
    ll x = 6563116;

    ll pos = 0;
    ll L_adj = (ll)L_VAL - (ll)N_MARBLES * D_VAL / 2;

    for (int i = 0; i < N_MARBLES; i++) {
        int gap_before = (int)(x % 1000) + 1;
        int is_west = (x > 10000000);

        pos += gap_before;
        if (is_west)
            distances[i] = L_adj + pos;
        else
            distances[i] = L_adj - pos;

        x = (x * x) % m;
    }

    qsort(distances, N_MARBLES, sizeof(ll), cmp_ll);

    /* The J-th ball from the left corresponds to index N - J in sorted distances */
    printf("%lld\n", distances[N_MARBLES - J_VAL]);
    return 0;
}
