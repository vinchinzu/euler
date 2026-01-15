"""Project Euler Problem 789: Minimal Pairing Product.

Let ((a1, b1), (a2, b2), ...) be the pairing of the numbers from 1 to N-1
such that the sum Σ_i (a_i * b_i (mod N)) is minimal. Find Π_i (a_i * b_i
(mod N)) for that minimal pairing.

The product of the (a_i * b_i)s is always congruent to (N-1)! ≡ -1 (mod N), so
we need to find an integer congruent to -1 (mod N) where the sum of (p-1) over
all its factors p is minimal. We can do this by bidirectional search, finding
numbers which are the product of factors p for small Σ p-1, until we find two
of them with products that multiply to -1 (mod N), and take the one with the
smallest sum.
"""

from __future__ import annotations

from typing import Dict, List

from sympy import primerange


def mod_inv(a: int, m: int) -> int:
    """Modular inverse."""
    return pow(a, m - 2, m)


def solve() -> int:
    """Solve Problem 789."""
    N = 2_000_000_011
    primes = list(primerange(2, 100))

    values: Dict[int, int] = {}

    def helper(min_index: int, cost: int, prod: int, cost_bound: int) -> None:
        """Recursive helper to find products with bounded cost."""
        if prod in values:
            values[prod] = min(values[prod], cost)
        else:
            values[prod] = cost

        for index in range(min_index, len(primes)):
            p = primes[index]
            if cost + p - 1 > cost_bound or prod * p > 10**10:
                return
            helper(index, cost + p - 1, prod * p, cost_bound)

    ans = None
    cost_bound = 1
    while ans is None:
        values.clear()
        helper(0, 0, 1, cost_bound)

        min_cost = float("inf")
        for prod in values:
            inv = mod_inv(-prod % N, N)
            if inv in values:
                cost = values[prod] + values[inv]
                if cost < min_cost:
                    min_cost = cost
                    ans = (prod * inv) % N

        cost_bound *= 2

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
