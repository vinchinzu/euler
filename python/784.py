"""Project Euler Problem 784: Reciprocal Pairs.

For valid (p,q), we need D = p+q | p^2-1 with q > p.
Equivalently, find small divisors d of p^2-1 with d <= (p-1)/2;
then D = (p^2-1)/d gives p+q = D.

Approach: Factor p^2-1 = (p-1)(p+1) via SPF sieve, merge prime factors
(handling shared factor of 2), then DFS to enumerate small divisors with
pruning. Factors sorted by decreasing prime for aggressive early pruning.
Parallelized with OpenMP.
"""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdint.h>
#include <omp.h>

#define N 2000000

static int spf[N + 2];

void compute_spf() {
    for (int i = 0; i <= N + 1; i++) spf[i] = i;
    for (int i = 2; (long long)i * i <= N + 1; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j <= N + 1; j += i)
                if (spf[j] == j) spf[j] = i;
        }
    }
}

typedef struct { int p; int e; } PF;

static inline void dfs(const PF *f, int nf, int idx, long long d, long long m,
                long long limit, unsigned long long *ans) {
    if (d > limit) return;
    if (idx == nf) { *ans += m / d; return; }
    long long pp = f[idx].p;
    int e = f[idx].e;
    long long dd = d;
    for (int i = 0; i <= e; i++) {
        if (dd > limit) break;
        dfs(f, nf, idx + 1, dd, m, limit, ans);
        dd *= pp;
    }
}

int main() {
    compute_spf();
    unsigned long long ans = 0;

    #pragma omp parallel for reduction(+:ans) schedule(guided)
    for (long long p = 2; p <= N; p++) {
        long long m = p * p - 1;
        long long limit = (p - 1) / 2;
        PF factors[20];
        int nf = 0;
        int pm = (int)(p - 1), pp = (int)(p + 1);
        int tc = __builtin_ctz(pm); pm >>= tc;
        int tc2 = __builtin_ctz(pp); pp >>= tc2;
        tc += tc2;
        if (tc) { factors[nf].p = 2; factors[nf++].e = tc; }
        while (pm > 1) {
            int q = spf[pm]; int e = 0;
            do { pm /= q; e++; } while (pm % q == 0);
            factors[nf].p = q; factors[nf++].e = e;
        }
        while (pp > 1) {
            int q = spf[pp]; int e = 0;
            do { pp /= q; e++; } while (pp % q == 0);
            factors[nf].p = q; factors[nf++].e = e;
        }

        /* Sort by decreasing prime for better pruning */
        for (int i = 0; i < nf - 1; i++)
            for (int j = i + 1; j < nf; j++)
                if (factors[j].p > factors[i].p) {
                    PF tmp = factors[i]; factors[i] = factors[j]; factors[j] = tmp;
                }

        unsigned long long la = 0;
        dfs(factors, nf, 0, 1, m, limit, &la);
        ans += la;
    }

    printf("%llu\n", ans);
    return 0;
}
'''

    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name

    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-march=native', '-fopenmp', '-o', exe, c_file],
                   check=True, capture_output=True)
    result = subprocess.check_output([exe]).decode().strip()

    os.unlink(c_file)
    os.unlink(exe)

    return int(result)


if __name__ == "__main__":
    print(solve())
