"""Project Euler Problem 598: Split Divisibilities.

Find the number of pairs of integers a≤b such that a*b = N! and a and b
have the same number of divisors.

First factor N! = prod_i (p_i)^(e_i). The prime factorization of a must
have exponents 0 ≤ f_i ≤ e_i such that the number of divisors,
prod_i (f_i + 1), is equal to the number of divisors of b,
prod_i (e_i - f_i + 1).
"""

from __future__ import annotations

from collections import defaultdict
from itertools import product
from math import gcd
from typing import Dict, List, Tuple

from sympy import primerange


class LPoint:
    """Represents a rational point (x, y) in lowest terms."""

    def __init__(self, x: int, y: int) -> None:
        """Initialize point."""
        g = gcd(x, y)
        self.x = x // g
        self.y = y // g

    def __hash__(self) -> int:
        """Hash for use in dictionary."""
        return hash((self.x, self.y))

    def __eq__(self, other: object) -> bool:
        """Equality comparison."""
        if not isinstance(other, LPoint):
            return False
        return self.x == other.x and self.y == other.y


def solve() -> int:
    """Solve Problem 598."""
    N = 100

    # Compute exponents for N!
    es: List[int] = []
    for p in primerange(2, N + 1):
        e = 0
        n = N
        while n > 0:
            n //= p
            e += n
        es.append(e)

    # Process from largest primes first
    map_dict: Dict[LPoint, int] = {LPoint(1, 1): 1}
    index = len(es) - 1

    while True:
        e = es[index]
        new_map: Dict[LPoint, int] = defaultdict(int)

        for ratio, count in map_dict.items():
            for f in range(e + 1):
                new_x = ratio.x * (f + 1)
                new_y = ratio.y * (e - f + 1)
                g = gcd(new_x, new_y)
                product_point = LPoint(new_x // g, new_y // g)
                new_map[product_point] += count

        map_dict = new_map

        # Check if we should switch to brute force
        prod = 1
        for i in range(index):
            prod *= es[i]
        if prod < len(map_dict):
            break

        index -= 1

    final_map = map_dict

    # Brute force for remaining primes
    axes: List[List[int]] = []
    for i in range(index):
        axes.append(list(range(es[i] + 1)))

    ans = 0
    for fs in product(*axes):
        x = 1
        y = 1
        for i in range(len(fs)):
            x *= fs[i] + 1
            y *= es[i] - fs[i] + 1
        g = gcd(x, y)
        target = LPoint(y // g, x // g)
        ans += final_map.get(target, 0)

    ans //= 2
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
