"""Project Euler Problem 505: Bidirectional Recurrence.

Define x(0) = 0, x(1) = 1, x(2k) = (3x(k) + 2x(⌊k/2⌋)) (mod M), and
x(2k+1) = (2x(k) + 3x(⌊k/2⌋)) (mod M). Then define y(k) = x(k) if k ≥ N
and y(k) = M - 1 - max(y(2k), y(2k+1)) otherwise. Find y(1).
"""

from __future__ import annotations

from typing import Dict


def solve() -> int:
    """Solve Problem 505."""
    N = 10**12
    M = 2**60
    K = M - 1

    def helper(
        k: int, prev_x: int, x: int, alpha: int, beta: int
    ) -> int:
        """Recursive helper with alpha-beta pruning."""
        if k >= N:
            return x

        new_x = (2 * prev_x + 3 * x) & K
        y = helper(2 * k, x, new_x, K - beta, K - alpha)

        if K - y <= alpha:
            return alpha

        new_x2 = (3 * prev_x + 2 * x) & K
        y2 = helper(2 * k + 1, x, new_x2, y, K - alpha)

        return K - max(y, y2)

    return helper(1, 0, 1, 0, K)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
