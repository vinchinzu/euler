#include <stdio.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>

/*
 * Kempner-like series: sum 1/n for n without 3+ consecutive equal digits.
 *
 * Algorithm:
 * 1. Enumerate all valid k-digit numbers exactly (sum 1/n).
 * 2. For each valid k-digit prefix p in state s, compute tail:
 *    tail(p) = sum_{r=1}^{inf} sum over valid r-digit suffixes x of 1/(p*10^r + x)
 *
 * For the tail, use:
 *    1/(p*10^r + x) = (1/(p*10^r)) * sum_{j=0}^{J} (-x/(p*10^r))^j + O(...)
 *                   = sum_{j=0}^{J} (-1)^j * x^j / (p*10^r)^{j+1}
 *
 * So: tail(p) = sum_{r=1}^{inf} sum_{j=0}^{J} (-1)^j / (p*10^r)^{j+1} * Sj(s,r)
 *    where Sj(s,r) = sum over valid r-digit suffixes x of x^j.
 *
 * Regroup by j:
 * tail(p) = sum_{j=0}^{J} (-1)^j / p^{j+1} * sum_{r=1}^{inf} Sj(s,r) / 10^{r*(j+1)}
 *         = sum_{j=0}^{J} (-1)^j / p^{j+1} * Tj(s)
 *
 * where Tj(s) = sum_{r=1}^{inf} Sj(s,r) / 10^{r*(j+1)}.
 *
 * These can be computed by solving linear systems!
 *
 * So we solve T0 first (20x20 system), then T1, T2, T3, T4.
 *
 * Final formula:
 * tail(p,s) = T0(s)/p - T1(s)/p^2 + T2(s)/p^3 - ...
 *
 * Total = sum over 1..k-digit good numbers of 1/n
 *       + sum over k-digit good prefixes p with state s of tail(p,s)
 */

#define NSTATES 20
#define ENC(d, c) ((d) * 2 + (c) - 1)
#define DIG(s) ((s) / 2)
#define CON(s) ((s) % 2 + 1)

static inline int trans(int s, int nd) {
    int d = DIG(s), c = CON(s);
    if (nd == d) {
        if (c + 1 > 2) return -1;
        return ENC(nd, c + 1);
    }
    return ENC(nd, 1);
}

/* Gaussian elimination for 20x20 system */
static void gauss(double A[NSTATES][NSTATES+1], double x[NSTATES]) {
    int n = NSTATES;
    for (int col = 0; col < n; col++) {
        int pivot = col;
        for (int row = col+1; row < n; row++)
            if (fabs(A[row][col]) > fabs(A[pivot][col])) pivot = row;
        for (int j = 0; j <= n; j++) {
            double tmp = A[col][j]; A[col][j] = A[pivot][j]; A[pivot][j] = tmp;
        }
        for (int row = col+1; row < n; row++) {
            double f = A[row][col] / A[col][col];
            for (int j = col; j <= n; j++) A[row][j] -= f * A[col][j];
        }
    }
    for (int row = n-1; row >= 0; row--) {
        double s = A[row][n];
        for (int j = row+1; j < n; j++) s -= A[row][j] * x[j];
        x[row] = s / A[row][row];
    }
}

static double T0[NSTATES], T1[NSTATES], T2[NSTATES], T3[NSTATES], T4[NSTATES];
static long double direct_sum, tail_sum;

static void compute_T0(void) {
    double A[NSTATES][NSTATES+1];
    memset(A, 0, sizeof(A));
    for (int s = 0; s < NSTATES; s++) {
        A[s][s] = 1.0;
        int nv = 0;
        for (int nd = 0; nd < 10; nd++) {
            int t = trans(s, nd);
            if (t >= 0) { A[s][t] -= 0.1; nv++; }
        }
        A[s][NSTATES] = nv / 10.0;
    }
    gauss(A, T0);
}

static void compute_T1(void) {
    double A[NSTATES][NSTATES+1];
    memset(A, 0, sizeof(A));
    for (int s = 0; s < NSTATES; s++) {
        A[s][s] = 1.0;
        double rhs = 0;
        for (int nd = 0; nd < 10; nd++) {
            int t = trans(s, nd);
            if (t < 0) continue;
            A[s][t] -= 0.01;
            rhs += nd * (1.0 + T0[t]);
        }
        A[s][NSTATES] = rhs / 100.0;
    }
    gauss(A, T1);
}

static void compute_T2(void) {
    double A[NSTATES][NSTATES+1];
    memset(A, 0, sizeof(A));
    for (int s = 0; s < NSTATES; s++) {
        A[s][s] = 1.0;
        double rhs = 0;
        for (int nd = 0; nd < 10; nd++) {
            int t = trans(s, nd);
            if (t < 0) continue;
            A[s][t] -= 0.001;
            rhs += (double)nd*nd * (1.0 + T0[t]) + 2.0*nd*T1[t];
        }
        A[s][NSTATES] = rhs / 1000.0;
    }
    gauss(A, T2);
}

static void compute_T3(void) {
    double A[NSTATES][NSTATES+1];
    memset(A, 0, sizeof(A));
    for (int s = 0; s < NSTATES; s++) {
        A[s][s] = 1.0;
        double rhs = 0;
        for (int nd = 0; nd < 10; nd++) {
            int t = trans(s, nd);
            if (t < 0) continue;
            double d = nd;
            A[s][t] -= 1e-4;
            rhs += d*d*d*(1.0+T0[t]) + 3*d*d*T1[t] + 3*d*T2[t];
        }
        A[s][NSTATES] = rhs * 1e-4;
    }
    gauss(A, T3);
}

static void compute_T4(void) {
    double A[NSTATES][NSTATES+1];
    memset(A, 0, sizeof(A));
    for (int s = 0; s < NSTATES; s++) {
        A[s][s] = 1.0;
        double rhs = 0;
        for (int nd = 0; nd < 10; nd++) {
            int t = trans(s, nd);
            if (t < 0) continue;
            double d = nd;
            A[s][t] -= 1e-5;
            rhs += d*d*d*d*(1.0+T0[t]) + 4*d*d*d*T1[t] + 6*d*d*T2[t] + 4*d*T3[t];
        }
        A[s][NSTATES] = rhs * 1e-5;
    }
    gauss(A, T4);
}

/* Enumerate valid k-digit numbers */
static void enumerate(long long prefix, int state, int digits_left) {
    if (digits_left == 0) {
        if (prefix <= 0) return;
        /* Direct sum */
        direct_sum += 1.0L / (long double)prefix;
        /* Tail: T0/p - T1/p^2 + T2/p^3 - T3/p^4 + T4/p^5 - ... */
        long double p = (long double)prefix;
        long double inv_p = 1.0L / p;
        long double tail = (long double)T0[state] * inv_p
                         - (long double)T1[state] * inv_p * inv_p
                         + (long double)T2[state] * inv_p * inv_p * inv_p
                         - (long double)T3[state] * inv_p * inv_p * inv_p * inv_p
                         + (long double)T4[state] * inv_p * inv_p * inv_p * inv_p * inv_p;
        tail_sum += tail;
        return;
    }

    for (int nd = 0; nd < 10; nd++) {
        if (prefix == 0 && nd == 0) continue;
        int ns;
        if (state < 0) ns = ENC(nd, 1);
        else { ns = trans(state, nd); if (ns < 0) continue; }
        enumerate(prefix * 10 + nd, ns, digits_left - 1);
    }
}

/* Enumerate shorter numbers (exact computation) */
static void enumerate_short(long long prefix, int state, int digits_left) {
    if (digits_left == 0) {
        if (prefix > 0) direct_sum += 1.0L / (long double)prefix;
        return;
    }
    for (int nd = 0; nd < 10; nd++) {
        if (prefix == 0 && nd == 0) continue;
        int ns;
        if (state < 0) ns = ENC(nd, 1);
        else { ns = trans(state, nd); if (ns < 0) continue; }
        enumerate_short(prefix * 10 + nd, ns, digits_left - 1);
    }
}

int main(void) {
    int PREFIX_LEN = 7;

    compute_T0();
    compute_T1();
    compute_T2();
    compute_T3();
    compute_T4();

    direct_sum = 0.0L;
    tail_sum = 0.0L;

    /* Exact sum for 1 to PREFIX_LEN-1 digit numbers */
    for (int d = 1; d < PREFIX_LEN; d++)
        enumerate_short(0, -1, d);

    /* PREFIX_LEN digit numbers: direct sum + tail */
    enumerate(0, -1, PREFIX_LEN);

    long double total = direct_sum + tail_sum;
    printf("%.10Lf\n", total);
    return 0;
}
