"""Project Euler Problem 412: Young Tableaux.

Let L(m, n) be an m x m grid with the top right n x n grid removed. Find the
number of ways to number each cell of L(N, K) with the consecutive integers
1, 2, ... N²-K² such that every cell is smaller than the numbers below it and
to the left of it.

L(m, n) is a Young Tableaux, and the number of labellings can be computed using
the Hook formula:

num labellings = (# cells)! / prod_{square} (1 + number of squares above and
to the right).

To compute the numerator efficiently, we use Wilson's Theorem. Let L = M -
(N²-K²). Then (M-1)! ≡ -1 (mod M), so

(N²-K²)! ≡ (M-1)! ( (N²-K²+1)(N²-K²+2)...(M-1) )⁻¹
         ≡ (-1) ( (-1)(-2)...(-(L-1)) )⁻¹
         ≡ (-1)^L (L-1)!⁻¹.

We compute the denominator efficiently by noting that hook lengths of squares
in a row are consecutive integers, which is equal to the ratio of two
factorials. For example, the hook lengths of the squares in the top row are
1, 2, ... N-K, so we divide by (N-K)! / 0!.
"""

from __future__ import annotations

from typing import List


class Zp:
    """Modular arithmetic helper."""

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


def mod_inv(a: int, m: int) -> int:
    """Modular inverse using Fermat's little theorem."""
    return pow(a, m - 2, m)


def parity(n: int) -> int:
    """Return (-1)^n mod m (for appropriate m)."""
    return 1 if n % 2 == 0 else -1


def solve() -> int:
    """Solve Problem 412."""
    N = 10000
    K = 5000
    M = 76543217

    L = M - (N * N - K * K)
    zp = Zp(L, M)

    # Compute numerator using Wilson's theorem
    ans = mod_inv(parity(L) * zp.factorial(L - 1), M)

    # Compute denominator using hook lengths
    # Top part: rows 0 to K-1
    for s in range(2):
        for i in range(K):
            ans = (ans * zp.inv_factorial(N - K + i)) % M
            ans = (ans * zp.factorial(i)) % M

    # Bottom part: rows K to N-1
    for i in range(K, N):
        ans = (ans * zp.inv_factorial(N + i)) % M
        ans = (ans * zp.factorial(K + i)) % M

    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
