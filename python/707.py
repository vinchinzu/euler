"""Project Euler Problem 707: Lights Out.

Let F(w, h) be the number of solvable Lights Out games on a w x h grid. Find
sum_{k=1}^N F(N, f_k), where f_k is the k'th Fibonacci number.

If the Lights Out matrix has a corank of R, then the number of solvable games
is 2^{w*h - R}. The corank can be computed as the degree of GCD(p_w(x), p_h(x+1)),
where the polynomials p are defined by p_0(x) = 1, p_1(x) = x, and
p_{n+1}(x) = x*p_n(x) + p_{n-1}(x).

As an optimization, we compute all p_h(x+1) modulo p_N(x), which ensures
that the polynomials are always relatively small. At some point p_h(x+1) = 0
(mod p_N(x)), so the coranks are periodic and we can take the modulo of the
large Fibonacci numbers f_k by this period length.
"""

from __future__ import annotations

from typing import List


class Poly:
    """Polynomial with integer coefficients, stored as list [c0, c1, ..., cd]."""

    def __init__(self, coeffs: List[int]):
        """Initialize with list of coefficients. coeffs[i] is coefficient of x^i."""
        self.c = list(coeffs)
        # Remove trailing zeros
        while len(self.c) > 1 and self.c[-1] == 0:
            self.c.pop()

    @staticmethod
    def zero():
        return Poly([0])

    @staticmethod
    def one():
        return Poly([1])

    @staticmethod
    def x():
        return Poly([0, 1])

    def is_zero(self) -> bool:
        return len(self.c) == 1 and self.c[0] == 0

    def degree(self) -> int:
        if self.is_zero():
            return -1
        return len(self.c) - 1

    def shift_up(self, n: int) -> Poly:
        """Multiply by x^n."""
        return Poly([0] * n + self.c)

    def add(self, other: Poly, mod: int) -> Poly:
        """Add another polynomial, coefficients mod mod."""
        max_len = max(len(self.c), len(other.c))
        result = [0] * max_len
        for i in range(len(self.c)):
            result[i] = self.c[i] % mod
        for i in range(len(other.c)):
            result[i] = (result[i] + other.c[i]) % mod
        return Poly(result)

    def mod_poly(self, divisor: Poly, mod: int) -> Poly:
        """Reduce self modulo divisor polynomial, with coefficients mod mod."""
        if len(self.c) < len(divisor.c):
            return Poly([x % mod for x in self.c])
        result = [x % mod for x in self.c]
        while len(result) >= len(divisor.c):
            if result[-1] == 0:
                result.pop()
                if not result:
                    return Poly([0])
                continue
            factor = result[-1]
            shift = len(result) - len(divisor.c)
            for i in range(len(divisor.c)):
                result[shift + i] = (result[shift + i] - factor * divisor.c[i]) % mod
            result.pop()
            if not result:
                return Poly([0])
        # Clean trailing zeros
        while len(result) > 1 and result[-1] == 0:
            result.pop()
        return Poly(result)

    def gcd(self, other: Poly, mod: int) -> Poly:
        """Compute GCD of two polynomials with coefficients mod mod."""
        a = self
        b = other
        while not b.is_zero():
            a, b = b, a.mod_poly(b, mod)
        return a

    def __eq__(self, other):
        if isinstance(other, Poly):
            return self.c == other.c
        return False


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    if mod <= 0:
        raise ValueError("mod must be positive")
    if mod == 1:
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
    """Compute n-th Fibonacci number modulo mod using matrix exponentiation."""
    if n <= 0:
        return 0
    if n == 1:
        return 1 % mod

    def mat_mult(a, b, m):
        return [
            [(a[0][0] * b[0][0] + a[0][1] * b[1][0]) % m,
             (a[0][0] * b[0][1] + a[0][1] * b[1][1]) % m],
            [(a[1][0] * b[0][0] + a[1][1] * b[1][0]) % m,
             (a[1][0] * b[0][1] + a[1][1] * b[1][1]) % m],
        ]

    def mat_pow(mat, exp, m):
        result = [[1, 0], [0, 1]]
        base = [row[:] for row in mat]
        while exp > 0:
            if exp & 1:
                result = mat_mult(result, base, m)
            base = mat_mult(base, base, m)
            exp >>= 1
        return result

    fib_mat = [[1, 1], [1, 0]]
    result_mat = mat_pow(fib_mat, n - 1, mod)
    return result_mat[0][0] % mod


def solve() -> int:
    """Solve Problem 707."""
    N = 199
    M = 10**9 + 7

    # Build p_n(x) polynomials: p_0 = 1, p_1 = x, p_{n+1} = x*p_n + p_{n-1}
    # All arithmetic mod 2 (GF(2))
    px = [Poly.one(), Poly.x()]
    for n in range(2, N + 1):
        # p_n = x * p_{n-1} + p_{n-2}
        new_poly = px[-1].shift_up(1).add(px[-2], 2)
        px.append(new_poly)

    # Build p_h(x+1) polynomials modulo p_N(x), all mod 2
    # p_0(x+1) = 1, p_1(x+1) = x+1
    # p_{n+1}(x+1) = (x+1)*p_n(x+1) + p_{n-1}(x+1)
    px1 = [Poly.one(), Poly([1, 1])]
    while not px1[-1].is_zero():
        last = px1[-1]
        penult = px1[-2]
        # (x+1)*last = x*last + last
        new_poly = last.shift_up(1).add(last, 2).add(penult, 2).mod_poly(px[N], 2)
        px1.append(new_poly)
    period = len(px1)

    ans = 0
    for k in range(1, N + 1):
        fib_k_mod_period = fibonacci(k, period)
        fib_k_mod_m_minus_1 = fibonacci(k, M - 1)
        gcd_poly = px[N].gcd(px1[fib_k_mod_period], 2)
        corank = gcd_poly.degree()
        if corank < 0:
            corank = 0
        exponent = (N * fib_k_mod_m_minus_1 - corank) % (M - 1)
        if exponent < 0:
            exponent += M - 1
        ans = (ans + pow_mod(2, exponent, M)) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
