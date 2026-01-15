"""Project Euler Problem 553: Power Sets of Power Sets.

Find the number of graphs whose vertices are subsets of {1, ..., N} and such that if two
vertices are connected when they share at least one element, then the number of connected
components is exactly K.

We use exponential generating functions (EGFs) as described in Generatingfunctionology. For
subsets of {1, ..., n}, the total number of graphs without any restrictions is 2^{2^n}.

The number of graphs that contain all of the elements {1, ..., n} can be found by
subtracting the number of graphs with only n-1 elements, etc., via Inclusion Exclusion;
this is equivalent to multiplying the EGF by e^{-x}.

To find the number of connected graphs, we follow section 3.10 in the book, which notes
that this is taking the natural log of the EGF. Since ln(x) = ∫(x'/x), we can compute this.

Now to find the number of graphs with K connected components, we raise the EGF to the Kth
power.

Finally, since we don't need to include all elements of {1, ..., n}, we multiply by e^x.
The coefficient of x^N is almost our answer; to get the correct answer we need to multiply
by N! (since these are EGFs, not GFs), and divide by K! because our connected components
shouldn't be labeled.
"""

from __future__ import annotations

from typing import List


N = 10000
K = 10
M = 10**9 + 7


class LPolynomial:
    """Laurent polynomial (polynomial with potentially negative powers)."""

    def __init__(self, coeffs: List[int]) -> None:
        """Initialize polynomial with coefficients [x^0, x^1, x^2, ...]."""
        self.coefficients = coeffs[:]
        # Remove trailing zeros
        while self.coefficients and self.coefficients[-1] == 0:
            self.coefficients.pop()
        if not self.coefficients:
            self.coefficients = [0]

    def multiply(self, other: "LPolynomial", mod: int) -> "LPolynomial":
        """Multiply two polynomials modulo mod."""
        result = [0] * (len(self.coefficients) + len(other.coefficients))
        for i, c1 in enumerate(self.coefficients):
            for j, c2 in enumerate(other.coefficients):
                result[i + j] = (result[i + j] + c1 * c2) % mod
        return LPolynomial(result)

    def reciprocal(self, mod: int) -> "LPolynomial":
        """Compute reciprocal polynomial (1/p) modulo mod using Newton iteration."""
        # For e^x, reciprocal is e^{-x}
        # We compute this by finding the inverse series
        result = [0] * len(self.coefficients)
        result[0] = pow(self.coefficients[0], mod - 2, mod)  # Inverse of constant term
        for i in range(1, len(self.coefficients)):
            s = 0
            for j in range(1, i + 1):
                s = (s + self.coefficients[j] * result[i - j]) % mod
            result[i] = (-result[0] * s) % mod
        return LPolynomial(result)

    def cap_to(self, degree: int) -> "LPolynomial":
        """Cap polynomial to given degree."""
        return LPolynomial(self.coefficients[: degree + 1])

    def derivative(self, mod: int) -> "LPolynomial":
        """Compute derivative."""
        result = [0] * max(0, len(self.coefficients) - 1)
        for i in range(1, len(self.coefficients)):
            result[i - 1] = (self.coefficients[i] * i) % mod
        return LPolynomial(result)

    def divide(self, other: "LPolynomial", mod: int) -> "LPolynomial":
        """Divide polynomials (p'/p)."""
        num = self.derivative(mod)
        den = other
        # Compute num / den using series division
        # p'/p = (p'/p) where p is the original polynomial
        result = [0] * len(self.coefficients)
        inv_denom0 = pow(den.coefficients[0], mod - 2, mod) if den.coefficients[0] != 0 else 0
        for i in range(len(result)):
            if i < len(num.coefficients):
                s = num.coefficients[i]
            else:
                s = 0
            for j in range(1, min(i + 1, len(den.coefficients))):
                if i - j < len(result):
                    s = (s - den.coefficients[j] * result[i - j]) % mod
            if den.coefficients[0] != 0:
                result[i] = (s * inv_denom0) % mod
        return LPolynomial(result)

    def integral(self, mod: int) -> "LPolynomial":
        """Compute integral (antiderivative)."""
        result = [0] * (len(self.coefficients) + 1)
        for i in range(len(self.coefficients)):
            inv_i_plus_1 = pow(i + 1, mod - 2, mod)
            result[i + 1] = (self.coefficients[i] * inv_i_plus_1) % mod
        return LPolynomial(result)

    def pow(self, k: int, mod_poly: "LPolynomial", mod: int) -> "LPolynomial":
        """Raise polynomial to k-th power modulo mod_poly and mod."""
        if k == 0:
            return LPolynomial([1])
        if k == 1:
            return self.cap_to(len(self.coefficients) - 1)
        half = self.pow(k // 2, mod_poly, mod)
        result = half.multiply(half, mod)
        if k % 2 == 1:
            result = result.multiply(self, mod)
        return result.cap_to(len(self.coefficients) - 1)

    def shift_up(self, n: int) -> "LPolynomial":
        """Shift polynomial up by n degrees."""
        return LPolynomial([0] * n + self.coefficients)


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


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    if mod <= 0:
        msg = "mod must be positive"
        raise ValueError(msg)
    if base == 0:
        return 0
    base %= mod
    result = 1
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def solve() -> int:
    """Solve Problem 553."""
    zp = Zp(N, M)
    num_all_graphs = [0] * (N + 1)
    for i in range(N + 1):
        # 2^{2^i} mod M
        exp = pow_mod(2, i, M - 1)
        num_all_graphs[i] = (
            pow_mod(2, exp, M) * zp.inv_factorial(i) % M
        )

    # e^x coefficients: 1/i!
    e_coeffs = [zp.inv_factorial(i) for i in range(N + 1)]
    e_poly = LPolynomial(e_coeffs)

    # p = num_all_graphs * e^{-x}
    p = LPolynomial(num_all_graphs).multiply(
        e_poly.reciprocal(M), M
    ).cap_to(N)

    # p = p' / p (logarithmic derivative)
    p = p.derivative(M).divide(p, M).integral(M)

    # p = p^K
    mod_poly = LPolynomial([1]).shift_up(N + 1)
    p = p.pow(K, mod_poly, M)

    # p = p * e^x
    p = p.multiply(e_poly, M)

    # Answer: coefficient[N] * N! / K!
    ans = (
        p.coefficients[N] * zp.factorial(N) % M * zp.inv_factorial(K) % M
    )
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
