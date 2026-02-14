"""Project Euler Problem 391 - Hopping Game.

Compute sum of M(n)^3 for 1 <= n <= 1000 where M(n) is the largest first move
that allows the first player to force a win in the hopping game on set S.

S = {s_k : k >= 0} where s_k is the cumulative popcount from 0 to k.
The game starts at counter c=0; players add values in [1,n] to c, which must
land on elements of S. A player who cannot move loses.

Uses iterated function composition DP (embedded C for speed).
"""

from __future__ import annotations

import os
import subprocess
import tempfile

C_SOURCE = r"""
#include <stdio.h>
#include <stdint.h>

#define N 1010

static short F[2][N][N];

static long long compute(int n) {
    n += 1;
    short (*f)[N] = F[0], (*g)[N] = F[1];

    for (int s = 0; s <= n + 1 && s < N; ++s)
        for (int x = 0; x <= n && x < N; ++x)
            if (s + x >= n)
                f[s][x] = 0;
            else
                f[s][x] = s + x;

    for (int h = 1; h <= n; ++h) {
        for (int s = 0; s <= n - h; ++s) {
            for (int x = 0; x <= n && x < N; ++x) {
                int y = f[s + 1][x];
                g[s][x] = f[s][y];
            }
        }
        short (*tmp)[N] = f;
        f = g;
        g = tmp;
    }
    return f[0][0];
}

int main(void) {
    long long ans = 0;
    for (int i = 1; i <= 1000; ++i) {
        long long x = compute(i);
        ans += x * x * x;
    }
    printf("%lld\n", ans);
    return 0;
}
"""


def main() -> None:
    with tempfile.TemporaryDirectory() as tmpdir:
        c_path = os.path.join(tmpdir, "sol.c")
        bin_path = os.path.join(tmpdir, "sol")
        with open(c_path, "w") as f:
            f.write(C_SOURCE)
        subprocess.run(["gcc", "-O2", "-o", bin_path, c_path], check=True)
        result = subprocess.run(
            [bin_path], capture_output=True, text=True, check=True, timeout=280
        )
    print(result.stdout.strip())


if __name__ == "__main__":
    main()
