"""Project Euler Problem 748: Upside Down Diophantine Equation.

Find the sum of all solutions to 1/x^2 + 1/y^2 = 13/z^2 such that x<=y, x,y,z <= N,
and GCD(x,y,z) = 1.

Parameterize rational solutions by drawing a line with slope m/n from (3,2).
Uses C extension for performance.
"""

import os
import subprocess
import sys
import tempfile


C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef long long ll;

static const ll N = 10000000000000000LL;  /* 10^16 */
static const ll M = 1000000000LL;         /* 10^9 */

static ll ans = 0;

static ll sq(ll x) { return x * x; }

static ll gcd_func(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

static void process(ll m, ll n, int g) {
    ll a = sq(m) + sq(n);
    ll b = -2*sq(m) + 6*m*n + 2*sq(n);
    ll c = 3*sq(m) + 4*m*n - 3*sq(n);
    ll x = a * b / g;
    ll y = a * c / g;
    ll z = b * c / g;
    if (y <= N && z <= N && y > 0 && z > 0) {
        ans = (ans + x + y + z) % M;
    }
}

int main(void) {
    double A = (sqrt(6.5) - 2.0) / (3.0 - sqrt(6.5));
    double B = 2.0 / (sqrt(13.0) - 3.0);

    /* Precompute GCD table */
    int gcd_limit = (int)pow((double)N * 4.0 / 8.0, 0.25) + 2;
    /* Use 1D array for cache efficiency */
    int *gcds = NULL;
    int gs = gcd_limit + 1;
    gcds = (int*)malloc((long long)gs * gs * sizeof(int));
    if (!gcds) { fprintf(stderr, "malloc failed\n"); return 1; }
    for (int i = 0; i < gs; i++)
        for (int j = 1; j < gs; j++)
            gcds[i * gs + j] = (int)gcd_func(i, j);

    /* Section 1: bound 4*N */
    for (ll n = 1; 8 * sq(n) * sq(n) <= 4 * N; n++) {
        for (ll m = n + 1; m < B * n && (sq(m)+sq(n)) * (3*sq(m)+4*m*n-3*sq(n)) <= 4*N; m++) {
            if (m > A * n && gcds[(int)(m % n) * gs + (int)n] == 1 && (2*m - 3*n) % 13 != 0) {
                process(m, n, (m + n) % 2 == 0 ? 4 : 1);
            }
        }
    }

    /* Section 2: bound 676*N, only m = 8n (mod 13) */
    for (ll n = 1; 8 * sq(n) * sq(n) <= 676 * N; n++) {
        for (ll m = n + (7*n) % 13; m < B * n && (sq(m)+sq(n)) * (3*sq(m)+4*m*n-3*sq(n)) <= 676*N; m += 13) {
            if (m > A * n && gcd_func(m, n) == 1) {
                process(m, n, (m + n) % 2 == 0 ? 676 : 169);
            }
        }
    }

    printf("%lld\n", ans);
    free(gcds);
    return 0;
}
"""


def solve():
    tmpdir = tempfile.mkdtemp()
    c_file = os.path.join(tmpdir, "p748.c")
    exe_file = os.path.join(tmpdir, "p748")

    with open(c_file, "w") as f:
        f.write(C_CODE)

    result = subprocess.run(
        ["gcc", "-O2", "-o", exe_file, c_file, "-lm"],
        capture_output=True, text=True
    )
    if result.returncode != 0:
        print(f"Compile error: {result.stderr}", file=sys.stderr)
        sys.exit(1)

    result = subprocess.run(
        [exe_file],
        capture_output=True, text=True, timeout=25
    )

    os.unlink(c_file)
    os.unlink(exe_file)
    os.rmdir(tmpdir)

    return result.stdout.strip()


if __name__ == "__main__":
    print(solve())
