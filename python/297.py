"""Project Euler Problem 297: Zeckendorf Representation.

Let z(n) be the number of Fibonacci numbers, no two consecutive, that
add up to n. Find sum_{0<n<N} z(n).
"""

from __future__ import annotations

from typing import Dict


def fibonacci_sequence(max_val: int) -> list[int]:
    """Generate Fibonacci sequence up to max_val."""
    fibs = [1, 2]
    while fibs[-1] < max_val:
        fibs.append(fibs[-1] + fibs[-2])
    return fibs


def solve() -> int:
    """Solve Problem 297."""
    N = 10**17
    fibs = fibonacci_sequence(N)
    cache: Dict[int, int] = {}

    def Z(n: int) -> int:
        """Compute Z(n) with memoization."""
        if n in cache:
            return cache[n]
        if n == 1:
            result = 0
        else:
            # Find largest Fibonacci number < n
            a = 1
            for f in fibs:
                if f < n:
                    a = f
                else:
                    break
            result = Z(a) + n - a + Z(n - a)
        cache[n] = result
        return result

    return Z(N)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
