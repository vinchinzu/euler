"""Project Euler Problem 793: Median of Products.

Find the median of the products S_i * S_j for 0 <= i < j < N.

Embedded C for speed: binary search with two-pointer counting.
"""

from __future__ import annotations

import os
import subprocess
import tempfile

C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 1000003

static long long S[N];

int cmp_ll(const void *a, const void *b) {
    long long x = *(const long long *)a;
    long long y = *(const long long *)b;
    return (x > y) - (x < y);
}

int main(void) {
    /* Generate BBS sequence */
    S[0] = 290797;
    for (int i = 1; i < N; i++) {
        S[i] = (S[i-1] * S[i-1]) % 50515093LL;
    }

    /* Sort */
    qsort(S, N, sizeof(long long), cmp_ll);

    long long low = 0;
    long long high = S[N-1] * S[N-1];
    long long target = (long long)(N - 1) * (long long)N / 2;  /* tr(N-1) */

    while (low + 1 < high) {
        long long mid = low + (high - low) / 2;
        long long rank = 0;
        int row_count = N - 1;

        for (int i = 0; i < N; i++) {
            long long s = S[i];
            while (row_count >= 0 && s * S[row_count] >= mid) {
                row_count--;
            }
            /*
             * row_count + 1 = number of j where s * S[j] < mid
             * We count ordered pairs (i,j) with i != j, so subtract 1
             * if s*s < mid (the diagonal i==j was counted).
             */
            rank += row_count + (s * s < mid ? 0 : 1);
        }

        if (rank > target) {
            high = mid;
        } else {
            low = mid;
        }
    }

    printf("%lld\n", low);
    return 0;
}
"""


def solve() -> int:
    """Compile and run C solution."""
    tmp_dir = tempfile.mkdtemp()
    src = os.path.join(tmp_dir, "p793.c")
    exe = os.path.join(tmp_dir, "p793")

    try:
        with open(src, "w") as f:
            f.write(C_CODE)

        # Compile with optimisation
        subprocess.run(
            ["gcc", "-O2", "-o", exe, src, "-lm"],
            check=True,
            timeout=30,
        )

        # Run with generous timeout
        result = subprocess.run(
            [exe],
            capture_output=True,
            text=True,
            check=True,
            timeout=280,
        )

        return int(result.stdout.strip())
    finally:
        for f in [src, exe]:
            if os.path.exists(f):
                os.remove(f)
        os.rmdir(tmp_dir)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
