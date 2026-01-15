"""Project Euler Problem 559: Permutation matrices.

Let P(k, r, n) be the number of r x n matrices such that each row is a
permutation of 1 to n, and iff column i is divisible by k, then each element is
smaller than the one to its right. Find Σ_{k=1}^N P(k, N, N).

Suppose that there are groups of columns of sizes g_t that are ascending, where
Σ g_t = N. Then the number of matrices ascending in at least these columns is
(N! / Π_t (g_t)!)^r, where the base is the number of ways to partition the
elements of 1 to n into groups with those sizes. For a given k, the n columns
are already grouped into ⌈N/k⌉ parts, and they can be further grouped together.

To compute the number of matrices ascending at exactly those columns, we use
Inclusion Exclusion, considering all possible ways that the ⌈N/k⌉ parts can be
grouped together. This can be computed efficiently using dynamic programming
where dp(i) is Σ N! / Π (g_t)! where the g_t add up to the length of the first
i parts, and keeping track of the number of groups to determine the sign.
Finally, for performance, we precompute all inverse factorials, and pull out
the factor of N!.
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


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Compute base^exp mod mod."""
    result = 1
    base %= mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp //= 2
    return result


def parity(n: int) -> int:
    """Return (-1)^n."""
    return 1 if n % 2 == 0 else -1


def solve() -> int:
    """Solve Problem 559."""
    N = 50000
    M = 10**9 + 123

    zp = Zp(N, M)
    pow_inv_factorials = [0] * (N + 1)
    for i in range(N + 1):
        pow_inv_factorials[i] = pow_mod(zp.inv_factorial(i), N, M)

    def P(k: int) -> int:
        """Compute P(k, N, N)."""
        # Partition columns into groups of size k
        parts: List[int] = []
        for length in range(0, N, k):
            parts.append(length)
        parts.append(N)
        num_parts = len(parts)

        # DP: dp[i] = sum over ways to group first i parts
        dp = [0] * num_parts
        dp[0] = 1
        for i in range(1, num_parts):
            for start in range(i):
                length = parts[i] - parts[start]
                dp[i] = (dp[i] - pow_inv_factorials[length] * dp[start]) % M

        return (-parity(num_parts) * dp[num_parts - 1]) % M

    ans = 0
    for k in range(1, N + 1):
        ans = (ans + P(k)) % M

    # Multiply by (N!)^N
    ans = (ans * pow_mod(zp.factorial(N), N, M)) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
