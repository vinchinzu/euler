"""Project Euler Problem 427: n-sequences.

If S is a sequence of N integers, each from 1 to N inclusive, let L(S) be the
length of the longest contiguous subsequence of the same value. Find Σ L(S)
over all such sequences S.

We use generating functions and binomial coefficients to compute this
efficiently.
"""

from __future__ import annotations

from typing import List


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


def ipow(base: int, exp: int) -> int:
    """Integer power."""
    result = 1
    for _ in range(exp):
        result *= base
    return result


def pows(base: int, max_exp: int, mod: int) -> List[int]:
    """Precompute powers of base up to max_exp."""
    result = [1] * (max_exp + 1)
    for i in range(1, max_exp + 1):
        result[i] = (result[i - 1] * base) % mod
    return result


def parity(i: int) -> int:
    """Return (-1)^i."""
    return 1 if i % 2 == 0 else -1


def solve() -> int:
    """Solve Problem 427."""
    N = 7500000
    M = ipow(10, 9) + 9

    pow_ns = pows(N, N, M)
    pow_nm1s = pows(N - 1, N, M)
    zp = Zp(N, M)

    f = [0] * (N + 1)
    for k in range(1, N + 1):
        for i in range(0, N + 1):
            if i * (k + 1) > N:
                break
            A = N - i * k - 1
            term = 0
            term += (
                zp.nCr(A, i) * pow_nm1s[i] % M * pow_ns[A - i + 1]
            ) % M
            if i >= 1:
                term += (
                    zp.nCr(A, i - 1)
                    * pow_nm1s[i - 1]
                    % M
                    * pow_ns[A - i + 2]
                ) % M
            f[k] = (f[k] + parity(i) * term) % M

    ans = 0
    for k in range(1, N + 1):
        ans = (ans + (f[k] - f[k - 1]) * k) % M

    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
