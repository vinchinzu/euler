"""Project Euler Problem 588: Quintinomial coefficients.

Let Q(k) be the number of odd coefficients in (x⁴+x³+x²+x+1)^k.
Find sum_{k=1}^N Q(10^k).

Define the Quintinomial Sierpinski triangle similarly to the binary
Sierpinski triangle, where the element in row n and entry k is 1 if
the coefficient of x^k (x⁴+x³+x²+x+1)^n is odd and 0 otherwise.
The Quintinomial triangle is a fractal.
"""

from __future__ import annotations

from typing import List, Tuple


def helper(n: int, size: int, K: int = 5) -> List[int]:
    """Recursive helper to compute counts for Quintinomial triangle."""
    counts: List[int]
    small_sections = [0] * (2 * K)

    if n == 0:
        counts = [0, 1]
        small_sections[0] = 1
    elif n < size // 2:
        counts = helper(n, size // 2, K)
        for i in range(K - 1):
            small_sections[i] = 1 << i
    else:
        counts = helper(n - size // 2, size // 2, K)
        for i in range(K - 1):
            small_sections[i] = (1 << (i + 1)) - 1
            small_sections[i + K - 1] = (1 << (K - 1)) - (1 << i)

    new_counts = [0] * (1 << (K - 1))
    for subset in range(len(new_counts)):
        left_half = 0
        right_half = 0
        for i in range(K - 1):
            if subset & (1 << i):
                left_half ^= small_sections[2 * i]
                right_half ^= small_sections[2 * i + 1]
        new_counts[subset] = counts[left_half] + counts[right_half]

    return new_counts


def solve() -> int:
    """Solve Problem 588."""
    N = 18
    ans = 0

    for k in range(1, N + 1):
        n = 10**k
        size = 1 << 62  # Large enough size
        result = helper(n, size)
        ans += result[1]  # Count for section A

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
