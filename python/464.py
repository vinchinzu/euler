"""Project Euler Problem 464: Mobius function and balanced pairs.

Find the number of pairs 1 <= a <= b <= n such that the P(a, b), the number of
integers n in [a, b] where mu(n) = 1, and N(a, b), the number of integers where
mu(n) = -1, satisfy 99 N(a,b) <= 100 P(a,b) and 99 P(a,b) <= 100 N(a,b).
"""

from __future__ import annotations

import ctypes
import os
import subprocess
import tempfile
from math import isqrt


def solve() -> int:
    """Solve Problem 464."""
    N = 20_000_000
    K = 100
    L = K * isqrt(N)

    c_code = r"""
#include <stdlib.h>
#include <string.h>

static void pre_mobius(int limit, signed char *mu) {
    char *is_prime = (char *)calloc(limit + 1, 1);
    for (int i = 0; i <= limit; i++) {
        is_prime[i] = (i >= 2) ? 1 : 0;
        mu[i] = 1;
    }
    for (int i = 2; i <= limit; i++) {
        if (is_prime[i]) {
            for (int j = i; j <= limit; j += i) {
                if (j != i) is_prime[j] = 0;
                if ((long long)j % ((long long)i * i) == 0)
                    mu[j] = 0;
                else
                    mu[j] = -mu[j];
            }
        }
    }
    free(is_prime);
}

/* BIT (Fenwick tree) */
static long long *bit_tree;
static int bit_size;

static void bit_init(int size) {
    bit_size = size + 2;
    bit_tree = (long long *)calloc(bit_size + 1, sizeof(long long));
}

static void bit_free(void) {
    free(bit_tree);
}

static void bit_add(int idx, long long val) {
    idx++;
    while (idx <= bit_size) {
        bit_tree[idx] += val;
        idx += idx & (-idx);
    }
}

static long long bit_sum(int idx) {
    idx++;
    long long result = 0;
    while (idx > 0) {
        result += bit_tree[idx];
        idx -= idx & (-idx);
    }
    return result;
}

long long solve(int N, int K, int L) {
    signed char *mu = (signed char *)calloc(N + 1, 1);
    pre_mobius(N, mu);

    long long ans = (long long)N * (N + 1) / 2;

    for (int sign = 1; sign >= -1; sign -= 2) {
        int f = 0;
        bit_init(N + L);
        memset(bit_tree, 0, (bit_size + 1) * sizeof(long long));
        for (int b = 1; b <= N; b++) {
            bit_add(f + L, 1);
            if (mu[b] == sign)
                f += K;
            else if (mu[b] == -sign)
                f -= K - 1;
            ans -= bit_sum(N + L) - bit_sum(f + L);
        }
        bit_free();
    }

    free(mu);
    return ans;
}
"""
    tmpdir = tempfile.mkdtemp()
    c_path = os.path.join(tmpdir, "p464.c")
    so_path = os.path.join(tmpdir, "p464.so")
    with open(c_path, "w") as f:
        f.write(c_code)
    subprocess.run(
        ["gcc", "-O2", "-shared", "-fPIC", "-o", so_path, c_path],
        check=True,
        capture_output=True,
    )
    lib = ctypes.CDLL(so_path)
    lib.solve.restype = ctypes.c_longlong
    lib.solve.argtypes = [ctypes.c_int, ctypes.c_int, ctypes.c_int]

    result = lib.solve(N, K, L)

    os.unlink(c_path)
    os.unlink(so_path)
    os.rmdir(tmpdir)

    return result


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
