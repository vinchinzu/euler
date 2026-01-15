"""Project Euler Problem 469: Empty chairs.

Knights sit at a round table of N chairs one by one, always choosing at
random an empty chair not adjacent to any other knights. Find the expected
fraction of the number of empty chairs when no more knights can sit.
"""

from __future__ import annotations

from typing import List


def solve() -> float:
    """Solve Problem 469."""
    Es: List[float] = []
    ans = float("nan")

    for i in range(1000):  # Reasonable limit
        E = 0.0
        if i > 1:
            for j in range(1, i - 1):
                E += (1 + Es[j] + Es[i - j - 1]) / (i - 2)
        Es.append(E)
        cand = (i - E) / (i + 1) if i > 0 else 0.0
        if cand == ans:
            break
        ans = cand

    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.14f}")
    return result


if __name__ == "__main__":
    main()
