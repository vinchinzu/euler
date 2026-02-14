/*
 * Project Euler 529 - 10-substring-friendly Numbers
 *
 * A number is 10-substring-friendly if every digit belongs to a substring
 * whose digit sum is 10. T(n) counts such numbers from 1 to 10^n.
 *
 * Approach:
 * 1. Build DP states (mask, s) where mask is a bitmask of suffix sums
 *    and s tracks uncovered digit sum.
 * 2. Build transition matrix and compute sequence terms via matrix-vector multiply.
 * 3. Find linear recurrence via Berlekamp-Massey.
 * 4. Evaluate at n=10^18 using polynomial exponentiation mod char. poly.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL
#define B_DIGIT 10
#define MAX_STATES 600
#define NUM_TERMS 5600

static ll mod(ll x) {
    return ((x % MOD) + MOD) % MOD;
}

static ll power(ll base, ll exp, ll m) {
    ll result = 1;
    base = ((base % m) + m) % m;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % m;
        base = (lll)base * base % m;
        exp >>= 1;
    }
    return result;
}

/* State: (mask, s) packed as mask * 11 + s */
typedef struct {
    int mask; /* bitmask of suffix sums (bits 0..9) */
    int s;    /* uncovered partial sum (0..10) */
} State;

static State states[MAX_STATES];
static int nstates;
static int state_map[1024 * 11]; /* map from packed to index, -1 if absent */

static int pack(int mask, int s) { return mask * 11 + s; }

/* Sparse transition matrix: for each source state, list of (target, count) */
typedef struct { int target; } Trans;
static Trans *trans_list;
static int *trans_offset; /* trans_offset[i] .. trans_offset[i+1] */
static int ntrans;

/* Sequence values */
static ll seq[NUM_TERMS];

/* Berlekamp-Massey */
static ll rec[NUM_TERMS]; /* recurrence coefficients */
static int rec_len;

static void berlekamp_massey(ll *s, int n) {
    ll C[NUM_TERMS], B_arr[NUM_TERMS], T[NUM_TERMS];
    int L = 0, m = 1;
    ll b = 1;

    memset(C, 0, sizeof(C));
    memset(B_arr, 0, sizeof(B_arr));
    C[0] = 1;
    B_arr[0] = 1;

    for (int i = 0; i < n; i++) {
        ll d = s[i];
        for (int j = 1; j <= L; j++) {
            d = (d + (lll)C[j] * s[i - j]) % MOD;
            if (d < 0) d += MOD;
        }

        if (d == 0) {
            m++;
        } else if (2 * L <= i) {
            memcpy(T, C, (L + 1) * sizeof(ll));
            int T_len = L;

            ll coeff = (lll)d * power(b, MOD - 2, MOD) % MOD;
            int new_len = L;
            int B_len_plus_m = 1 + m; /* B_arr has length 1 initially */
            /* Find actual B length */
            int blen = 0;
            for (int j = NUM_TERMS - 1; j >= 0; j--) {
                if (B_arr[j] != 0) { blen = j + 1; break; }
            }
            if (blen + m > new_len + 1) {
                /* Extend C */
                for (int j = new_len + 1; j < blen + m; j++) C[j] = 0;
            }
            int clen = L + 1;
            if (blen + m > clen) clen = blen + m;

            for (int j = 0; j < blen; j++) {
                C[j + m] = (C[j + m] - (lll)coeff * B_arr[j] % MOD + MOD) % MOD;
            }

            L = i + 1 - L;
            memcpy(B_arr, T, (T_len + 1) * sizeof(ll));
            b = d;
            m = 1;
        } else {
            ll coeff = (lll)d * power(b, MOD - 2, MOD) % MOD;
            int blen = 0;
            for (int j = NUM_TERMS - 1; j >= 0; j--) {
                if (B_arr[j] != 0) { blen = j + 1; break; }
            }
            int clen = L + 1;
            if (blen + m > clen) {
                for (int j = clen; j < blen + m; j++) C[j] = 0;
            }

            for (int j = 0; j < blen; j++) {
                C[j + m] = (C[j + m] - (lll)coeff * B_arr[j] % MOD + MOD) % MOD;
            }
            m++;
        }
    }

    rec_len = L;
    for (int i = 0; i < L; i++) {
        rec[i] = (MOD - C[i + 1]) % MOD;
    }
}

/* Polynomial multiplication mod characteristic polynomial */
/* Polynomials of degree < rec_len */
static ll *poly_a, *poly_b, *poly_c, *poly_tmp;

static void poly_mult_mod(ll *a, ll *b, ll *result) {
    int L = rec_len;
    int full = 2 * L;
    memset(poly_tmp, 0, full * sizeof(ll));

    for (int i = 0; i < L; i++) {
        if (a[i] == 0) continue;
        for (int j = 0; j < L; j++) {
            poly_tmp[i + j] = (poly_tmp[i + j] + (lll)a[i] * b[j]) % MOD;
        }
    }

    /* Reduce mod characteristic polynomial */
    for (int i = full - 2; i >= L; i--) {
        if (poly_tmp[i] == 0) continue;
        ll c = poly_tmp[i];
        poly_tmp[i] = 0;
        for (int j = 0; j < L; j++) {
            poly_tmp[i - L + j] = (poly_tmp[i - L + j] + (lll)c * rec[L - 1 - j]) % MOD;
        }
    }

    memcpy(result, poly_tmp, L * sizeof(ll));
}

int main(void) {
    ll N = 1000000000000000000LL; /* 10^18 */

    /* Phase 1: Find all reachable DP states */
    memset(state_map, -1, sizeof(state_map));

    /* BFS to discover states */
    /* Start with state (mask=1, s=0) */
    int queue[MAX_STATES * 100];
    int qfront = 0, qback = 0;

    int init_pack = pack(1, 0);
    state_map[init_pack] = 0;
    states[0].mask = 1;
    states[0].s = 0;
    nstates = 1;
    queue[qback++] = init_pack;

    while (qfront < qback) {
        int p = queue[qfront++];
        int mask = p / 11;
        int s = p % 11;

        for (int d = 0; d < B_DIGIT; d++) {
            if (d > B_DIGIT - s) break;
            int new_suf = ((mask << d) & ((1 << B_DIGIT) - 1)) | 1;
            int new_s = (mask & (1 << (B_DIGIT - d))) ? 0 : d + s;

            int np = pack(new_suf, new_s);
            if (state_map[np] == -1) {
                state_map[np] = nstates;
                states[nstates].mask = new_suf;
                states[nstates].s = new_s;
                nstates++;
                if (nstates >= MAX_STATES) {
                    fprintf(stderr, "Too many states\n");
                    return 1;
                }
                queue[qback++] = np;
            }
        }
    }

    /* Phase 2: Build transition list */
    trans_list = (Trans *)malloc(nstates * B_DIGIT * sizeof(Trans));
    trans_offset = (int *)malloc((nstates + 1) * sizeof(int));
    ntrans = 0;

    /* Build column-major transitions: for each source i, list targets */
    /* Actually we need: for matrix-vector multiply, M * v where M[j][i] = 1 */
    /* So we store: for each source column i, the target rows j */
    for (int i = 0; i < nstates; i++) {
        trans_offset[i] = ntrans;
        int mask = states[i].mask;
        int s = states[i].s;

        for (int d = 0; d < B_DIGIT; d++) {
            if (d > B_DIGIT - s) break;
            int new_suf = ((mask << d) & ((1 << B_DIGIT) - 1)) | 1;
            int new_s = (mask & (1 << (B_DIGIT - d))) ? 0 : d + s;
            int np = pack(new_suf, new_s);
            int j = state_map[np];
            if (j >= 0) {
                trans_list[ntrans].target = j;
                ntrans++;
            }
        }
    }
    trans_offset[nstates] = ntrans;

    /* Target vector: states with s == 0 */
    int *target = (int *)calloc(nstates, sizeof(int));
    for (int i = 0; i < nstates; i++) {
        if (states[i].s == 0) target[i] = 1;
    }

    /* Phase 3: Compute sequence via sparse matrix-vector multiplication */
    ll *cur = (ll *)calloc(nstates, sizeof(ll));
    ll *nxt = (ll *)calloc(nstates, sizeof(ll));
    int init_idx = state_map[pack(1, 0)];
    cur[init_idx] = 1;

    for (int t = 0; t < NUM_TERMS; t++) {
        /* seq[t] = target . cur */
        ll val = 0;
        for (int i = 0; i < nstates; i++) {
            if (target[i] && cur[i]) {
                val = (val + cur[i]) % MOD;
            }
        }
        seq[t] = val;

        /* cur = M * cur */
        memset(nxt, 0, nstates * sizeof(ll));
        for (int i = 0; i < nstates; i++) {
            if (cur[i] == 0) continue;
            for (int tt = trans_offset[i]; tt < trans_offset[i + 1]; tt++) {
                int j = trans_list[tt].target;
                nxt[j] = (nxt[j] + cur[i]) % MOD;
            }
        }
        ll *swap = cur; cur = nxt; nxt = swap;
    }

    /* Phase 4: Berlekamp-Massey */
    berlekamp_massey(seq, NUM_TERMS);
    int L = rec_len;

    /* Phase 5: Polynomial exponentiation to evaluate at n = 10^18 */
    poly_a = (ll *)calloc(L, sizeof(ll));
    poly_b = (ll *)calloc(L, sizeof(ll));
    poly_c = (ll *)calloc(L, sizeof(ll));
    poly_tmp = (ll *)calloc(2 * L + 1, sizeof(ll));

    /* base = x */
    ll *base_poly = (ll *)calloc(L, sizeof(ll));
    ll *result_poly = (ll *)calloc(L, sizeof(ll));
    base_poly[1] = 1;
    result_poly[0] = 1;

    ll exp = N;
    while (exp > 0) {
        if (exp & 1) {
            poly_mult_mod(result_poly, base_poly, poly_c);
            memcpy(result_poly, poly_c, L * sizeof(ll));
        }
        poly_mult_mod(base_poly, base_poly, poly_c);
        memcpy(base_poly, poly_c, L * sizeof(ll));
        exp >>= 1;
    }

    /* Evaluate: answer = sum(result_poly[i] * seq[i]) mod MOD */
    ll ans = 0;
    for (int i = 0; i < L; i++) {
        ans = (ans + (lll)result_poly[i] * seq[i]) % MOD;
    }

    /* T(N) = seq[N] - 1 */
    ans = (ans - 1 + MOD) % MOD;
    printf("%lld\n", ans);

    free(trans_list);
    free(trans_offset);
    free(target);
    free(cur);
    free(nxt);
    free(poly_a);
    free(poly_b);
    free(poly_c);
    free(poly_tmp);
    free(base_poly);
    free(result_poly);

    return 0;
}
