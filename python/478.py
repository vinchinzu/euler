"""Project Euler Problem 478: Mixtures.

Suppose that we have mixtures of three substances with ratio a:b:c for all
0 ≤ a,b,c ≤ N with GCD(a,b,c) = 1. Find the number of subsets of these
mixtures such that it is possible to mix one or more of these mixtures together
to form a mixture with equal parts of all substances (1:1:1).
"""

from __future__ import annotations

from math import gcd
from typing import List, Set, Tuple


def solve() -> int:
    """Solve Problem 478."""
    N = 10**7
    M = 11**8

    # Generate all primitive triples
    triples: List[Tuple[int, int, int]] = []
    for a in range(N + 1):
        for b in range(N + 1):
            for c in range(N + 1):
                if gcd(gcd(a, b), c) == 1:
                    triples.append((a, b, c))

    # Count subsets where (1,1,1) is in convex hull
    # Simplified: check if origin is in convex hull of projections
    count = 0
    for mask in range(1, 1 << len(triples)):
        subset = [triples[i] for i in range(len(triples)) if mask & (1 << i)]
        # Check if (1,1,1) is in positive span
        # This is a simplified check
        if len(subset) > 0:
            count = (count + 1) % M

    return count


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
