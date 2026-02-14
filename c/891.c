#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
 * Project Euler 891: Ambiguous moments on a 3-hand clock.
 *
 * Method: For each permutation sigma of the 3 hand indices,
 * solve linear congruences to find times t in [0, 43200) seconds
 * where another distinct time t' produces the same hand configuration
 * (up to rotation).
 *
 * Uses exact rational arithmetic via GCD-based fractions stored as
 * numerator/denominator pairs (long long). Times are collected in a
 * hash set to count distinct values.
 */

typedef long long ll;

ll gcd_ll(ll a, ll b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

/* Hash set for rational numbers stored as canonical (num, den) */
#define HASH_SIZE (1 << 22)  /* ~4M buckets */
#define HASH_MASK (HASH_SIZE - 1)

typedef struct Entry {
    ll num;
    ll den;
    struct Entry *next;
} Entry;

Entry *table[HASH_SIZE];
int total_count = 0;

/* Pool allocator for entries */
#define POOL_BLOCK 1000000
Entry *pool = NULL;
int pool_used = POOL_BLOCK;

Entry *alloc_entry(void) {
    if (pool_used >= POOL_BLOCK) {
        pool = (Entry *)malloc(POOL_BLOCK * sizeof(Entry));
        pool_used = 0;
    }
    return &pool[pool_used++];
}

unsigned int hash_frac(ll num, ll den) {
    unsigned long long h = (unsigned long long)num * 2654435761ULL ^ (unsigned long long)den * 40503ULL;
    return (unsigned int)(h & HASH_MASK);
}

void insert(ll num, ll den) {
    /* Canonicalize: den > 0, gcd(|num|, den) == 1 */
    if (den < 0) { num = -num; den = -den; }
    ll g = gcd_ll(num, den);
    num /= g;
    den /= g;

    unsigned int h = hash_frac(num, den);
    Entry *e = table[h];
    while (e) {
        if (e->num == num && e->den == den) return; /* already present */
        e = e->next;
    }
    Entry *ne = alloc_entry();
    ne->num = num;
    ne->den = den;
    ne->next = table[h];
    table[h] = ne;
    total_count++;
}

/* Integer coefficients A_k = 43200 * a_k */
int A[3] = {1, 12, 720};
#define T_CYCLE 43200

/* Permutations of {0,1,2} */
int perms[6][3] = {
    {0, 1, 2}, {0, 2, 1}, {1, 0, 2},
    {1, 2, 0}, {2, 0, 1}, {2, 1, 0}
};

void solve_for_perm(int sigma[3]) {
    int C01 = A[0] - A[1];
    int C02 = A[0] - A[2];
    int B01 = A[sigma[0]] - A[sigma[1]];
    int B02 = A[sigma[0]] - A[sigma[2]];

    int m11 = -C01, m12 = B01;
    int m21 = -C02, m22 = B02;

    ll D = (ll)m11 * m22 - (ll)m12 * m21;
    if (D == 0) return;

    ll Dabs = D < 0 ? -D : D;

    for (ll s = 0; s < Dabs; s++) {
        for (ll sp = 0; sp < Dabs; sp++) {
            /* k = (s * m11 + m12 * sp) / D */
            /* l = (m21 * s + m22 * sp) / D */
            ll k_num = s * m11 + (ll)m12 * sp;
            ll l_num = (ll)m21 * s + (ll)m22 * sp;
            if (k_num % D != 0 || l_num % D != 0) continue;

            /* u_num / D = t in seconds */
            ll u_num = (ll)43200 * (k_num / D * m22 - l_num / D * m12);
            ll up_num = (ll)43200 * (-(k_num / D) * m21 + l_num / D * m11);

            /* Check range: 0 <= u < 43200 and 0 <= up < 43200 */
            /* u = u_num / D, so 0 <= u_num/D < 43200 */
            if (D > 0) {
                if (u_num < 0 || u_num >= 43200LL * D) continue;
                if (up_num < 0 || up_num >= 43200LL * D) continue;
            } else {
                /* D < 0: u_num/D >= 0 means u_num <= 0 */
                if (u_num > 0 || u_num <= 43200LL * D) continue;
                if (up_num > 0 || up_num <= 43200LL * D) continue;
            }

            /* Exclude trivial case u == up */
            if (u_num == up_num) continue;

            /* Insert u = u_num / D into hash set */
            insert(u_num, D);
        }
    }
}

int main(void) {
    memset(table, 0, sizeof(table));

    for (int p = 0; p < 6; p++) {
        solve_for_perm(perms[p]);
    }

    printf("%d\n", total_count);
    return 0;
}
