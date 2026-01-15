"""Project Euler Problem 554: Centaurs on a chessboard.

Let C(n) be the number of ways that n² centaurs (a piece that can move as a
king or knight) can be placed on a (2n)x(2n) chessboard such that no centaur
attacks another. Find Σ_{i=2}^N C(F_i), where F_i is the i'th Fibonacci number.

Firstly, note that each 2x2 block can have at most one centaur, and to place n²
centaurs we need exactly one centaur in each block. If a block has a centaur
in a particular corner, then all 2x2 blocks in the direction of that corner
must have the centaur in the same position. This means that a placement of
centaurs is equivalent to partitioning the grid into up to 4 regions, one region
per corner, such that no square can "fall" closer to the corner.

There are 4 ways to fill the board with only one region.

To fill two regions corresponding to two adjacent corners, we can only select
one of the n-1 straight lines between those corners. There are 4 different ways
to select the corners.

To fill two regions corresponding to two opposite corners, we can draw any path
between the other two corners as the boundary of the two regions. There are
nCr(2n, n) paths, though we have to subtract the 2 that result in only one
region. There are 2 ways to select the corners.

To fill three regions, we must make a rectangular region for the "middle" corner
(and it cannot extend all the way to either opposing edge), and then draw any
path from the open corner of that rectangle to the opposite corner. The number
of paths to any point on the grid is an entry of Pascal's Triangle, and the
sum of the number of paths over all interior points can be summed up to be
nCr(2n, n) - 2n. There are 4 ways to select the corners.

Finally, to fill four regions, we choose a rectangular region for two opposite
corners (again, neither can extend all the way to an opposing edge) and draw
any path between the two open corners. Each sum is nCr(a+b, a) - 1, so the
sum over all sums is nCr(2n, n) - 2n - (n-1)². We double this for selecting
the other two opposite corners. However, we've double-counted the configurations
where all four regions are rectangles meeting at a corner; there are (n-1)² of
these.
"""

from __future__ import annotations

from typing import List


class Zp:
    """Modular arithmetic helper class."""

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

    def factorial(self, n: int) -> int:
        """Return n! mod mod."""
        if n > self.max_n:
            raise ValueError(f"n={n} exceeds max_n={self.max_n}")
        return self._factorials[n]

    def inv_factorial(self, n: int) -> int:
        """Return 1/(n!) mod mod."""
        if n > self.max_n:
            raise ValueError(f"n={n} exceeds max_n={self.max_n}")
        return self._inv_factorials[n]

    def nCr(self, n: int, r: int) -> int:
        """Return C(n, r) mod mod."""
        if r < 0 or r > n:
            return 0
        if n > self.max_n:
            # Compute directly for large n
            result = 1
            for i in range(r):
                result = (result * (n - i)) % self.mod
            return (result * pow(self.factorial(r), self.mod - 2, self.mod)) % self.mod
        return (
            self._factorials[n]
            * self._inv_factorials[r]
            * self._inv_factorials[n - r]
        ) % self.mod


def fibonacci(n: int) -> int:
    """Compute the n-th Fibonacci number."""
    if n <= 1:
        return n
    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b


def solve() -> int:
    """Solve Problem 554."""
    N = 90
    M = 10**8 + 7

    zp = Zp(2 * N, M)
    ans = 0

    def C(n: int) -> int:
        """Compute C(n) mod M."""
        n_mod = n % M
        nCr_2n_n = zp.nCr(2 * n, n)
        term1 = 4
        term2 = ((n_mod - 1) * 4) % M
        term3 = ((nCr_2n_n - 2) * 2) % M
        term4 = ((nCr_2n_n - (2 * n_mod) % M) * 4) % M
        sq_n_minus_1 = ((n_mod - 1) ** 2) % M
        term5 = ((nCr_2n_n - (2 * n_mod) % M - sq_n_minus_1) * 2) % M
        term6 = (-sq_n_minus_1) % M
        return (term1 + term2 + term3 + term4 + term5 + term6) % M

    for i in range(2, N + 1):
        fib = fibonacci(i)
        ans = (ans + C(fib)) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
