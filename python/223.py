"""Project Euler Problem 223: Almost right-angled triangles I.

A barely acute triangle has sides a≤b≤c with a²+b²=c²+1. Find the number of
barely acute triangles with perimeter at most N.
"""

from __future__ import annotations

from typing import List


def solve() -> int:
    """Solve Problem 223."""
    N = 25_000_000

    stack: List[int] = [1, 1, 1, 1, 2, 2]
    ans = 0

    while stack:
        c = stack.pop()
        b = stack.pop()
        a = stack.pop()

        if a + b + c <= N:
            # Generate new solutions
            stack.extend([a - 2 * b + 2 * c, 2 * a - b + 2 * c, 2 * a - 2 * b + 3 * c])

            if a != b:
                stack.extend([-a + 2 * b + 2 * c, -2 * a + b + 2 * c, -2 * a + 2 * b + 3 * c])

            stack.extend([a + 2 * b + 2 * c, 2 * a + b + 2 * c, 2 * a + 2 * b + 3 * c])
            ans += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
