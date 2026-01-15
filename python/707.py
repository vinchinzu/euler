"""Project Euler Problem 707: Lights Out.

Let F(w, h) be the number of solvable Lights Out games on a w x h grid. Find
Σ_{k=1}^N F(N, f_k), where f_k is the k'th Fibonacci number.

If the Lights Out matrix has a corank of R, then the number of solvable games
is 2^{w*h - R}. According to https://www.win.tue.nl/~aeb/ca/madness/madrect.html,
the corank can be computed as the degree of GCD(p_w(x), p_h(x+1)), where the
polynomials p are defined by p_0(x) = 1, p_1(x) = x, and
p_{n+1}(x) = x*p_n(x) + p_{n-1}(x).

As an optimization, we can compute all p_h(x+1) modulo p_N(x), which ensures
that the polynomials are always relatively small (having no more than degree N).
And at some point p_h(x+1) ≡ 0 (mod p_N(x)), so the coranks are periodic and
we can take the modulo of the large Fibonacci numbers f_k by this period length.
"""

from __future__ import annotations

from typing import List


class LPolynomial:
    """Laurent polynomial (polynomial with integer coefficients)."""

    ZERO: LPolynomial

    def __init__(self, *coeffs: int) -> None:
        """Initialize polynomial with coefficients.

        Args:
            *coeffs: Variable number of coefficients. If one int, it's the
                     constant term. If two ints, they are [const, x^1].
        """
        if len(coeffs) == 1 and isinstance(coeffs[0], int):
            # Single integer: constant term
            self.coefficients = [coeffs[0]]
        else:
            # Multiple coefficients: [x^0, x^1, x^2, ...]
            self.coefficients = list(coeffs)
        # Remove trailing zeros
        while self.coefficients and self.coefficients[-1] == 0:
            self.coefficients.pop()
        if not self.coefficients:
            self.coefficients = [0]

    def shift_up(self, n: int) -> LPolynomial:
        """Shift polynomial up by n degrees."""
        return LPolynomial([0] * n + self.coefficients)

    def add(self, other: LPolynomial, mod: int) -> LPolynomial:
        """Add another polynomial modulo mod."""
        max_len = max(len(self.coefficients), len(other.coefficients))
        result = [0] * max_len
        for i in range(len(self.coefficients)):
            result[i] = self.coefficients[i] % mod
        for i in range(len(other.coefficients)):
            result[i] = (result[i] + other.coefficients[i]) % mod
        return LPolynomial(*result)

    def mod(self, mod_poly: LPolynomial, mod: int) -> LPolynomial:
        """Reduce polynomial modulo mod_poly and mod."""
        if len(self.coefficients) < len(mod_poly.coefficients):
            return self
        result = LPolynomial(*self.coefficients)
        while len(result.coefficients) >= len(mod_poly.coefficients):
            if result.coefficients[-1] == 0:
                result.coefficients.pop()
                continue
            factor = result.coefficients[-1]
            shift = len(result.coefficients) - len(mod_poly.coefficients)
            for i, c in enumerate(mod_poly.coefficients):
                idx = shift + i
                if idx < len(result.coefficients):
                    result.coefficients[idx] = (
                        result.coefficients[idx] - factor * c
                    ) % mod
            result.coefficients.pop()
            # Remove trailing zeros
            while result.coefficients and result.coefficients[-1] == 0:
                result.coefficients.pop()
            if not result.coefficients:
                result.coefficients = [0]
        return result

    def gcd(self, other: LPolynomial, mod: int) -> LPolynomial:
        """Compute GCD of two polynomials modulo mod."""
        a = self
        b = other
        while not b.equals(LPolynomial.ZERO):
            a, b = b, a.mod(b, mod)
        return a

    def degree(self) -> int:
        """Return the degree of the polynomial."""
        if len(self.coefficients) == 1 and self.coefficients[0] == 0:
            return -1
        return len(self.coefficients) - 1

    def equals(self, other: LPolynomial) -> bool:
        """Check if two polynomials are equal."""
        return self.coefficients == other.coefficients


# Initialize ZERO constant
LPolynomial.ZERO = LPolynomial(0)


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


def fibonacci(n: int, mod: int) -> int:
    """Compute n-th Fibonacci number modulo mod."""
    if n <= 1:
        return n % mod

    def mat_mult(a: List[List[int]], b: List[List[int]], mod: int) -> List[List[int]]:
        """Multiply two 2x2 matrices modulo mod."""
        return [
            [
                (a[0][0] * b[0][0] + a[0][1] * b[1][0]) % mod,
                (a[0][0] * b[0][1] + a[0][1] * b[1][1]) % mod,
            ],
            [
                (a[1][0] * b[0][0] + a[1][1] * b[1][0]) % mod,
                (a[1][0] * b[0][1] + a[1][1] * b[1][1]) % mod,
            ],
        ]

    def mat_pow(mat: List[List[int]], exp: int, mod: int) -> List[List[int]]:
        """Raise matrix to power modulo mod."""
        if exp == 0:
            return [[1, 0], [0, 1]]
        if exp == 1:
            return mat
        half = mat_pow(mat, exp // 2, mod)
        result = mat_mult(half, half, mod)
        if exp % 2 == 1:
            result = mat_mult(result, mat, mod)
        return result

    fib_mat = [[1, 1], [1, 0]]
    result_mat = mat_pow(fib_mat, n - 1, mod)
    return result_mat[0][0] % mod


def solve() -> int:
    """Solve Problem 707."""
    N = 199
    M = 10**9 + 7

    # Build p_n(x) polynomials: p_0 = 1, p_1 = x, p_{n+1} = x*p_n + p_{n-1}
    px: List[LPolynomial] = [LPolynomial(1), LPolynomial(0, 1)]
    for n in range(2, N + 1):
        # p_n = x * p_{n-1} + p_{n-2}
        # x * p_{n-1} is shift_up(1)
        new_poly = px[-1].shift_up(1).add(px[-2], 2)
        px.append(new_poly)

    # Build p_n(x+1) polynomials modulo p_N(x)
    px1: List[LPolynomial] = [LPolynomial(1), LPolynomial(1, 1)]
    while not px1[-1].equals(LPolynomial.ZERO):
        # p_{n+1}(x+1) = (x+1) * p_n(x+1) + p_{n-1}(x+1)
        # (x+1) * p_n = x*p_n + p_n = shift_up(1) + p_n
        new_poly = (
            px1[-1]
            .shift_up(1)
            .add(px1[-1], 2)
            .add(px1[-2], 2)
            .mod(px[N], 2)
        )
        px1.append(new_poly)
    period = len(px1)

    ans = 0
    for k in range(1, N + 1):
        fib_k_mod_period = fibonacci(k, period)
        fib_k_mod_m_minus_1 = fibonacci(k, M - 1)
        gcd_poly = px[N].gcd(px1[fib_k_mod_period], 2)
        corank = gcd_poly.degree()
        exponent = N * fib_k_mod_m_minus_1 - corank
        ans = (ans + pow_mod(2, exponent, M)) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
