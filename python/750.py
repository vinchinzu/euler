"""Project Euler Problem 750: Optimal Card Stacking.

Cards at position n have value 3^n mod (N+1). Find minimal total drag distance
to combine all cards into a single sequential stack.

Uses interval DP in C for performance: dp[s][e] = min cost to merge cards s..e
into one stack. For each interval, try all split points.
"""

from __future__ import annotations

import subprocess
import tempfile
import os


def solve(n: int = 976) -> int:
    """Solve Problem 750 using compiled C for O(N^3) DP."""
    c_code = f"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N {n}

static long long dp[N][N+1];
static int pos[N];

int main(void) {{
    long long p = 1;
    int i, s, length, mid, end;
    for (i = 0; i < N; i++) {{
        p = (p * 3) % (N + 1);
        pos[(int)p - 1] = i;
    }}

    memset(dp, 0, sizeof(dp));

    for (length = 2; length <= N; length++) {{
        for (s = 0; s <= N - length; s++) {{
            end = s + length;
            long long best = -1;
            for (mid = s + 1; mid < end; mid++) {{
                long long cost = dp[s][mid] + dp[mid][end]
                    + abs(pos[mid - 1] - pos[end - 1]);
                if (best < 0 || cost < best)
                    best = cost;
            }}
            dp[s][end] = best;
        }}
    }}

    printf("%lld\\n", dp[0][N]);
    return 0;
}}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(c_code)
        c_path = f.name

    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, check=True)
        return int(result.stdout.strip())
    finally:
        os.unlink(c_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
