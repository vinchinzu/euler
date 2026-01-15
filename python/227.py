"""Project Euler Problem 227: The Chase.

N players sit in a circle, and two diametrically opposite players each start
with a K-sided die. Find the expected number of rounds until a single player
has possession of both dice.
"""

from __future__ import annotations

from typing import List


def fsq(n: float) -> float:
    """Return n squared."""
    return n * n


def solve() -> float:
    """Solve Problem 227."""
    N = 100
    K = 6
    L = 100000

    table: List[float] = [0.0] * (N // 2 + 1)
    table[N // 2] = 1.0
    ans = 0.0

    for num_rounds in range(L):
        ans += num_rounds * table[0]
        new_table: List[float] = [0.0] * (N // 2 + 1)

        for dist in range(1, N // 2 + 1):
            # Both dice move towards each other
            if dist == 1:
                new_table[dist] += table[dist] / fsq(K)
            else:
                new_table[dist - 2] += table[dist] / fsq(K)

            # One die moves towards the other
            new_table[dist - 1] += table[dist] * 2 * (K - 2) / fsq(K)

            # Distance stays the same
            new_table[dist] += table[dist] * (2 + fsq(K - 2)) / fsq(K)

            # One die moves away
            if dist == N // 2:
                new_table[dist - 1] += table[dist] * 2 * (K - 2) / fsq(K)
            else:
                new_table[dist + 1] += table[dist] * 2 * (K - 2) / fsq(K)

            # Both dice move away
            if dist == N // 2:
                new_table[dist - 2] += table[dist] / fsq(K)
            elif dist == N // 2 - 1:
                new_table[dist] += table[dist] / fsq(K)
            else:
                new_table[dist + 2] += table[dist] / fsq(K)

        table = new_table

    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.6f}")
    return result


if __name__ == "__main__":
    main()
