/*
 * Project Euler 844: k-Markov Numbers
 *
 * Compressed BFS for small k, polynomial summation for large k.
 * M_k(N) for K=10^18, N=10^18.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 i128;
typedef unsigned long long ull;

#define MOD 1405695061LL
#define CUTOFF 40000

/* ---- Hash set for BFS states ---- */
/* State: (ones_count, sorted non_ones tuple) */
/* We encode small states as a hash key */

#define HT_BITS 20
#define HT_SIZE (1 << HT_BITS)
#define HT_MASK (HT_SIZE - 1)

typedef struct {
    int ones;
    ll non_ones[10];
    int n_non_ones;
} State;

typedef struct ht_node {
    ull hash;
    State st;
    struct ht_node *next;
} ht_node;

static ht_node *ht[HT_SIZE];
static ht_node pool[5000000];
static int pool_idx = 0;

static ull state_hash(State *s) {
    ull h = s->ones * 1000003ULL;
    for (int i = 0; i < s->n_non_ones; i++)
        h = h * 999999937ULL + (ull)s->non_ones[i];
    return h;
}

static int state_eq(State *a, State *b) {
    if (a->ones != b->ones || a->n_non_ones != b->n_non_ones) return 0;
    for (int i = 0; i < a->n_non_ones; i++)
        if (a->non_ones[i] != b->non_ones[i]) return 0;
    return 1;
}

static void ht_clear(void) {
    memset(ht, 0, sizeof(ht));
    pool_idx = 0;
}

static int ht_insert(State *s) {
    ull h = state_hash(s);
    int idx = h & HT_MASK;
    for (ht_node *n = ht[idx]; n; n = n->next) {
        if (n->hash == h && state_eq(&n->st, s)) return 0;
    }
    ht_node *n = &pool[pool_idx++];
    n->hash = h;
    n->st = *s;
    n->next = ht[idx];
    ht[idx] = n;
    return 1;
}

/* Found numbers hash set */
#define FN_SIZE (1 << 18)
#define FN_MASK (FN_SIZE - 1)
typedef struct fn_node { ll val; struct fn_node *next; } fn_node;
static fn_node *fn_ht[FN_SIZE];
static fn_node fn_pool[2000000];
static int fn_pool_idx;

static void fn_clear(void) { memset(fn_ht, 0, sizeof(fn_ht)); fn_pool_idx = 0; }

static int fn_add(ll val) {
    int idx = (ull)val & FN_MASK;
    for (fn_node *n = fn_ht[idx]; n; n = n->next)
        if (n->val == val) return 0;
    fn_node *n = &fn_pool[fn_pool_idx++];
    n->val = val;
    n->next = fn_ht[idx];
    fn_ht[idx] = n;
    return 1;
}

/* BFS queue */
static State queue[5000000];
static int q_head, q_tail;

static void sort_ll(ll *arr, int n) {
    for (int i = 0; i < n - 1; i++)
        for (int j = i + 1; j < n; j++)
            if (arr[i] > arr[j]) { ll t = arr[i]; arr[i] = arr[j]; arr[j] = t; }
}

static ll solve_compressed(int k, ll N) {
    ht_clear();
    fn_clear();
    q_head = q_tail = 0;

    State init; init.ones = k; init.n_non_ones = 0;
    ht_insert(&init);
    queue[q_tail++] = init;
    fn_add(1);
    ll found_sum = 1;

    while (q_head < q_tail) {
        State cur = queue[q_head++];

        ll P = 1;
        for (int i = 0; i < cur.n_non_ones; i++) P *= cur.non_ones[i];

        /* Try replacing a 1 */
        if (cur.ones > 0) {
            if ((i128)k * P <= N + 1) {
                ll val = (ll)((i128)k * P - 1);
                if (val <= N && val > 1) {
                    State ns;
                    ns.ones = cur.ones - 1;
                    ns.n_non_ones = cur.n_non_ones + 1;
                    for (int i = 0; i < cur.n_non_ones; i++) ns.non_ones[i] = cur.non_ones[i];
                    ns.non_ones[cur.n_non_ones] = val;
                    sort_ll(ns.non_ones, ns.n_non_ones);
                    if (ht_insert(&ns)) {
                        if (fn_add(val)) found_sum = (found_sum + val) % MOD;
                        queue[q_tail++] = ns;
                    }
                }
            }
        }

        /* Try replacing each non-one */
        for (int i = 0; i < cur.n_non_ones; i++) {
            ll x = cur.non_ones[i];
            ll P_others = P / x;
            if ((i128)k * P_others > N + x) continue;
            ll val = (ll)((i128)k * P_others - x);
            if (val <= N && val > x) {
                State ns;
                ns.ones = cur.ones;
                ns.n_non_ones = cur.n_non_ones;
                for (int j = 0; j < cur.n_non_ones; j++) ns.non_ones[j] = cur.non_ones[j];
                ns.non_ones[i] = val;
                sort_ll(ns.non_ones, ns.n_non_ones);
                if (ht_insert(&ns)) {
                    if (fn_add(val)) found_sum = (found_sum + val) % MOD;
                    queue[q_tail++] = ns;
                }
            }
        }
    }

    return found_sum % MOD;
}

/* Sum of i^p for i=1..n mod MOD */
static ll sum_pow(int p, ll n) {
    if (n < 1) return 0;
    ll nm = n % MOD;
    ll inv2 = (MOD + 1) / 2;
    ll inv6 = 0; /* computed below */

    /* Use pow_mod */
    i128 r = 1, b = 6;
    ll e = MOD - 2;
    while (e > 0) { if (e & 1) r = r * b % MOD; b = b * b % MOD; e >>= 1; }
    inv6 = (ll)r;

    if (p == 0) return nm;
    if (p == 1) return (i128)nm * ((nm + 1) % MOD) % MOD * inv2 % MOD;
    if (p == 2) return (i128)nm * ((nm + 1) % MOD) % MOD * ((2 * nm + 1) % MOD) % MOD * inv6 % MOD;
    if (p == 3) {
        ll v = (i128)nm * ((nm + 1) % MOD) % MOD * inv2 % MOD;
        return (i128)v * v % MOD;
    }
    if (p == 4) {
        ll t1 = (i128)nm * ((nm + 1) % MOD) % MOD;
        ll t2 = (2 * nm + 1) % MOD;
        ll t3 = (3LL * nm % MOD * nm % MOD + 3 * nm % MOD - 1 + MOD) % MOD;
        t3 = ((i128)3 * nm % MOD * nm % MOD + 3 * nm % MOD - 1 + MOD) % MOD;
        ll inv30_r = 1; b = 30; e = MOD - 2; r = 1;
        while (e > 0) { if (e & 1) r = r * b % MOD; b = b * b % MOD; e >>= 1; }
        ll inv30 = (ll)r;
        return (i128)t1 * t2 % MOD * t3 % MOD * inv30 % MOD;
    }
    return 0;
}

static ll poly_eval_sum(ll *coeffs, int deg, ll limit) {
    ll total = 0;
    for (int p = 0; p <= deg; p++) {
        ll sp = sum_pow(p, limit);
        ll c = ((coeffs[p] % MOD) + MOD) % MOD;
        total = (total + (i128)c * sp) % MOD;
    }
    return total;
}

static ll poly_sum_range(ll *coeffs, int deg, ll start_k, ll end_k) {
    if (start_k > end_k) return 0;
    ll s_end = poly_eval_sum(coeffs, deg, end_k);
    ll s_start = poly_eval_sum(coeffs, deg, start_k - 1);
    return (s_end - s_start + MOD) % MOD;
}

/* Evaluate polynomial at exact integer value (for binary search) */
static __int128 poly_eval_exact(ll *coeffs, int deg, ll k) {
    __int128 res = 0, kp = 1;
    for (int i = 0; i <= deg; i++) {
        res += (__int128)coeffs[i] * kp;
        kp *= k;
    }
    return res;
}

static ll find_limit(ll *poly, int deg, ll N, ll start_k, ll max_k) {
    if (poly_eval_exact(poly, deg, start_k) > (__int128)N) return start_k - 1;
    ll low = start_k, high = max_k, ans = start_k - 1;
    while (low <= high) {
        ll mid = low + (high - low) / 2;
        if (poly_eval_exact(poly, deg, mid) <= (__int128)N) { ans = mid; low = mid + 1; }
        else high = mid - 1;
    }
    return ans;
}

int main(void) {
    ll K_VAL = 1000000000000000000LL;
    ll N_VAL = 1000000000000000000LL;

    ll total_sum = 0;

    /* 1. Small k: BFS */
    ll limit_small = CUTOFF;
    if (K_VAL < limit_small) limit_small = K_VAL;

    for (int k = 3; k <= limit_small; k++) {
        ll mk = solve_compressed(k, N_VAL);
        total_sum = (total_sum + mk) % MOD;
    }

    /* 2. Large k: polynomial summation */
    if (K_VAL > CUTOFF) {
        /* Trunk polynomials: v1=[1], v2=[-1,1], v3=[-1,-1,1], v4 = k*v3 - v2 */
        ll polys[5][10];
        int degs[5];

        polys[0][0] = 1; degs[0] = 0;
        polys[1][0] = -1; polys[1][1] = 1; degs[1] = 1;
        /* v3 = k * v2 - v1 = [-1, -1, 1] - wait, let me compute:
         * v3 = k * [-1,1] - [1] = [0,-1,1] - [1,0] = [-1,-1,1] */
        polys[2][0] = -1; polys[2][1] = -1; polys[2][2] = 1; degs[2] = 2;
        /* v4 = k * v3 - v2 = k*[-1,-1,1] - [-1,1]
         * = [0,-1,-1,1] - [-1,1,0,0] = [1,-2,-1,1] */
        polys[3][0] = 1; polys[3][1] = -2; polys[3][2] = -1; polys[3][3] = 1; degs[3] = 3;

        ll start_k_large = CUTOFF + 1;

        for (int m = 0; m < 4; m++) {
            ll limit = find_limit(polys[m], degs[m], N_VAL, start_k_large, K_VAL);
            if (limit >= start_k_large) {
                ll term_sum = poly_sum_range(polys[m], degs[m], start_k_large, limit);
                total_sum = (total_sum + term_sum) % MOD;
            }
        }
    }

    printf("%lld\n", total_sum);
    return 0;
}
