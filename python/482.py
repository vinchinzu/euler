"""Project Euler Problem 482: Arithmetic derivative.

Let I be the incenter of a triangle and p be its perimeter. Find Σ p + |IA| +
|IB| + |IC| for all integer triangles where p ≤ N and the segments IA, IB, IC
have integral length.
"""

from __future__ import annotations

from math import gcd, isqrt
from typing import List, Set, Tuple


def solve() -> int:
    """Solve Problem 482."""
    N = 10**7
    L = int(N / isqrt(108))

    # Generate primitive Pythagorean triples
    triples: List[Tuple[int, int, int]] = []
    for m in range(2, isqrt(L) + 1):
        for n in range(1, m):
            if (m + n) % 2 == 1 and gcd(m, n) == 1:
                a = m * m - n * n
                b = 2 * m * n
                c = m * m + n * n
                if c <= L:
                    triples.append((a, b, c))

    ans = 0
    seen: Set[Tuple[int, int, int, int]] = set()

    # Generate triangles from pairs of right triangles
    for i, (a1, b1, c1) in enumerate(triples):
        for j, (a2, b2, c2) in enumerate(triples):
            # Scale so first legs match
            scale1 = a2
            scale2 = a1
            x = a1 * scale1
            y = b1 * scale1
            r_sq = x * y / (x + y)  # Simplified
            if r_sq > 0 and int(r_sq) == r_sq:
                r = int(isqrt(int(r_sq)))
                z = int(r_sq * (x + y) / (x * y - r_sq)) if (x * y - r_sq) > 0 else 0
                if z > 0:
                    p = 2 * (x + y + z)
                    if p <= N:
                        key = (x, y, z, r)
                        if key not in seen:
                            seen.add(key)
                            ans += p + 3 * r  # Simplified: |IA| = |IB| = |IC| = r

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
