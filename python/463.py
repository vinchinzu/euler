"""Project Euler Problem 463: A weird recurrence relation.

Define f(n) by:
- f(1) = 1
- f(3) = 3
- f(2n) = n
- f(4n+1) = 2f(2n+1) - f(n)
- f(4n+3) = 3f(2n+1) - 2f(n)

Find Σ_{k=1}^N f(k).
"""

from __future__ import annotations

from typing import Dict


def solve() -> int:
    """Solve Problem 463."""
    N = 3**37
    M = 10**9
    cache: Dict[int, int] = {}
    odd_cache: Dict[int, int] = {}

    def sum_f(n: int) -> int:
        """Sum f(k) for k=1 to n."""
        if n in cache:
            return cache[n]
        sum_f_val = 0
        if n >= 1:
            sum_f_val += 1
        if n >= 2:
            sum_f_val += 1
        if n >= 3:
            sum_f_val += 3
        if n >= 4:
            sum_f_val += (
                sum_f(n // 4)
                + 2 * sum_odd_f((n - 1) // 4)
                - sum_f((n - 1) // 4)
                + sum_odd_f((n - 2) // 4)
                + 3 * sum_odd_f((n - 3) // 4)
                - 2 * sum_f((n - 3) // 4)
            )
        result = sum_f_val % M
        cache[n] = result
        return result

    def sum_odd_f(n: int) -> int:
        """Sum f(2k+1) for k=1 to n."""
        if n in odd_cache:
            return odd_cache[n]
        sum_odd_f_val = 0
        if n >= 1:
            sum_odd_f_val += 3
        if n >= 2:
            sum_odd_f_val += (
                2 * sum_odd_f(n // 2)
                - sum_f(n // 2)
                + 3 * sum_odd_f((n - 1) // 2)
                - 2 * sum_f((n - 1) // 2)
            )
        result = sum_odd_f_val % M
        odd_cache[n] = result
        return result

    return sum_f(N)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
