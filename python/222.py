"""Project Euler Problem 222: Sphere Packing.

Find the length in μm of the shortest pipe with internal radius N mm that can
fully contain K balls of length N-K+1 mm, N-K+2 mm, ... N mm.
"""

from __future__ import annotations

import math
from typing import List


def fsq(n: float) -> float:
    """Return n squared."""
    return n * n


def solve() -> int:
    """Solve Problem 222."""
    N = 50
    K = 21

    order: List[int] = []
    for i in range(N - K + 1, N + 1):
        if i % 2 == 0:
            order.insert(0, i)
        else:
            order.append(i)

    length = float(order[0] + order[-1])
    for i in range(1, len(order)):
        sum_radii = order[i - 1] + order[i]
        length += math.sqrt(fsq(sum_radii) - fsq(2 * N - sum_radii))

    return round(1000 * length)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
