"""Project Euler Problem 767: Matrix Counting.

Find the number of 16xN matrices such that every entry is 0 or 1, and every
2xK sub-matrix has exactly K 1s.

Consider the leftmost K columns. If the top row has i 1s, then the second row
must have K-i 1s, and the third row must have i 1s, etc., alternating i and
K-i 1s. So there are nCr(K,i) ways to fill each row, and nCr(K,i)^16 ways to
fill all rows. Since i is arbitrary, the total number of ways to fill K columns
is f(K) = Σ_{i=0}^K nCr(K,i).

In most cases, when the leftmost K columns are filled, the remaining columns are
all fixed. As long as a column C has two 1s or two 0s in a row, then the column
K positions to the right (call it D) must also have two 1s/two 0s, and then
the other positions in D also become fixed. The exception is if C alternates 1
and 0 for all 16 rows. In that case, D can also consist of 1s and 0s
alternated in the opposite way.

So suppose that of the leftmost K columns, i of them consist of alternating 1s
and 0s. There are nCr(K,i) ways to choose those columns. For each column, there
are 2^{N/K} ways to fill out all the other columns that are multiple of K
positions to the right of any of those columns, but we need to subtract 2 for
the cases where all the columns have the same pattern, which will be double
counted. Finally, there are f(K-i) ways to fill the remaining K-i rows.

Finally, for performance, we can compute all f(K) efficiently using polynomial
multiplication of two polynomials with coefficients (1/k!)^16 x^k.
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

    def factorial(self, n: int) -> int:
        """Return n! mod mod."""
        return self._factorials[n]

    def inv_factorial(self, n: int) -> int:
        """Return 1/(n!) mod mod."""
        return self._inv_factorials[n]

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


class LPolynomial:
    """Laurent polynomial."""

    def __init__(self, coeffs: List[int]) -> None:
        """Initialize with coefficients."""
        self.coefficients = coeffs[:]

    def multiply(self, other: "LPolynomial", mod: int) -> "LPolynomial":
        """Multiply two polynomials."""
        n = len(self.coefficients)
        m = len(other.coefficients)
        result = [0] * (n + m - 1)
        for i in range(n):
            for j in range(m):
                result[i + j] = (
                    result[i + j] + self.coefficients[i] * other.coefficients[j]
                ) % mod
        return LPolynomial(result)


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
    """Solve Problem 767."""
    N = 10**16
    K = 10**5
    T = 16
    M = 10**9 + 7

    zp = Zp(K, M)

    # Compute polynomial with coefficients (1/k!)^T
    coeffs = [0] * (K + 1)
    for i in range(K + 1):
        coeffs[i] = pow_mod(zp.inv_factorial(i), T, M)

    p = LPolynomial(coeffs)
    p2 = p.multiply(p, M)

    # Compute f[i] = i!^T * p2.coefficients[i]
    f = [0] * (K + 1)
    for i in range(K + 1):
        f[i] = (
            pow_mod(zp.factorial(i), T, M)
            * p2.coefficients[i]
            % M
        )

    ans = 0
    for i in range(K + 1):
        term = (
            zp.nCr(K, i)
            * pow_mod(pow_mod(2, N // K, M) - 2, i, M)
            % M
            * f[K - i]
            % M
        )
        ans = (ans + term) % M

    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
