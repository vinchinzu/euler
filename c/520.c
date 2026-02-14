/*
 * Project Euler Problem 520: Simbers.
 * Matrix exponentiation for counting Simbers with digit parity constraints.
 * State: (zero_odd, odd_odd, even_odd, odd_even, even_even, empty)
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000123LL
#define B 10

/* State: zero_odd, odd_odd, even_odd, odd_even, even_even, empty
 * Constraints: zero_odd + odd_odd + even_odd = B/2 = 5 (odd digits)
 *              odd_even + even_even = B/2 = 5 (even digits)
 * Start state: (5, 0, 0, 0, 5, true)
 */

typedef struct {
    int zero_odd, odd_odd, even_odd, odd_even, even_even;
    int empty;
} State;

State states[1000];
int n_states;

/* Map state to index */
int state_index[6][6][6][6][6][2]; /* [zo][oo][eo][oe][ee][empty] */

void generate_states() {
    memset(state_index, -1, sizeof(state_index));
    n_states = 0;

    /* Start state first */
    states[0] = (State){5, 0, 0, 0, 5, 1};
    state_index[5][0][0][0][5][1] = 0;
    n_states = 1;

    /* All non-empty states */
    for (int zo = 0; zo <= 5; zo++) {
        for (int oo = 0; oo <= 5 - zo; oo++) {
            int eo = 5 - zo - oo;
            for (int oe = 0; oe <= 5; oe++) {
                int ee = 5 - oe;
                if (state_index[zo][oo][eo][oe][ee][0] < 0) {
                    states[n_states] = (State){zo, oo, eo, oe, ee, 0};
                    state_index[zo][oo][eo][oe][ee][0] = n_states;
                    n_states++;
                }
            }
        }
    }
}

int get_idx(State s) {
    return state_index[s.zero_odd][s.odd_odd][s.even_odd][s.odd_even][s.even_even][s.empty];
}

/* Matrix operations */
typedef struct {
    ll *data;
    int n;
} Matrix;

Matrix mat_alloc(int n) {
    Matrix m;
    m.n = n;
    m.data = (ll*)calloc((ll)n * n, sizeof(ll));
    return m;
}

void mat_free(Matrix *m) {
    free(m->data);
}

#define MAT(m, i, j) ((m).data[(ll)(i) * (m).n + (j)])

Matrix mat_multiply(Matrix a, Matrix b) {
    int n = a.n;
    Matrix c = mat_alloc(n);
    for (int i = 0; i < n; i++) {
        for (int k = 0; k < n; k++) {
            if (MAT(a, i, k) == 0) continue;
            for (int j = 0; j < n; j++) {
                MAT(c, i, j) = (MAT(c, i, j) + (lll)MAT(a, i, k) * MAT(b, k, j)) % MOD;
            }
        }
    }
    return c;
}

int main() {
    int N = 39;

    generate_states();

    /* Build transition matrix A */
    Matrix A = mat_alloc(n_states);

    for (int si = 0; si < n_states; si++) {
        State s = states[si];

        if (s.zero_odd > 0) {
            State ns = {s.zero_odd - 1, s.odd_odd + 1, s.even_odd, s.odd_even, s.even_even, 0};
            int ni = get_idx(ns);
            MAT(A, ni, si) = (MAT(A, ni, si) + s.zero_odd) % MOD;
        }

        if (s.odd_odd > 0) {
            State ns = {s.zero_odd, s.odd_odd - 1, s.even_odd + 1, s.odd_even, s.even_even, 0};
            int ni = get_idx(ns);
            MAT(A, ni, si) = (MAT(A, ni, si) + s.odd_odd) % MOD;
        }

        if (s.even_odd > 0) {
            State ns = {s.zero_odd, s.odd_odd + 1, s.even_odd - 1, s.odd_even, s.even_even, 0};
            int ni = get_idx(ns);
            MAT(A, ni, si) = (MAT(A, ni, si) + s.even_odd) % MOD;
        }

        if (s.odd_even > 0) {
            State ns = {s.zero_odd, s.odd_odd, s.even_odd, s.odd_even - 1, s.even_even + 1, 0};
            int ni = get_idx(ns);
            MAT(A, ni, si) = (MAT(A, ni, si) + s.odd_even) % MOD;
        }

        if (s.even_even > 0) {
            State ns = {s.zero_odd, s.odd_odd, s.even_odd, s.odd_even + 1, s.even_even - 1, 0};
            int ni = get_idx(ns);
            if (s.empty) {
                /* Leading zero: stays in same state for one of them */
                MAT(A, si, si) = (MAT(A, si, si) + 1) % MOD;
                MAT(A, ni, si) = (MAT(A, ni, si) + s.even_even - 1) % MOD;
            } else {
                MAT(A, ni, si) = (MAT(A, ni, si) + s.even_even) % MOD;
            }
        }
    }

    /* Compute A^(2^u) for u = 1 to N, summing accepting states */
    int start_idx = 0; /* Start state index */

    /* Identify accepting states: even_odd == 0, odd_even == 0, not empty */
    int *accepting = (int*)calloc(n_states, sizeof(int));
    for (int i = 0; i < n_states; i++) {
        State s = states[i];
        if (s.even_odd == 0 && s.odd_even == 0 && !s.empty)
            accepting[i] = 1;
    }

    /* An starts as A */
    Matrix An = mat_multiply(A, A); /* A^2 = A^(2^1) */
    ll ans = 0;

    /* u=1: An = A^2 */
    for (int i = 0; i < n_states; i++)
        if (accepting[i])
            ans = (ans + MAT(An, i, start_idx)) % MOD;

    for (int u = 2; u <= N; u++) {
        Matrix An2 = mat_multiply(An, An);
        mat_free(&An);
        An = An2;
        for (int i = 0; i < n_states; i++)
            if (accepting[i])
                ans = (ans + MAT(An, i, start_idx)) % MOD;
    }

    printf("%lld\n", ans);

    mat_free(&An);
    mat_free(&A);
    free(accepting);
    return 0;
}
