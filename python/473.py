"""Project Euler Problem 473: Phigital palindromes.

Find the sum of all positive integers up to N whose unique representation in
base φ with no two adjacent ones is palindromic.
"""

from __future__ import annotations

import math


def solve() -> int:
    """Solve Problem 473."""
    N = 10**10
    PHI = (1 + math.sqrt(5)) / 2
    ans = 1  # 1 is trivially palindromic

    def helper(n: int, min_e: int) -> None:
        """Recursive helper."""
        nonlocal ans
        if n > N:
            return
        ans += n
        e = min_e
        while True:
            new_n = n + int(
                round(
                    PHI**e
                    + PHI ** (e + 3)
                    + PHI ** (-e - 1)
                    + PHI ** (-e - 4)
                )
            )
            if new_n > N:
                break
            helper(new_n, e + 6)
            e += 2

    helper(0, 2)
    helper(2, 4)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
