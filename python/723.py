"""Project Euler Problem 723: Pythagorean Quadrilaterals.

Let f(n) be the number of Pythagorean quadrilaterals, which are quadrilaterals
with side lengths (in order) a,b,c,d satisfy a²+b²+c²+d²=8r², whose vertices
are lattice points on the circle of radius √n. Find Σ_{d|N} f(d).

A quadrilateral is Pythagorean iff either its two diagonals are perpendicular,
or one of its diagonals is the diameter. To count those in the first category
that don't belong to the second, we note that any such quadrilateral either has
diagonals that are parallel to the axes or exactly 45º from the axes, or all
vertices can be divided by some complex number z that is a factor of √n in order
to get such a quadrilateral.

We enumerate such z: since each prime factor p ≡ 1 (mod 4), there are two
complex factors a±bi. So for the prime factor p^e, we can choose either 1, or
one of the two factors (a±bi)^{e'} for e'≤e. The number of lattice points on
the circle with the remaining factor, Π_d (a±bi)^d, is 4k = 4 Π_d d+1. We can
count 4 nCr(k,2) pairs of intersecting perpendicular line segments 45º from the
axes for such a set of lattice points. For line segments parallel to the axes,
we need to decrement k if there's a point on the x-axis, i.e. all the d values
are even.

Finally, for the second category (one of the diagonals is the diameter), we can
again let the number of lattice points on the original circle be 4k = 4 Π_e e+1.
Then we need to choose one of the 2k diameters, then one of the 2k-1 points on
one side of the diameter, and one of the 2k-1 points on the other side. We then
remove the nCr(2k,2) ways to select two diameters, since those are double counted.
"""

from __future__ import annotations

from itertools import product
from typing import Dict, List


def lprime_factor(n: int) -> Dict[int, int]:
    """Return prime factorization of n as a dictionary."""
    factors: Dict[int, int] = {}
    d = 2
    while d * d <= n:
        while n % d == 0:
            factors[d] = factors.get(d, 0) + 1
            n //= d
        d += 1
    if n > 1:
        factors[n] = factors.get(n, 0) + 1
    return factors


def all_divisors(n: int, prime_factors: List[int]) -> List[int]:
    """Return all divisors of n given prime factors."""
    divisors = [1]
    temp = n
    for p in prime_factors:
        if temp % p == 0:
            size = len(divisors)
            power = 1
            while temp % p == 0:
                temp //= p
                power *= p
                for i in range(size):
                    divisors.append(divisors[i] * power)
    if temp > 1:
        size = len(divisors)
        for i in range(size):
            divisors.append(divisors[i] * temp)
    return divisors


def cb(n: int) -> int:
    """Cube."""
    return n * n * n


def sq(n: int) -> int:
    """Square."""
    return n * n


def nCr(n: int, r: int) -> int:
    """Binomial coefficient."""
    if r < 0 or r > n:
        return 0
    if r == 0 or r == n:
        return 1
    r = min(r, n - r)
    result = 1
    for i in range(r):
        result = result * (n - i) // (i + 1)
    return result


def num_factors(exponents: List[int]) -> int:
    """Compute number of factors from exponents."""
    result = 1
    for e in exponents:
        result *= e + 1
    return result


def solve() -> int:
    """Solve Problem 723."""
    N = 5**6 * cb(13) * sq(17) * 29 * 37 * 41 * 53 * 61

    def f(n: int) -> int:
        """Compute f(n) for Pythagorean quadrilaterals."""
        factors = lprime_factor(n)
        exponents = list(factors.values())

        # Generate all combinations of d values
        axes = [list(range(e + 1)) for e in exponents]
        result = 0

        for ds in product(*axes):
            k = num_factors(list(ds))
            mult = 1
            for i, d in enumerate(ds):
                if d < exponents[i]:
                    mult *= 2

            # Count pairs 45º from axes
            result += 4 * mult * nCr(k, 2)

            # Count pairs parallel to axes
            k_parallel = k
            if all(d % 2 == 0 for d in ds):
                k_parallel -= 1
            result += 4 * mult * nCr(k_parallel, 2)

        # Add diameter-based quadrilaterals
        k_total = num_factors(exponents)
        result += 2 * k_total * sq(2 * k_total - 1) - nCr(2 * k_total, 2)

        return result

    # Get all divisors of N
    prime_factors_list = list(lprime_factor(N).keys())
    divisors = all_divisors(N, prime_factors_list)

    ans = 0
    for d in divisors:
        ans += f(d)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
