/*
 * Project Euler 774 - Conjunctive Sequences
 *
 * c(n,b) = number of conjunctive sequences of length n with all terms <= b.
 * A sequence is conjunctive if a_i & a_{i+1} != 0 for all consecutive pairs.
 *
 * Uses Tensor-Train / Matrix Product State (MPS) representation.
 * Each vector of size 2^m is represented as a tensor train with m cores.
 * Core[i] has shape (rL, 2, rR) for left rank rL, physical dim 2, right rank rR.
 *
 * Operations:
 *   - Hadamard product (element-wise multiply): ranks multiply
 *   - tt_add: ranks add (direct sum)
 *   - tt_apply_local: per-bit 2x2 matrix
 *   - tt_reduce_left: Gaussian elimination compression
 *
 * The iteration: dp_{t+1} = 1_{x<=b} * ((J - B) * dp_t)
 * where B_{y,x} = [x&y==0], computed per-bit.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MOD 998244353LL
typedef long long ll;

/* A TT core: shape (rL, 2, rR). Stored as flat array data[l*2*rR + bit*rR + r]. */
typedef struct {
    int rL, rR;
    ll *data;  /* rL * 2 * rR entries */
} Core;

typedef struct {
    int m;
    Core *cores;  /* array of m cores */
} TT;

ll mod(ll x) { return ((x % MOD) + MOD) % MOD; }

ll powmod(ll base, ll exp, ll m) {
    ll r = 1; base %= m;
    while (exp > 0) {
        if (exp & 1) r = r * base % m;
        base = base * base % m;
        exp >>= 1;
    }
    return r;
}

ll modinv(ll a) { return powmod(mod(a), MOD - 2, MOD); }

/* Access core data */
static inline ll core_get(Core *c, int l, int bit, int r) {
    return c->data[l * 2 * c->rR + bit * c->rR + r];
}
static inline void core_set(Core *c, int l, int bit, int r, ll val) {
    c->data[l * 2 * c->rR + bit * c->rR + r] = val;
}

Core core_alloc(int rL, int rR) {
    Core c;
    c.rL = rL;
    c.rR = rR;
    size_t sz = (size_t)rL * 2 * rR;
    c.data = (ll *)calloc(sz, sizeof(ll));
    return c;
}

void core_free(Core *c) {
    if (c->data) { free(c->data); c->data = NULL; }
}

TT tt_alloc(int m) {
    TT tt;
    tt.m = m;
    tt.cores = (Core *)calloc(m, sizeof(Core));
    return tt;
}

void tt_free(TT *tt) {
    for (int i = 0; i < tt->m; i++) core_free(&tt->cores[i]);
    free(tt->cores);
    tt->cores = NULL;
}

/* All-ones tensor train */
TT tt_all_ones(int m) {
    TT tt = tt_alloc(m);
    for (int i = 0; i < m; i++) {
        tt.cores[i] = core_alloc(1, 1);
        core_set(&tt.cores[i], 0, 0, 0, 1);
        core_set(&tt.cores[i], 0, 1, 0, 1);
    }
    return tt;
}

/* Indicator 1_{x <= b} for x in [0, 2^m) as TT using DFA */
TT tt_indicator_leq(ll b, int m) {
    int bits[64];
    for (int i = 0; i < m; i++)
        bits[i] = (int)((b >> (m - 1 - i)) & 1);  /* MSB first */

    TT tt = tt_alloc(m);

    if (m == 1) {
        /* Single core (1,2,1) */
        tt.cores[0] = core_alloc(1, 1);
        /* DFA: start=tight(1), accept both states */
        /* T0: tight->tight if bit=0; T1: tight->loose if bit=1, tight->dead if bit>b_bit */
        for (int xbit = 0; xbit < 2; xbit++) {
            /* Transitions from tight state (state 1) */
            int bb = bits[0];
            ll val;
            if (bb == 0) {
                /* T0: (1,0)(0,1); T1: (1,0)(0,0) */
                if (xbit == 0) val = mod(1 + 1); /* T[1][0]+T[1][1] = 0+1 = 1 */
                else val = mod(0 + 0);  /* T[1][0]+T[1][1] = 0+0 = 0 */
                /* Wait: T0 for bb=0: tight->loose if x=0 (which is <=0), tight stays tight */
                /* Actually: state 0 = loose, state 1 = tight */
                /* T_x[prev][next] */
                /* bb=0: T_0 = ((1,0),(0,1)), T_1 = ((1,0),(0,0)) */
                /* From tight (prev=1): T_0[1][0]+T_0[1][1] = 0+1 = 1 */
                /*                       T_1[1][0]+T_1[1][1] = 0+0 = 0 */
                val = (xbit == 0) ? 1 : 0;
            } else {
                /* bb=1: T_0 = ((1,0),(1,0)), T_1 = ((1,0),(0,1)) */
                /* From tight: T_0[1][0]+T_0[1][1] = 1+0 = 1 */
                /*             T_1[1][0]+T_1[1][1] = 0+1 = 1 */
                val = 1;
            }
            core_set(&tt.cores[0], 0, xbit, 0, val);
        }
        return tt;
    }

    for (int idx = 0; idx < m; idx++) {
        int bb = bits[idx];
        /* Transition matrices T_0 and T_1 (2x2, prev->next) */
        /* State 0 = loose, state 1 = tight */
        ll T[2][2][2]; /* T[xbit][prev][next] */
        if (bb == 0) {
            T[0][0][0]=1; T[0][0][1]=0; T[0][1][0]=0; T[0][1][1]=1;
            T[1][0][0]=1; T[1][0][1]=0; T[1][1][0]=0; T[1][1][1]=0;
        } else {
            T[0][0][0]=1; T[0][0][1]=0; T[0][1][0]=1; T[0][1][1]=0;
            T[1][0][0]=1; T[1][0][1]=0; T[1][1][0]=0; T[1][1][1]=1;
        }

        if (idx == 0) {
            /* (1,2,2): absorb start state = tight (prev=1) */
            tt.cores[0] = core_alloc(1, 2);
            for (int xbit = 0; xbit < 2; xbit++) {
                core_set(&tt.cores[0], 0, xbit, 0, T[xbit][1][0]);
                core_set(&tt.cores[0], 0, xbit, 1, T[xbit][1][1]);
            }
        } else if (idx == m - 1) {
            /* (2,2,1): absorb end accept = (1,1) */
            tt.cores[idx] = core_alloc(2, 1);
            for (int prev = 0; prev < 2; prev++)
                for (int xbit = 0; xbit < 2; xbit++)
                    core_set(&tt.cores[idx], prev, xbit, 0,
                             mod(T[xbit][prev][0] + T[xbit][prev][1]));
        } else {
            /* (2,2,2) */
            tt.cores[idx] = core_alloc(2, 2);
            for (int prev = 0; prev < 2; prev++)
                for (int xbit = 0; xbit < 2; xbit++)
                    for (int next = 0; next < 2; next++)
                        core_set(&tt.cores[idx], prev, xbit, next, T[xbit][prev][next]);
        }
    }
    return tt;
}

/* Scalar multiply: scale first core by c */
TT tt_scalar_mul(TT *src, ll c) {
    c = mod(c);
    TT tt = tt_alloc(src->m);
    for (int i = 0; i < src->m; i++) {
        Core *s = &src->cores[i];
        tt.cores[i] = core_alloc(s->rL, s->rR);
        size_t sz = (size_t)s->rL * 2 * s->rR;
        if (i == 0) {
            for (size_t j = 0; j < sz; j++)
                tt.cores[i].data[j] = s->data[j] * c % MOD;
        } else {
            memcpy(tt.cores[i].data, s->data, sz * sizeof(ll));
        }
    }
    return tt;
}

/* Add: a + coef_b * b. Ranks add (direct sum). */
TT tt_add(TT *a, TT *b, ll coef_b) {
    coef_b = mod(coef_b);
    int m = a->m;
    TT tt = tt_alloc(m);

    for (int i = 0; i < m; i++) {
        Core *A = &a->cores[i], *B = &b->cores[i];
        int rLa = A->rL, rRa = A->rR;
        int rLb = B->rL, rRb = B->rR;

        if (i == 0) {
            /* (1, 2, rRa+rRb) */
            tt.cores[i] = core_alloc(1, rRa + rRb);
            for (int bit = 0; bit < 2; bit++) {
                for (int r = 0; r < rRa; r++)
                    core_set(&tt.cores[i], 0, bit, r, core_get(A, 0, bit, r));
                for (int r = 0; r < rRb; r++)
                    core_set(&tt.cores[i], 0, bit, rRa + r,
                             core_get(B, 0, bit, r) * coef_b % MOD);
            }
        } else if (i == m - 1) {
            /* (rLa+rLb, 2, 1) */
            tt.cores[i] = core_alloc(rLa + rLb, 1);
            for (int l = 0; l < rLa; l++)
                for (int bit = 0; bit < 2; bit++)
                    core_set(&tt.cores[i], l, bit, 0, core_get(A, l, bit, 0));
            for (int l = 0; l < rLb; l++)
                for (int bit = 0; bit < 2; bit++)
                    core_set(&tt.cores[i], rLa + l, bit, 0, core_get(B, l, bit, 0));
        } else {
            /* block diagonal: (rLa+rLb, 2, rRa+rRb) */
            tt.cores[i] = core_alloc(rLa + rLb, rRa + rRb);
            for (int l = 0; l < rLa; l++)
                for (int bit = 0; bit < 2; bit++)
                    for (int r = 0; r < rRa; r++)
                        core_set(&tt.cores[i], l, bit, r, core_get(A, l, bit, r));
            for (int l = 0; l < rLb; l++)
                for (int bit = 0; bit < 2; bit++)
                    for (int r = 0; r < rRb; r++)
                        core_set(&tt.cores[i], rLa + l, bit, rRa + r, core_get(B, l, bit, r));
        }
    }
    return tt;
}

/* Hadamard (element-wise) product. Ranks multiply. */
TT tt_hadamard(TT *a, TT *b) {
    int m = a->m;
    TT tt = tt_alloc(m);

    for (int i = 0; i < m; i++) {
        Core *A = &a->cores[i], *B = &b->cores[i];
        int rLa = A->rL, rRa = A->rR;
        int rLb = B->rL, rRb = B->rR;
        int rL = rLa * rLb, rR = rRa * rRb;

        tt.cores[i] = core_alloc(rL, rR);
        for (int la = 0; la < rLa; la++)
            for (int lb = 0; lb < rLb; lb++) {
                int l = la * rLb + lb;
                for (int bit = 0; bit < 2; bit++) {
                    for (int ra = 0; ra < rRa; ra++) {
                        ll av = core_get(A, la, bit, ra);
                        if (!av) continue;
                        for (int rb = 0; rb < rRb; rb++) {
                            ll bv = core_get(B, lb, bit, rb);
                            if (!bv) continue;
                            int r = ra * rRb + rb;
                            ll old = core_get(&tt.cores[i], l, bit, r);
                            core_set(&tt.cores[i], l, bit, r, (old + av * bv) % MOD);
                        }
                    }
                }
            }
    }
    return tt;
}

/* Apply per-bit 2x2 matrix M: M[new_bit][old_bit] */
TT tt_apply_local(TT *src, ll M[2][2]) {
    int m = src->m;
    TT tt = tt_alloc(m);

    for (int i = 0; i < m; i++) {
        Core *S = &src->cores[i];
        int rL = S->rL, rR = S->rR;
        tt.cores[i] = core_alloc(rL, rR);

        for (int l = 0; l < rL; l++) {
            for (int r = 0; r < rR; r++) {
                ll a0 = core_get(S, l, 0, r);
                ll a1 = core_get(S, l, 1, r);
                core_set(&tt.cores[i], l, 0, r, (M[0][0] * a0 + M[0][1] * a1) % MOD);
                core_set(&tt.cores[i], l, 1, r, (M[1][0] * a0 + M[1][1] * a1) % MOD);
            }
        }
    }
    return tt;
}

/* Sum all entries */
ll tt_sum_all(TT *tt) {
    int m = tt->m;
    /* Start with left boundary [1] */
    int sz = 1;
    ll *vec = (ll *)malloc(sizeof(ll));
    vec[0] = 1;

    for (int i = 0; i < m; i++) {
        Core *c = &tt->cores[i];
        int rL = c->rL, rR = c->rR;
        ll *newvec = (ll *)calloc(rR, sizeof(ll));
        for (int l = 0; l < rL; l++) {
            if (!vec[l]) continue;
            for (int bit = 0; bit < 2; bit++) {
                for (int r = 0; r < rR; r++) {
                    newvec[r] = (newvec[r] + vec[l] * core_get(c, l, bit, r)) % MOD;
                }
            }
        }
        free(vec);
        vec = newvec;
        sz = rR;
    }
    ll result = mod(vec[0]);
    free(vec);
    return result;
}

/* Gaussian elimination over finite field for compression */
void tt_reduce_left(TT *tt) {
    int m = tt->m;

    for (int i = 0; i < m - 1; i++) {
        Core *core = &tt->cores[i];
        int rL = core->rL, rR = core->rR;
        int nrows = 2 * rL;

        /* Build matrix: rows = unfolding of core (2*rL rows, rR columns) */
        ll *mat = (ll *)malloc((size_t)nrows * rR * sizeof(ll));
        for (int l = 0; l < rL; l++) {
            for (int r = 0; r < rR; r++) {
                mat[(2*l) * rR + r] = core_get(core, l, 0, r);
                mat[(2*l+1) * rR + r] = core_get(core, l, 1, r);
            }
        }

        /* Forward elimination to find pivot columns */
        int *pivots = (int *)malloc(rR * sizeof(int));
        int rank = 0;
        int row_ptr = 0;

        for (int c = 0; c < rR && row_ptr < nrows; c++) {
            /* Find pivot */
            int piv = -1;
            for (int rr = row_ptr; rr < nrows; rr++) {
                if (mat[rr * rR + c]) { piv = rr; break; }
            }
            if (piv < 0) continue;

            /* Swap */
            if (piv != row_ptr) {
                for (int j = 0; j < rR; j++) {
                    ll tmp = mat[row_ptr * rR + j];
                    mat[row_ptr * rR + j] = mat[piv * rR + j];
                    mat[piv * rR + j] = tmp;
                }
            }

            /* Normalize */
            ll inv = modinv(mat[row_ptr * rR + c]);
            for (int j = c; j < rR; j++)
                mat[row_ptr * rR + j] = mat[row_ptr * rR + j] * inv % MOD;

            /* Eliminate below */
            for (int rr = row_ptr + 1; rr < nrows; rr++) {
                ll f = mat[rr * rR + c];
                if (f) {
                    for (int j = c; j < rR; j++)
                        mat[rr * rR + j] = mod(mat[rr * rR + j] - f * mat[row_ptr * rR + j] % MOD);
                }
            }

            pivots[rank] = c;
            rank++;
            row_ptr++;
        }

        /* Back elimination among pivot rows */
        for (int k = rank - 1; k >= 0; k--) {
            int c = pivots[k];
            for (int rr = 0; rr < k; rr++) {
                ll f = mat[rr * rR + c];
                if (f) {
                    for (int j = c; j < rR; j++)
                        mat[rr * rR + j] = mod(mat[rr * rR + j] - f * mat[k * rR + j] % MOD);
                }
            }
        }

        if (rank == rR) {
            free(mat);
            free(pivots);
            continue;
        }

        /* Build new core with only pivot columns */
        Core new_core = core_alloc(rL, rank);
        for (int l = 0; l < rL; l++) {
            for (int k = 0; k < rank; k++) {
                int p = pivots[k];
                core_set(&new_core, l, 0, k, core_get(core, l, 0, p));
                core_set(&new_core, l, 1, k, core_get(core, l, 1, p));
            }
        }

        /* Build coefficient matrix for next core */
        /* For pivot column p_k: coefficient is identity (delta_{k,k'}) */
        /* For non-pivot column j: coefficient is mat[k][j] for each pivot row k */
        Core *nxt = &tt->cores[i + 1];
        int rNext = nxt->rR;
        Core new_nxt = core_alloc(rank, rNext);

        /* Collect pivot set for fast lookup */
        char *is_pivot = (char *)calloc(rR, sizeof(char));
        for (int k = 0; k < rank; k++) is_pivot[pivots[k]] = 1;

        /* Pivot columns: identity contribution */
        for (int k = 0; k < rank; k++) {
            int p = pivots[k];
            for (int bit = 0; bit < 2; bit++)
                for (int t = 0; t < rNext; t++) {
                    ll old = core_get(&new_nxt, k, bit, t);
                    core_set(&new_nxt, k, bit, t, mod(old + core_get(nxt, p, bit, t)));
                }
        }

        /* Non-pivot columns: weighted by elimination coefficients */
        for (int j = 0; j < rR; j++) {
            if (is_pivot[j]) continue;
            for (int k = 0; k < rank; k++) {
                ll coeff = mat[k * rR + j];
                if (!coeff) continue;
                for (int bit = 0; bit < 2; bit++)
                    for (int t = 0; t < rNext; t++) {
                        ll old = core_get(&new_nxt, k, bit, t);
                        core_set(&new_nxt, k, bit, t,
                                 mod(old + coeff * core_get(nxt, j, bit, t) % MOD));
                    }
            }
        }

        /* Replace cores */
        core_free(core);
        tt->cores[i] = new_core;
        core_free(nxt);
        tt->cores[i + 1] = new_nxt;

        free(mat);
        free(pivots);
        free(is_pivot);
    }
}

ll solve_c(int n, ll b) {
    int m = 1;
    ll tmp = b;
    while (tmp > 1) { m++; tmp >>= 1; }
    if (b == 0) m = 1;

    TT mask = tt_indicator_leq(b, m);
    tt_reduce_left(&mask);

    TT dp = tt_indicator_leq(b, m);
    tt_reduce_left(&dp);

    TT ones = tt_all_ones(m);

    /* R_DISJOINT matrix: M[new_bit][old_bit] for x&y==0 predicate */
    /* rows=y_bit, cols=x_bit: (0,0)->1, (0,1)->1, (1,0)->1, (1,1)->0 */
    ll R_DISJOINT[2][2] = {{1, 1}, {1, 0}};

    for (int step = 0; step < n - 1; step++) {
        ll total = tt_sum_all(&dp);

        /* j = ones * total */
        TT j = tt_scalar_mul(&ones, total);

        /* bv = R_DISJOINT applied to dp */
        TT bv = tt_apply_local(&dp, R_DISJOINT);

        /* nxt = j + (-1)*bv = (J - B)*dp */
        TT nxt = tt_add(&j, &bv, MOD - 1);

        tt_free(&j);
        tt_free(&bv);

        /* Hadamard with mask: project to y <= b */
        TT masked = tt_hadamard(&nxt, &mask);
        tt_free(&nxt);

        /* Compress */
        tt_reduce_left(&masked);

        tt_free(&dp);
        dp = masked;
    }

    ll result = tt_sum_all(&dp);

    tt_free(&dp);
    tt_free(&mask);
    tt_free(&ones);

    return result;
}

int main(void) {
    ll ans = solve_c(123, 123456789LL);
    printf("%lld\n", ans);
    return 0;
}
