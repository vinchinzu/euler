/*
 * Project Euler Problem 584: Birthday Problem Revisited.
 *
 * Expected number of people before K=4 share birthdays within 7 days (Earth, D=365 days).
 * "Within 7 days" means a window of W=8 consecutive days (gap <= 7).
 *
 * E = sum_{n=0}^{inf} P(n people don't violate the constraint)
 * P(n) = n!/D^n * [x^n] Trace(T^D)
 * where T is the transfer matrix for a circular arrangement of D=365 days,
 * states are (c_{d-6},...,c_d) with sum <= 3, and T transitions by adding a new day.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define D 365
#define W 8           // "within 7 days" = window of 8 consecutive days
#define STATE_LEN 7   // W-1
#define MAX_OCC 3     // K-1
#define MAX_N 100     // truncation for polynomial

// Generate states: 7-tuples with values in [0, MAX_OCC] and sum <= MAX_OCC
typedef struct {
    int c[STATE_LEN];
} State;

#define MAX_STATES 200
State states[MAX_STATES];
int num_states = 0;

// state_map: 4^7 = 16384 entries
#define MAP_SIZE 16384
int state_map[MAP_SIZE];

int tuple_to_index(int c[]) {
    int idx = 0;
    for (int i = 0; i < STATE_LEN; i++) {
        idx = idx * 4 + c[i];
    }
    return idx;
}

void gen_states_rec(int pos, int remaining, int c[]) {
    if (pos == STATE_LEN) {
        int idx = tuple_to_index(c);
        state_map[idx] = num_states;
        for (int i = 0; i < STATE_LEN; i++)
            states[num_states].c[i] = c[i];
        num_states++;
        return;
    }
    for (int v = 0; v <= remaining; v++) {
        c[pos] = v;
        gen_states_rec(pos + 1, remaining - v, c);
    }
}

void gen_states() {
    memset(state_map, -1, sizeof(state_map));
    int c[STATE_LEN];
    memset(c, 0, sizeof(c));
    gen_states_rec(0, MAX_OCC, c);
}

// Polynomial matrix operations
double *mat_alloc(int ns) {
    return (double *)calloc((long long)ns * ns * (MAX_N + 1), sizeof(double));
}

#define MAT(m, i, j, k) m[((long long)(i) * num_states + (j)) * (MAX_N + 1) + (k)]

// Matrix multiplication: C = A * B (polynomial matrix multiply with truncation)
void mat_mul(const double *A, const double *B, double *C, int ns) {
    int plen = MAX_N + 1;
    long long total = (long long)ns * ns * plen;
    memset(C, 0, total * sizeof(double));

    for (int i = 0; i < ns; i++) {
        for (int k = 0; k < ns; k++) {
            const double *a = &A[((long long)i * ns + k) * plen];
            // Check if a is zero
            int nonzero = 0;
            for (int p = 0; p < plen; p++) if (a[p] != 0.0) { nonzero = 1; break; }
            if (!nonzero) continue;

            for (int j = 0; j < ns; j++) {
                const double *b = &B[((long long)k * ns + j) * plen];
                double *c = &C[((long long)i * ns + j) * plen];
                // c += truncated_conv(a, b)
                for (int p = 0; p < plen; p++) {
                    if (a[p] == 0.0) continue;
                    double ap = a[p];
                    for (int q = 0; q < plen - p; q++) {
                        c[p + q] += ap * b[q];
                    }
                }
            }
        }
    }
}

int main() {
    gen_states();

    int ns = num_states;

    double inv_fact[4] = {1.0, 1.0, 0.5, 1.0/6.0};

    // Build transfer matrix T
    double *T = mat_alloc(ns);

    for (int si = 0; si < ns; si++) {
        int s_sum = 0;
        for (int k = 0; k < STATE_LEN; k++) s_sum += states[si].c[k];
        for (int c_new = 0; c_new <= MAX_OCC - s_sum; c_new++) {
            // New state: shift left by 1, append c_new
            int nc[STATE_LEN];
            for (int k = 0; k < STATE_LEN - 1; k++) nc[k] = states[si].c[k + 1];
            nc[STATE_LEN - 1] = c_new;
            int mi = tuple_to_index(nc);
            int sj = state_map[mi];
            if (sj >= 0) {
                MAT(T, si, sj, c_new) += inv_fact[c_new];
            }
        }
    }

    // Compute T^D using matrix exponentiation (binary method)
    double *result = mat_alloc(ns);
    double *temp = mat_alloc(ns);

    // Initialize result = I
    for (int i = 0; i < ns; i++) {
        MAT(result, i, i, 0) = 1.0;
    }

    double *base = mat_alloc(ns);
    memcpy(base, T, (long long)ns * ns * (MAX_N + 1) * sizeof(double));

    int power = D;
    while (power > 0) {
        if (power & 1) {
            mat_mul(result, base, temp, ns);
            double *swap = result; result = temp; temp = swap;
        }
        if (power > 1) {
            mat_mul(base, base, temp, ns);
            double *swap = base; base = temp; temp = swap;
        }
        power >>= 1;
    }

    // Trace: sum of diagonal polynomial entries
    double trace[MAX_N + 1];
    memset(trace, 0, sizeof(trace));
    for (int i = 0; i < ns; i++) {
        for (int k = 0; k <= MAX_N; k++) {
            trace[k] += MAT(result, i, i, k);
        }
    }

    // E = sum_{n=0}^{max_n} n! / D^n * trace[n]
    double E = 0.0;
    double n_fact = 1.0;
    double D_pow = 1.0;

    for (int n = 0; n <= MAX_N; n++) {
        if (n > 0) {
            n_fact *= n;
            D_pow *= D;
        }
        double p = n_fact / D_pow * trace[n];
        E += p;
        if (n > 50 && fabs(p) < 1e-15) break;
    }

    printf("%.8f\n", E);

    free(result);
    free(temp);
    free(base);
    free(T);
    return 0;
}
