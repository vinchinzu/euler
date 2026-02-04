"""Project Euler Problem 368: A Kempner-like series.

Sum 1/n for all n whose decimal representation does NOT contain 3 or more
consecutive equal digits.

Uses Baillie's method with moment corrections:
- Solve linear system for T(s) = sum count(s,r)/10^r (generating function)
- Also track M1(s) = sum S1(s,r)/10^(2r) and M2(s) = sum S2(s,r)/10^(3r)
- Enumerate k-digit prefixes and use T, M1, M2 for tail correction
"""
import subprocess, os, tempfile

def solve():
    c_code = r"""
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
 * For j=0: T0(s) = sum count(s,r)/10^r. (Same as before.)
 * For j=1: T1(s) = sum S1(s,r)/10^{2r}.
 * For j=2: T2(s) = sum S2(s,r)/10^{3r}.
 *
 * These can be computed by solving linear systems!
 *
 * For S0(s,r) = count(s,r):
 *   count(s, r) = sum over valid d' of count(s', r-1)
 *   T0(s) = (1/10) * sum_{d'} T0(s')  + ... wait, let me redo.
 *
 * Actually: a suffix of length r starting in state s begins with digit d',
 * then the remaining r-1 digits form a suffix starting in state s' = trans(s, d').
 * The suffix value x = d' * 10^{r-1} + x'  where x' is the (r-1)-digit suffix.
 *
 * So:
 *   count(s, r) = sum_{d' valid} count(s', r-1)     [base: count(s,0) = 1]
 *   S1(s, r) = sum_{d'} [d' * 10^{r-1} * count(s', r-1) + S1(s', r-1)]
 *   S2(s, r) = sum_{d'} [(d')^2 * 10^{2(r-1)} * count(s', r-1) + 2*d'*10^{r-1}*S1(s',r-1) + S2(s',r-1)]
 *
 * Now for T0:
 *   T0(s) = sum_{r=1}^inf count(s,r)/10^r
 *         = sum_{r=1}^inf (1/10^r) * sum_{d'} count(s', r-1)
 *         = (1/10) * sum_{d'} sum_{r=1}^inf count(s', r-1)/10^{r-1}
 *         = (1/10) * sum_{d'} [count(s',0) + T0(s')]
 *         = (1/10) * sum_{d'} [1 + T0(s')]
 *
 * So: T0(s) = (N_valid(s)/10) + (1/10)*sum_{d'} T0(s')
 * where N_valid(s) = number of valid transitions from s.
 *
 * This gives: T0(s) - (1/10)*sum T0(s') = N_valid(s)/10.
 *
 * For T1:
 *   T1(s) = sum_{r=1}^inf S1(s,r)/10^{2r}
 *   S1(s,r) = sum_{d'} [d'*10^{r-1}*count(s',r-1) + S1(s',r-1)]
 *   S1(s,r)/10^{2r} = sum_{d'} [d'*count(s',r-1)/(10^{r+1}) + S1(s',r-1)/10^{2r}]
 *
 *   sum_{r=1}^inf S1(s,r)/10^{2r}
 *   = sum_{d'} sum_{r=1}^inf [d'*count(s',r-1)/(10^{r+1}) + S1(s',r-1)/10^{2r}]
 *   = sum_{d'} [d'/100 * sum_{r=0}^inf count(s',r)/10^r + (1/100)*sum_{r=0}^inf S1(s',r)/10^{2r}]
 *   = sum_{d'} [d'/100 * (1+T0(s')) + (1/100) * (0 + T1(s'))]
 *   [since S1(s',0) = 0 because suffix of length 0 has value 0]
 *
 *   T1(s) = (1/100) * sum_{d'} [d'*(1+T0(s')) + T1(s')]
 *
 * For T2:
 *   S2(s,r) = sum_{d'} [(d')^2*10^{2(r-1)}*count(s',r-1) + 2*d'*10^{r-1}*S1(s',r-1) + S2(s',r-1)]
 *   S2(s,r)/10^{3r} = sum_{d'} [(d')^2*count(s',r-1)/10^{r+2} + 2*d'*S1(s',r-1)/10^{2r+1} + S2(s',r-1)/10^{3r}]
 *
 *   T2(s) = sum_{d'} [(d')^2/1000 * (1+T0(s')) + 2*d'/1000 * (0 + T1(s')) + (1/1000)*T2(s')]
 *   [since S1(s',0)=0, S2(s',0)=0]
 *
 *   T2(s) = (1/1000)*sum_{d'} [(d')^2*(1+T0(s')) + 2*d'*T1(s') + T2(s')]
 *
 * So we solve T0 first (20x20 system), then T1 (20x20 using T0), then T2 (20x20 using T0, T1).
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

/* Valid transitions from state s: returns count and fills trans_states and trans_digits */
static int valid_trans(int s, int ns[10], int nd_arr[10]) {
    int cnt = 0;
    for (int nd = 0; nd < 10; nd++) {
        int t = trans(s, nd);
        if (t >= 0) { ns[cnt] = t; nd_arr[cnt] = nd; cnt++; }
    }
    return cnt;
}

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
    /* T1(s) = (1/100) * sum_{d'} [d'*(1+T0(s')) + T1(s')] */
    /* T1(s) - (1/100)*sum T1(s') = (1/100)*sum d'*(1+T0(s')) */
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
    /* T2(s) = (1/1000)*sum_{d'} [(d')^2*(1+T0(s')) + 2*d'*T1(s') + T2(s')] */
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
    /* Sj(s,r) for j=3 follows pattern:
     * S3(s,r) = sum_{d'} [d'^3*10^{3(r-1)}*count + 3*d'^2*10^{2(r-1)}*S1 + 3*d'*10^{r-1}*S2 + S3]
     * T3(s) = sum S3(s,r)/10^{4r}
     *       = (1/10000)*sum_{d'} [d'^3*(1+T0) + 3*d'^2*T1 + 3*d'*T2 + T3]
     */
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
    /* T4(s) = (1/10^5)*sum_{d'} [d'^4*(1+T0) + 4*d'^3*T1 + 6*d'^2*T2 + 4*d'*T3 + T4] */
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
""";

    tmpdir = tempfile.mkdtemp()
    src = os.path.join(tmpdir, "sol368.c")
    exe = os.path.join(tmpdir, "sol368")
    with open(src, 'w') as f:
        f.write(c_code)
    subprocess.run(["gcc", "-O3", "-o", exe, src, "-lm"], check=True, capture_output=True)
    result = subprocess.run([exe], capture_output=True, text=True, check=True, timeout=30)
    print(result.stdout.strip())

if __name__ == "__main__":
    solve()
