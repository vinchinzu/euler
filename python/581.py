"""Project Euler Problem 581: Størmer numbers.

Find the sum of all n such that tr(n) is N-smooth, where tr(n) is the
triangular number n(n+1)/2.

We use Størmer's theorem. We solve 2^k-1 Pell equations for each product
of a subset of the primes p_i other than 2, generate up to (p_k+1)/2
solutions each, and check (x-1)/2 and (x+1)/2 for all solutions to
x²-2qy²=1 for N-smoothness.
"""

from __future__ import annotations

from math import isqrt
from typing import List, Set

from sympy import primerange


def is_square(n: int) -> bool:
    """Check if n is a perfect square."""
    if n < 0:
        return False
    root = isqrt(n)
    return root * root == n


class PellSolution:
    """Represents a solution to a Pell equation."""

    def __init__(self, x: int, y: int) -> None:
        """Initialize a Pell solution."""
        self.x = x
        self.y = y


def solve_pell_standard(d: int) -> List[PellSolution]:
    """Solve the standard Pell equation x² - d*y² = 1.

    Returns solutions until x exceeds 64-bit or we have enough solutions.
    """
    if is_square(d):
        return []

    # Find fundamental solution using continued fractions
    a0 = isqrt(d)
    m = 0
    den = 1
    a = a0

    h_prev, k_prev = 1, 0
    h, k = a0, 1

    solutions: List[PellSolution] = []
    max_iterations = 10000

    for _ in range(max_iterations):
        if h * h - d * k * k == 1:
            solutions.append(PellSolution(h, k))
            break

        m = den * a - m
        den = (d - m * m) // den
        a = (a0 + m) // den

        h_next = a * h + h_prev
        k_next = a * k + k_prev

        h_prev, k_prev = h, k
        h, k = h_next, k_next

    if not solutions:
        return []

    # Generate more solutions using recurrence
    x0, y0 = solutions[0].x, solutions[0].y
    x, y = x0, y0

    # Generate up to N/2 solutions or until x exceeds 64-bit
    max_solutions = 47 // 2 + 1
    while len(solutions) < max_solutions and x.bit_length() <= 64:
        if x != x0 or y != y0:  # Don't duplicate fundamental solution
            solutions.append(PellSolution(x, y))
        x_next = x0 * x + d * y0 * y
        y_next = x0 * y + y0 * x
        x, y = x_next, y_next

    return solutions


def is_smooth(n: int, primes: List[int]) -> bool:
    """Check if n is smooth with respect to the given primes."""
    if n <= 1:
        return n == 1
    for p in primes:
        while n % p == 0:
            n //= p
    return n == 1


def solve() -> int:
    """Solve Problem 581."""
    N = 47
    primes = list(primerange(2, N + 1))

    nums: Set[int] = set()

    # Iterate over all subsets of primes (excluding the empty set and {2})
    num_primes = len(primes)
    for subset in range(1 << num_primes):
        if subset == 1:  # Skip subset containing only 2
            continue

        # Compute q as product of primes in subset
        q = 1
        for i in range(num_primes):
            if subset & (1 << i):
                q *= primes[i]

        # Solve Pell equation x² - 2q*y² = 1
        d = 2 * q
        solutions = solve_pell_standard(d)

        # Check solutions: for x² - 2q*y² = 1, check (x-1)/2 and (x+1)/2
        # Note: x is odd (since x² ≡ 1 mod 2), so both are integers
        for sol in solutions:
            if sol.x.bit_length() > 64:
                break
            # Check (x-1)/2 (this is what x >> 1 gives for odd x)
            b = sol.x >> 1
            if b > 0 and is_smooth(b, primes) and is_smooth(b + 1, primes):
                nums.add(b)

    return sum(nums)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
