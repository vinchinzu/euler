#!/usr/bin/env python3
"""Project Euler Problem 408: Admissible paths through a square grid.

Find the number of admissible paths from (0,0) to (N,N).
A point (x,y) is inadmissible if x, y, and x+y are all perfect squares.
Uses inclusion-exclusion with modular binomial coefficients.
Implemented in C for performance.
"""
import ctypes
import os
import subprocess
import sys
import tempfile

C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define N 10000000
#define MOD 1000000007LL
#define MAX_PTS 10000
#define MAX_FACT (2*N+1)

static long long fact[MAX_FACT];
static long long inv_fact[MAX_FACT];

long long power(long long base, long long exp, long long mod) {
    long long result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

void precompute() {
    fact[0] = 1;
    for (int i = 1; i < MAX_FACT; i++)
        fact[i] = fact[i-1] * i % MOD;
    inv_fact[MAX_FACT-1] = power(fact[MAX_FACT-1], MOD-2, MOD);
    for (int i = MAX_FACT-2; i >= 0; i--)
        inv_fact[i] = inv_fact[i+1] * (i+1) % MOD;
}

long long nCr(int n, int r) {
    if (r < 0 || r > n) return 0;
    return fact[n] % MOD * inv_fact[r] % MOD * inv_fact[n-r] % MOD;
}

int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

typedef struct { int x, y; } Point;

int cmp_points(const void *a, const void *b) {
    const Point *pa = (const Point *)a;
    const Point *pb = (const Point *)b;
    if (pa->x != pb->x) return pa->x - pb->x;
    return pa->y - pb->y;
}

int main() {
    precompute();

    Point pts[MAX_PTS];
    int npts = 0;

    /* Generate inadmissible points via Pythagorean triples */
    int sq_limit = (int)sqrt((double)N);
    int m_limit = (int)sqrt(4.0 * sq_limit) + 1;

    for (int m = 2; m <= m_limit; m++) {
        for (int n = 1; n < m; n++) {
            if ((m + n) % 2 == 1 && gcd(m, n) == 1) {
                int a = m*m - n*n;
                int b = 2*m*n;
                int c = m*m + n*n;
                for (int k = 1; (long long)k*c <= 4*sq_limit; k++) {
                    long long ax = (long long)(k*a)*(k*a);
                    long long bx = (long long)(k*b)*(k*b);
                    if (ax <= N && bx <= N) {
                        pts[npts].x = (int)ax;
                        pts[npts].y = (int)bx;
                        npts++;
                    }
                    if (bx <= N && ax <= N) {
                        pts[npts].x = (int)bx;
                        pts[npts].y = (int)ax;
                        npts++;
                    }
                }
            }
        }
    }

    /* Remove duplicates */
    qsort(pts, npts, sizeof(Point), cmp_points);
    int unique = 0;
    for (int i = 0; i < npts; i++) {
        if (i == 0 || pts[i].x != pts[i-1].x || pts[i].y != pts[i-1].y) {
            pts[unique++] = pts[i];
        }
    }
    npts = unique;

    /* Add destination point (N, N) at the end */
    pts[npts].x = N;
    pts[npts].y = N;
    npts++;

    /* Sort again */
    qsort(pts, npts, sizeof(Point), cmp_points);

    /* Inclusion-exclusion: for each point p, compute admissible paths from (0,0) to p */
    long long *adm = (long long *)calloc(npts, sizeof(long long));

    for (int pi = 0; pi < npts; pi++) {
        long long total = nCr(pts[pi].x + pts[pi].y, pts[pi].x);
        for (int qi = 0; qi < pi; qi++) {
            if (pts[qi].x <= pts[pi].x && pts[qi].y <= pts[pi].y) {
                int dx = pts[pi].x - pts[qi].x;
                int dy = pts[pi].y - pts[qi].y;
                total = (total - adm[qi] % MOD * nCr(dx + dy, dx) % MOD + MOD) % MOD;
            }
        }
        adm[pi] = (total % MOD + MOD) % MOD;
    }

    printf("%lld\n", adm[npts-1]);
    free(adm);
    return 0;
}
"""

def solve():
    # Write C code to temp file, compile, and run
    tmpdir = tempfile.mkdtemp()
    c_file = os.path.join(tmpdir, "p408.c")
    exe_file = os.path.join(tmpdir, "p408")

    with open(c_file, "w") as f:
        f.write(C_CODE)

    subprocess.run(
        ["gcc", "-O2", "-o", exe_file, c_file, "-lm"],
        check=True, capture_output=True
    )

    result = subprocess.run(
        [exe_file], capture_output=True, text=True, check=True
    )
    return int(result.stdout.strip())

if __name__ == "__main__":
    print(solve())
