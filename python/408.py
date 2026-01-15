"""Project Euler Problem 408: Admissible paths through a square grid.

Find the number of paths from (0, 0) to (n, n) that are non-decreasing in x
and y, and do not pass through any point (x, y) such that x, y, and x+y are
all perfect squares.

First we can compute all such (x, y) by iterating over Pythagorean triples.
Then, we use Dynamic Programming / Inclusion Exclusion to compute the number
of paths from (0, 0) to each inadmissible point. The number of paths from
(0, 0) to a point p is equal to nCr(p.x + p.y, p.x), minus the number of
paths whose first inadmissible point is q, for each point q inside the
rectangle with corners at (0, 0) and p. This is just the number of
admissible paths to q, times the number of all paths from q to p.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import gcd, isqrt
from typing import Dict, List


@dataclass(frozen=True, order=True)
class IPoint:
    """Integer point with x and y coordinates."""

    x: int
    y: int


def sq(n: int) -> int:
    """Return n squared."""
    return n * n


def is_square(n: int) -> bool:
    """Check if n is a perfect square."""
    root = isqrt(n)
    return root * root == n


def pythagorean_triples(limit: int) -> List[tuple[int, int, int]]:
    """Generate all Pythagorean triples with c <= limit."""
    triples: List[tuple[int, int, int]] = []
    m_limit = isqrt(limit)

    for m in range(2, m_limit + 1):
        for n in range(1, m):
            if (m + n) % 2 == 1 and gcd(m, n) == 1:
                a = m * m - n * n
                b = 2 * m * n
                c = m * m + n * n

                if c > limit:
                    break

                # Generate all multiples
                k = 1
                while k * c <= limit:
                    triples.append((k * a, k * b, k * c))
                    triples.append((k * b, k * a, k * c))
                    k += 1

    return triples


class Zp:
    """Modular arithmetic helper for binomial coefficients."""

    def __init__(self, max_n: int, mod: int) -> None:
        """Initialize with maximum n and modulus."""
        self.mod = mod
        self.max_n = max_n
        self._factorials: List[int] = []
        self._inv_factorials: List[int] = []
        self._precompute()

    def _precompute(self) -> None:
        """Precompute factorials and inverse factorials."""
        self._factorials = [1] * (self.max_n + 1)
        for i in range(1, self.max_n + 1):
            self._factorials[i] = (self._factorials[i - 1] * i) % self.mod

        self._inv_factorials = [1] * (self.max_n + 1)
        self._inv_factorials[self.max_n] = pow(
            self._factorials[self.max_n], self.mod - 2, self.mod
        )
        for i in range(self.max_n - 1, -1, -1):
            self._inv_factorials[i] = (
                self._inv_factorials[i + 1] * (i + 1)
            ) % self.mod

    def nCr(self, n: int, r: int) -> int:
        """Return C(n, r) mod mod."""
        if r < 0 or r > n:
            return 0
        if n > self.max_n:
            # Compute directly for large n
            result = 1
            for i in range(r):
                result = (result * (n - i)) % self.mod
            return (
                result * pow(self.factorial(r), self.mod - 2, self.mod)
            ) % self.mod
        return (
            self._factorials[n]
            * self._inv_factorials[r]
            * self._inv_factorials[n - r]
        ) % self.mod

    def factorial(self, n: int) -> int:
        """Return n! mod mod."""
        if n > self.max_n:
            raise ValueError(f"n={n} exceeds max_n={self.max_n}")
        return self._factorials[n]


def solve() -> int:
    """Solve Problem 408."""
    N = 10**7
    M = 10**9 + 7

    # Find all inadmissible points
    last = IPoint(N, N)
    points: List[IPoint] = [last]

    # Generate Pythagorean triples and find points where x, y, x+y are squares
    triples = pythagorean_triples(4 * isqrt(N))
    for a, b, _c in triples:
        a_sq = sq(a)
        b_sq = sq(b)
        if a_sq <= N and b_sq <= N:
            points.append(IPoint(a_sq, b_sq))
            points.append(IPoint(b_sq, a_sq))

    # Sort points by x, then y
    points.sort()

    # Compute paths using inclusion-exclusion
    zp = Zp(2 * N, M)
    all_num_paths_from: Dict[IPoint, int] = {}

    for p in points:
        num_paths_from = zp.nCr(p.x + p.y, p.x)
        for q in points:
            if q.x <= p.x and q.y <= p.y and p != q:
                num_paths_from = (
                    num_paths_from
                    - all_num_paths_from[q]
                    * zp.nCr(p.x - q.x + p.y - q.y, p.x - q.x)
                ) % M
        all_num_paths_from[p] = num_paths_from % M

    return all_num_paths_from[last]


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
