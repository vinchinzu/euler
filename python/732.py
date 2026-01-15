"""Project Euler Problem 732: Standing on the Shoulders of Trolls.

Given trolls each with distance h from feet to shoulders, length of arms l,
and IQ q, find the maximum IQ of a subset of trolls that can escape a hole
D = (1/√2) Σ h deep, assuming trolls can stand on each other's shoulders and
a troll whose arm can reach the surface can escape.

For each troll, we try letting that troll be the last one to escape - that
means we find the minimum IQ of trolls whose distances h add up to D-h-l for
the given troll, using standard dynamic programming similar to Knapsack. All
other trolls can stand on this last troll to escape.

For efficiency, instead of computing a DP for each troll, we compute a DP for
each "prefix" of trolls and for each "suffix" of trolls. These intermediate
results are already available in the standard DP algorithm. For each troll t,
we can just look at the DP for the prefix of trolls before t, and the suffix
of trolls after t, to find the optimum combination of trolls in the prefix
with trolls in the suffix with the minimum IQ.
"""

from __future__ import annotations

import math
from dataclasses import dataclass
from typing import List


@dataclass
class Troll:
    """Troll with height, arm length, and IQ."""

    h: int
    l: int
    q: int


def imod(a: int, m: int) -> int:
    """Integer modulo (handles negative)."""
    return ((a % m) + m) % m


def generate_trolls(n: int) -> List[Troll]:
    """Generate trolls using the given sequence."""
    M = 10**9 + 7
    trolls: List[Troll] = []
    r = 1
    for _ in range(n):
        h_val = imod(r, 101) + 50
        r = (r * 5) % M
        l_val = imod(r, 101) + 50
        r = (r * 5) % M
        q_val = imod(r, 101) + 50
        r = (r * 5) % M
        trolls.append(Troll(h_val, l_val, q_val))
    return trolls


def solve() -> int:
    """Solve Problem 732."""
    N = 1000
    trolls = generate_trolls(N)

    # Compute D
    total_h = sum(t.h for t in trolls)
    D = int(math.ceil(total_h / math.sqrt(2)))

    total_iq = sum(t.q for t in trolls)

    # DP arrays: left[i][j] = min IQ to reach distance j using first i trolls
    # right[i][j] = min IQ to reach distance j using last i trolls
    INF = 10**9
    left = [[INF] * (D + 1) for _ in range(N)]
    right = [[INF] * (D + 1) for _ in range(N)]

    # Initialize: 0 distance requires 0 IQ
    for i in range(N):
        left[i][0] = 0
        right[i][0] = 0

    # Fill left DP
    for i in range(1, N):
        troll = trolls[i - 1]
        for j in range(D, 0, -1):
            left[i][j] = min(left[i - 1][j], left[i][j + 1] if j < D else INF)
            if j >= troll.h:
                left[i][j] = min(
                    left[i][j], left[i - 1][j - troll.h] + troll.q
                )

    # Fill right DP
    for i in range(1, N):
        troll = trolls[N - i]
        for j in range(D, 0, -1):
            right[i][j] = min(
                right[i - 1][j], right[i][j + 1] if j < D else INF
            )
            if j >= troll.h:
                right[i][j] = min(
                    right[i][j], right[i - 1][j - troll.h] + troll.q
                )

    # Find maximum IQ
    ans = 0
    for i in range(N):
        troll = trolls[i]
        dist = D - troll.h - troll.l
        if dist < 0:
            continue
        for j in range(dist + 1):
            iq_used = left[i][j] + right[N - 1 - i][dist - j]
            if iq_used < INF:
                ans = max(ans, total_iq - iq_used)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
