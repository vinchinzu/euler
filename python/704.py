"""Project Euler Problem 704: Factors of Two in Binomial Coefficients.

Let g(n, m) be the largest k such that 2^k divides nCr(n, m), and let
F(n) = max_m g(n, m). Find Σ_{n=1}^N F(n).

F(n) equals ⌊log₂ n⌋ if n is even, and equals F(⌊n/2⌋) if n is odd. This means
that to compute Σ_{n=1}^N F(n), we can first compute the sum of the terms for
even n; first we add 1 for all even terms starting from 2, then add 1 for all
even terms starting from 4, and so on. Finally we add the odd terms, which we
compute recursively as Σ_{n=1}^{⌊(N-1)/2⌋} F(n).
"""

from __future__ import annotations


def solve() -> int:
    """Solve Problem 704."""
    n = 10**16
    ans = 0

    # Process in reverse order: for each n from N down to 1
    current_n = n
    while current_n > 1:
        # For even numbers starting from 2, 4, 8, ...
        start = 2
        while start <= current_n:
            # Count even numbers from start to current_n
            count = (current_n - start) // 2 + 1
            ans += count
            start *= 2
        # Move to odd numbers: F(n) for odd n equals F(floor(n/2))
        current_n = (current_n - 1) // 2

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
