#!/usr/bin/env python3
"""Project Euler Problem 352 - Blood Tests (Optimal Group Testing)

T(s,p) = expected number of tests to screen s sheep, each infected with prob p.
Compute sum T(10000, p) for p = 0.01, 0.02, ..., 0.50.

Uses C for performance: DP with T[n] and U[n] arrays.
T[n] = min expected tests for n sheep (unconditional).
U[n] = min expected tests for n sheep given >= 1 infected.
"""
import subprocess, os, tempfile

C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define S 10000
#define MAX_SPLIT 200

static double T[S + 1];
static double U[S + 1];
static double QP[S + 1];

void precompute_qpow(double q) {
    QP[0] = 1.0;
    for (int i = 1; i <= S; i++) {
        QP[i] = QP[i-1] * q;
        if (QP[i] < 1e-300) QP[i] = 0.0;
    }
}

double solve_one(double p) {
    double q = 1.0 - p;
    precompute_qpow(q);

    T[0] = 0.0;
    T[1] = 1.0;
    U[0] = 0.0;
    U[1] = 0.0;  /* Known infected single sheep: no test needed */

    for (int n = 2; n <= S; n++) {
        double denom = 1.0 - QP[n];
        double best_u;

        if (denom <= 1e-300) {
            best_u = (double)n;
        } else {
            best_u = (double)n;

            /* Split strategies for U[n]:
               Test first g as pool. Given >= 1 infected in n:
               P(first g healthy | >= 1 in n) = (q^g - q^n)/(1-q^n)
               P(>= 1 in first g | >= 1 in n) = (1-q^g)/(1-q^n)
               If healthy: U[n-g]. If infected: U[g] + T[n-g]. */
            int lim = n - 1;
            if (lim > MAX_SPLIT) lim = MAX_SPLIT;
            for (int g = 1; g <= lim; g++) {
                double qg = QP[g];
                double p_g_healthy = (qg - QP[n]) / denom;
                double p_g_infected = 1.0 - p_g_healthy;
                double cost = 1.0 + p_g_healthy * U[n - g] + p_g_infected * (U[g] + T[n - g]);
                if (cost < best_u) best_u = cost;
            }
        }
        U[n] = best_u;

        /* Compute T[n] */
        double best_t = (double)n;  /* individual testing */

        /* Pool all n: 1 test, if negative done, if positive resolve with U[n] */
        double qn = QP[n];
        double pool_all = 1.0 + (1.0 - qn) * U[n];
        if (pool_all < best_t) best_t = pool_all;

        /* Split g / (n-g): test first g as pool */
        int lim = n - 1;
        if (lim > MAX_SPLIT) lim = MAX_SPLIT;
        for (int g = 1; g <= lim; g++) {
            double qg = QP[g];
            double cost = 1.0 + qg * T[n - g] + (1.0 - qg) * (U[g] + T[n - g]);
            if (cost < best_t) best_t = cost;
        }

        T[n] = best_t;
    }

    return T[S];
}

int main(void) {
    double total = 0.0;
    for (int p_idx = 1; p_idx <= 50; p_idx++) {
        double p = p_idx * 0.01;
        double result = solve_one(p);
        total += result;
    }
    printf("%.6f\n", total);
    return 0;
}
"""

def solve():
    tmpdir = tempfile.mkdtemp()
    src = os.path.join(tmpdir, "p352.c")
    exe = os.path.join(tmpdir, "p352")
    with open(src, "w") as f:
        f.write(C_CODE)
    r = subprocess.run(["gcc", "-O2", "-o", exe, src, "-lm"], capture_output=True, text=True)
    if r.returncode != 0:
        import sys
        print(r.stderr, file=sys.stderr)
        return None
    result = subprocess.run([exe], capture_output=True, text=True, check=True, timeout=120)
    return result.stdout.strip()

if __name__ == "__main__":
    print(solve())
