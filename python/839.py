"""Project Euler Problem 839: Beans in Bowls — embedded C for speedup."""

from __future__ import annotations

import os
import subprocess
import sys
import tempfile

C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>

#define NMAX 10000000

/*
 * Block: a merged group of bowls.
 * value = total beans, length = number of bowls in the block.
 */
typedef struct {
    long long value;
    int length;
} Block;

static long long S[NMAX];      /* BBS sequence / reused for diffs */
static Block blocks[NMAX];     /* stack of blocks */
static long long T[NMAX];      /* final state */

int main(void) {
    int N = NMAX;
    long long seed = 290797LL;
    long long m = 50515093LL;

    /* Generate BBS sequence */
    S[0] = seed;
    for (int i = 1; i < N; i++) {
        S[i] = (S[i-1] * S[i-1]) % m;
    }

    /* Merge blocks */
    int nb = 0;  /* number of blocks on stack */
    for (int i = 0; i < N; i++) {
        blocks[nb].value = S[i];
        blocks[nb].length = 1;
        nb++;
        while (nb >= 2) {
            Block *b1 = &blocks[nb - 2];
            Block *b2 = &blocks[nb - 1];
            /* avg1 = ceil(b1->value / b1->length) = (val + len - 1) / len */
            long long avg1 = (b1->value + b1->length - 1) / b1->length;
            /* avg2 = floor(b2->value / b2->length) */
            long long avg2 = b2->value / b2->length;
            if (avg1 <= avg2) break;
            /* Merge */
            b1->value += b2->value;
            b1->length += b2->length;
            nb--;
        }
    }

    /* Compute final state T */
    int idx = 0;
    for (int b = 0; b < nb; b++) {
        long long v = blocks[b].value;
        int len = blocks[b].length;
        for (int i = 0; i < len; i++) {
            T[idx++] = (v + i) / len;
        }
    }

    /* Count steps: accumulate diffs left to right */
    long long ans = 0;
    for (int i = 0; i < N - 1; i++) {
        long long diff = S[i] - T[i];
        S[i + 1] += diff;
        ans += diff;
    }

    printf("%lld\n", ans);
    return 0;
}
"""


def main() -> int:
    """Main entry point."""
    with tempfile.NamedTemporaryFile(suffix=".c", mode="w", delete=False) as f:
        f.write(C_CODE)
        c_path = f.name
    bin_path = c_path.replace(".c", "")
    try:
        subprocess.run(
            ["gcc", "-O2", "-o", bin_path, c_path, "-lm"],
            check=True, capture_output=True,
        )
        result = subprocess.run(
            [bin_path],
            capture_output=True, text=True, timeout=280,
        )
        answer = int(result.stdout.strip())
        print(answer)
        return answer
    finally:
        for p in [c_path, bin_path]:
            if os.path.exists(p):
                os.unlink(p)


if __name__ == "__main__":
    main()
