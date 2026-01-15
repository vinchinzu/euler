"""Project Euler Problem 788: Dominating Numbers.

Find the number of dominating numbers, which are numbers where more than half
of its digits are equal, that have up to N digits.

Let l be the number of non-zero digits. Then ⌊l/2⌋+1 ≤ k ≤ l of them must be
the same digit d. First suppose d≠0 (so there are 9 choices). If the first
digit is d, then k-1 of the remaining l-1 digits must be d, and there are 9
choices for each of the remaining l-k digits. If the first digit is not d,
then k of the remaining l-1 digits must be d, and there are only 8 choices for
the first digit (which can't be zero), and 9 for each of the rest. Finally,
if d=0, then the first digit can't be d, so k of the remaining l-1 digits must
be d, and there are 9 choices for each of the remaining l-k digits.
"""

from __future__ import annotations

from typing import List


class Zp:
    """Modular arithmetic helper class."""

    def __init__(self, n: int, mod: int) -> None:
        """Initialize with precomputed factorials."""
        self.mod = mod
        self._factorials = [1] * (n + 1)
        self._inv_factorials = [1] * (n + 1)

        for i in range(1, n + 1):
            self._factorials[i] = (self._factorials[i - 1] * i) % mod

        self._inv_factorials[n] = pow(self._factorials[n], mod - 2, mod)
        for i in range(n - 1, -1, -1):
            self._inv_factorials[i] = (
                self._inv_factorials[i + 1] * (i + 1)
            ) % mod

    def nCr(self, n: int, k: int) -> int:
        """Binomial coefficient C(n, k) mod mod."""
        if k < 0 or k > n:
            return 0
        return (
            self._factorials[n]
            * self._inv_factorials[k]
            % self.mod
            * self._inv_factorials[n - k]
            % self.mod
        )


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def solve() -> int:
    """Solve Problem 788."""
    N = 2022
    M = 10**9 + 7
    B = 10

    zp = Zp(N, M)
    ans = 0

    for l in range(1, N + 1):
        for k in range(l // 2 + 1, l + 1):
            # Case 1: d ≠ 0, first digit is d
            ans = (
                ans
                + (B - 1)
                * zp.nCr(l - 1, k - 1)
                % M
                * pow_mod(B - 1, l - k, M)
                % M
            ) % M

            # Case 2: d ≠ 0, first digit is not d
            ans = (
                ans
                + (B - 1)
                * zp.nCr(l - 1, k)
                % M
                * (B - 2)
                % M
                * pow_mod(B - 1, l - k - 1, M)
                % M
            ) % M

            # Case 3: d = 0
            ans = (
                ans + zp.nCr(l - 1, k) * pow_mod(B - 1, l - k, M) % M
            ) % M

    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
