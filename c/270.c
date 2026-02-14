/*
 * Project Euler 270: Cutting Squares
 *
 * Count ways to cut a square (side N=30) into triangles with vertices
 * at unit locations along sides. DP on triangular, trapezoidal, and
 * pentagonal sub-shapes.
 *
 * Shape types:
 *   type 2: triangle(a,b) - 2 sides
 *   type 3: trapezoid(a,b,c) - 3 sides (b is always N)
 *   type 5: pentagon(a,b,c,d) - 4 sides (b=c=N)
 *   type 4: square(N,N,N,N) - special initial
 */
#include <stdio.h>
#include <string.h>
#include <stdint.h>

#define NN 30
#define MOD 100000000LL

/* Memoization via hash table */
/* Encode: type*1000000 + a*10000 + b_or_0*100 + c_or_d */
/* Actually use composite key */

typedef struct {
    int type, a, b, c, d;
} MKey;

typedef struct MNode {
    MKey key;
    long long val;
    struct MNode *next;
} MNode;

#define MHASH_SIZE (1 << 20)
#define MHASH_MASK (MHASH_SIZE - 1)

static MNode *mtable[MHASH_SIZE];
static MNode mpool[6000000];
static int mpool_idx = 0;

static unsigned mhash(MKey *k) {
    unsigned h = (unsigned)k->type * 100003u;
    h ^= (unsigned)k->a * 99983u;
    h ^= (unsigned)k->b * 99979u;
    h ^= (unsigned)k->c * 99961u;
    h ^= (unsigned)k->d * 99953u;
    return h & MHASH_MASK;
}

static int mkey_eq(MKey *a, MKey *b) {
    return a->type == b->type && a->a == b->a && a->b == b->b
        && a->c == b->c && a->d == b->d;
}

static long long solve(int type, int a, int b, int c, int d);

static long long memo_get(int type, int a, int b, int c, int d) {
    MKey k = {type, a, b, c, d};
    unsigned h = mhash(&k);
    for (MNode *n = mtable[h]; n; n = n->next)
        if (mkey_eq(&n->key, &k))
            return n->val;
    long long v = solve(type, a, b, c, d);
    MNode *n = &mpool[mpool_idx++];
    n->key = k;
    n->val = v;
    n->next = mtable[h];
    mtable[h] = n;
    return v;
}

/*
 * Mirrors the Python exactly:
 *   helper(()) -> 0
 *   helper((0, ...)) -> helper(rest)
 *   helper((..., 0)) -> helper(init)
 *   helper((x,)) -> 0
 *   helper((1,1)) -> 1
 *   helper((a,b)) -> helper((a-1,b)) + helper((a,b-1))
 *   helper((a,b,c)) with b=N -> ...
 *   helper((N,N,N,N)) -> special
 *   helper((a,b,c,d)) with b=c=N -> ...
 *
 * We encode the tuple length as type.
 */

static long long solve(int type, int a, int b, int c, int d) {
    /* Remove leading zeros */
    if (type >= 2 && a == 0) {
        if (type == 2) return 0;
        if (type == 3) return memo_get(2, b, c, 0, 0);
        if (type == 4) return memo_get(3, b, c, d, 0);
    }
    /* Remove trailing zeros */
    if (type == 2 && b == 0) return 0;
    if (type == 3 && c == 0) return memo_get(2, a, b, 0, 0);
    if (type == 4 && d == 0) return memo_get(3, a, b, c, 0);

    if (type <= 1) return 0;

    if (type == 2) {
        /* Triangle (a, b) */
        if (a == 1 && b == 1) return 1;
        return (memo_get(2, a-1, b, 0, 0) + memo_get(2, a, b-1, 0, 0)) % MOD;
    }

    if (type == 3) {
        /* Trapezoid (a, N, c) */
        long long res = (memo_get(3, a-1, NN, c, 0) + memo_get(3, a, NN, c-1, 0)) % MOD;
        for (int i = 1; i < NN; i++) {
            res = (res + memo_get(2, a, i, 0, 0) * memo_get(2, NN-i, c, 0, 0)) % MOD;
        }
        return res;
    }

    if (type == 4) {
        /* Check if it's the initial square */
        if (a == NN && b == NN && c == NN && d == NN) {
            long long res = memo_get(4, NN-1, NN, NN, NN-1) % MOD;
            for (int i = 1; i < NN; i++) {
                res = (res + memo_get(3, NN, NN, i, 0) * memo_get(2, NN-i, NN-1, 0, 0)) % MOD;
            }
            for (int i = 1; i <= NN; i++) {
                res = (res + memo_get(2, NN, i, 0, 0) * memo_get(3, NN-i, NN, NN-1, 0)) % MOD;
            }
            return res % MOD;
        }
        /* General pentagon (a, N, N, d) */
        long long res = (memo_get(4, a-1, NN, NN, d) + memo_get(4, a, NN, NN, d-1)) % MOD;
        for (int i = 1; i < NN; i++) {
            res = (res + memo_get(3, a, NN, i, 0) * memo_get(2, NN-i, d, 0, 0)) % MOD;
        }
        for (int i = 1; i <= NN; i++) {
            res = (res + memo_get(2, a, i, 0, 0) * memo_get(3, NN-i, NN, d, 0)) % MOD;
        }
        return res % MOD;
    }

    return 0;
}

int main(void) {
    memset(mtable, 0, sizeof(mtable));
    long long result = memo_get(4, NN, NN, NN, NN);
    printf("%lld\n", result);
    return 0;
}
