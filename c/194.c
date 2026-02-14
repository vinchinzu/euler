/* Project Euler 194: Coloured Configurations.
   C(100,25) * C*(C-1) * qa^25 * qb^75 mod 10^8
   where qa, qb = Q(1984) for the two unit graphs (chromatic polynomial / (C*(C-1))). */
#include <stdio.h>
#include <stdlib.h>

#define MOD 100000000LL
#define NV 7
#define MAX_EDGES 10

typedef struct { int u, v; } Edge;

static int adj_mask[NV]; /* bitmask of neighbours */

static int bt_coloring[NV];
static long long bt_count;
static int bt_c;

static void backtrack(int v) {
    if (v == NV) {
        bt_count++;
        return;
    }
    for (int col = 0; col < bt_c; col++) {
        int ok = 1;
        int mask = adj_mask[v];
        while (mask) {
            int u = __builtin_ctz(mask);
            mask &= mask - 1;
            if (bt_coloring[u] == col) { ok = 0; break; }
        }
        if (ok) {
            bt_coloring[v] = col;
            backtrack(v + 1);
            bt_coloring[v] = -1;
        }
    }
}

static long long chromatic_poly_at(Edge *edges, int ne, int c) {
    if (c == 0) return 0;
    if (c == 1) return (ne == 0) ? 1 : 0;

    /* Build adjacency */
    for (int i = 0; i < NV; i++) adj_mask[i] = 0;
    for (int i = 0; i < ne; i++) {
        adj_mask[edges[i].u] |= (1 << edges[i].v);
        adj_mask[edges[i].v] |= (1 << edges[i].u);
    }

    for (int i = 0; i < NV; i++) bt_coloring[i] = -1;
    bt_count = 0;
    bt_c = c;
    backtrack(0);
    return bt_count;
}

static long long gcd_ll(long long a, long long b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { long long t = b; b = a % b; a = t; }
    return a;
}

/* Lagrange interpolation to get Q(C_val) where Q is degree 5 polynomial
   evaluated at x_vals[0..5] with values q_vals[0..5].
   Returns Q(C_val) mod MOD */
static long long lagrange_mod(int *x_vals, long long *q_vals, int n, long long C_val) {
    /* Compute exact with big integers: use __int128 for intermediate steps.
       Actually, the values can get very large. Let's do exact rational arithmetic. */
    /* numerator, denominator pairs as __int128 */
    __int128 result_num = 0;
    __int128 result_den = 1;

    for (int i = 0; i < n; i++) {
        __int128 num = q_vals[i];
        __int128 den = 1;
        for (int j = 0; j < n; j++) {
            if (i == j) continue;
            num *= (C_val - x_vals[j]);
            den *= (x_vals[i] - x_vals[j]);
        }
        /* Add num/den to result_num/result_den */
        result_num = result_num * den + num * result_den;
        result_den *= den;

        /* Simplify to prevent overflow (though __int128 is 128 bits) */
        __int128 g = result_num;
        __int128 h = result_den;
        if (g < 0) g = -g;
        if (h < 0) h = -h;
        while (h) { __int128 t = h; h = g % h; g = t; }
        if (g > 1) {
            result_num /= g;
            result_den /= g;
        }
    }

    if (result_den < 0) {
        result_num = -result_num;
        result_den = -result_den;
    }

    /* result_num / result_den should be integer */
    long long val = (long long)(result_num / result_den);
    val %= MOD;
    if (val < 0) val += MOD;
    return val;
}

static long long mod_pow(long long base, long long exp, long long mod) {
    long long result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    int A_COUNT = 25, B_COUNT = 75;
    long long C = 1984;

    Edge a_edges[] = {{0,1},{0,2},{0,5},{1,2},{1,6},{2,3},{3,4},{4,5},{4,6},{5,6}};
    Edge b_edges[] = {{0,1},{0,2},{0,5},{1,2},{1,6},{2,3},{3,4},{4,5},{4,6}};
    int na = 10, nb = 9;

    /* Evaluate Q at c = 2, 3, ..., 7 (6 points for degree-5 polynomial) */
    int x_vals[6];
    long long qa_vals[6], qb_vals[6];
    for (int i = 0; i < 6; i++) {
        int c = i + 2;
        x_vals[i] = c;
        long long pa = chromatic_poly_at(a_edges, na, c);
        long long pb = chromatic_poly_at(b_edges, nb, c);
        qa_vals[i] = pa / (c * (c - 1));
        qb_vals[i] = pb / (c * (c - 1));
    }

    long long qa = lagrange_mod(x_vals, qa_vals, 6, C);
    long long qb = lagrange_mod(x_vals, qb_vals, 6, C);

    /* C(100, 25) mod MOD using Lucas / direct computation */
    /* Compute C(100,25) mod 10^8. Since 10^8 = 2^8 * 5^8,
       we need to handle this carefully. Use the standard method. */
    /* Direct computation with big arithmetic:
       C(100,25) = 100! / (25! * 75!)
       Compute modulo MOD by tracking prime factorization or just use __int128 */

    /* Actually, let's compute C(100,25) mod MOD directly by
       computing numerator/denominator with prime factorizations */
    /* Factor out primes from C(100,25) */
    int fact[101]; /* exponent count for primes */
    for (int i = 0; i <= 100; i++) fact[i] = 0;

    /* Add factors of 100! */
    for (int i = 2; i <= 100; i++) {
        int x = i;
        for (int p = 2; p <= x; p++) {
            while (x % p == 0) { fact[p]++; x /= p; }
        }
    }
    /* Subtract factors of 25! and 75! */
    for (int i = 2; i <= 25; i++) {
        int x = i;
        for (int p = 2; p <= x; p++) {
            while (x % p == 0) { fact[p]--; x /= p; }
        }
    }
    for (int i = 2; i <= 75; i++) {
        int x = i;
        for (int p = 2; p <= x; p++) {
            while (x % p == 0) { fact[p]--; x /= p; }
        }
    }

    long long comb = 1;
    for (int p = 2; p <= 100; p++) {
        if (fact[p] > 0) {
            comb = comb * mod_pow(p, fact[p], MOD) % MOD;
        }
    }

    long long ans = C % MOD * ((C - 1) % MOD) % MOD;
    ans = ans * comb % MOD;
    ans = ans * mod_pow(qa, A_COUNT, MOD) % MOD;
    ans = ans * mod_pow(qb, B_COUNT, MOD) % MOD;

    printf("%lld\n", ans);
    return 0;
}
