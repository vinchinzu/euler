"""Project Euler Problem 438 — Integer part of polynomial equation's solutions."""
import subprocess, tempfile, os

C_CODE = r'''
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define N 7

static double ineq[N+1][(N+1)][(N+1)];
static long long ans;

static void helper(int *t, int tlen) {
    int index = tlen + 1;
    double lower_bound = -1e18;
    double upper_bound = 1e18;

    for (int j = 0; j <= index; j++) {
        double goal = -ineq[index][j][0];
        for (int k = 0; k < index - 1; k++) {
            goal -= ineq[index][j][k + 1] * t[k];
        }
        goal /= ineq[index][j][index];

        if (j % 2 == 0) {
            double c = ceil(goal);
            if (c > lower_bound) lower_bound = c;
        } else {
            double f = floor(goal);
            if (f < upper_bound) upper_bound = f;
        }
    }

    long long lb = (long long)lower_bound;
    long long ub = (long long)upper_bound;

    if (index == N) {
        long long num_terms = ub - lb + 1;
        if (num_terms > 0) {
            long long sum_prefix = 0;
            for (int k = 0; k < tlen; k++) {
                sum_prefix += llabs((long long)t[k]);
            }
            /* Sum of |a| for a = lb..ub */
            long long sum_abs;
            if (lb >= 0) {
                sum_abs = (ub * (ub + 1) - lb * (lb - 1)) / 2;
            } else if (ub <= 0) {
                long long nlb = -lb, nub = -ub;
                sum_abs = (nlb * (nlb + 1) - nub * (nub - 1)) / 2;
            } else {
                long long nlb = -lb;
                long long sum_neg = nlb * (nlb + 1) / 2;
                long long sum_pos = ub * (ub + 1) / 2;
                sum_abs = sum_neg + sum_pos;
            }
            ans += num_terms * sum_prefix + sum_abs;
        }
        return;
    }

    for (long long a_r = lb; a_r <= ub; a_r++) {
        t[tlen] = (int)a_r;
        helper(t, tlen + 1);
    }
}

int main(void) {
    double EPS = 1.0;
    for (int i = 1; i <= N; i++) EPS /= i;

    /* Base level: evaluate polynomial at N+1-j-EPS for j=0..N */
    for (int j = 0; j <= N; j++) {
        double x = N + 1 - j - EPS;
        for (int k = 0; k <= N; k++) {
            ineq[N][j][k] = pow(x, N - k);
        }
    }

    /* Take differences to eliminate variables */
    for (int i = N - 1; i >= 1; i--) {
        for (int j = 0; j <= i; j++) {
            for (int k = 0; k <= i; k++) {
                ineq[i][j][k] = ineq[i+1][j][k] - ineq[i+1][j+1][k];
            }
        }
    }

    ans = 0;
    int t[N];
    helper(t, 0);

    printf("%lld\n", ans);
    return 0;
}
'''

if __name__ == "__main__":
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(C_CODE.encode())
        c_file = f.name
    exe = c_file[:-2]
    try:
        subprocess.run(['gcc', '-O2', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
        result = subprocess.run([exe], capture_output=True, text=True, timeout=280)
        print(result.stdout.strip())
    finally:
        os.unlink(c_file)
        if os.path.exists(exe):
            os.unlink(exe)
