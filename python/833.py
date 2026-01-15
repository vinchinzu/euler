"""Project Euler Problem 833: Triangular Square.

Find the sum of c for all integer triples (a,b,c) such that c≤N, a<b, and
the product of the triangular numbers T_a and T_b is a perfect square, c².

Fix a. Then we need to find b such that b(b+1) = a(a+1)k², where
c=a(a+1)k/2. The equation can be written as

(2b+1)² - a(a+1) (2k)² = 1,

which is a Pell equation with a "trivial" solution at (2b+1,2k) = (2a+1,2)
(so b=a, k=1). If that's the base solution, we can find all solutions
using the theory of Pell equations.

However, that might not be base solution if a(a+1) isn't square-free. But
if it's not, it must be a solution generated from a smaller a. So for
each set of solutions of a Pell equation, we take all pairs of two
solutions (2k1, 2k2), where c=D(2k1)(2k2)/8. But to avoid
double-counting when we iterate over larger a, we only take pairs of
solutions where the indices of the two solutions are relatively prime.

The above can be done for each a, but the final values of c are
polynomials in a. So instead we symbolically determine the polynomials and
the sum of polynomials, and for each pair of two symbolic solutions we can
compute the sum of all c in logarithmic time.
"""

from __future__ import annotations

from math import gcd
from typing import List


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def mod_inverse(a: int, m: int) -> int:
    """Modular inverse."""
    return pow(a, -1, m)


def sum_powers(n: int, k: int, mod: int) -> int:
    """Sum of k-th powers from 1 to n mod mod."""
    if k == 0:
        return n % mod
    if k == 1:
        return (n * (n + 1) // 2) % mod
    if k == 2:
        return (n * (n + 1) * (2 * n + 1) // 6) % mod
    # For higher powers, use Faulhaber's formula (simplified)
    result = 0
    for i in range(1, n + 1):
        result = (result + pow_mod(i, k, mod)) % mod
    return result


class Polynomial:
    """Simple polynomial representation."""

    def __init__(self, coeffs: List[int]):
        """Initialize polynomial with coefficients."""
        self.coeffs = coeffs[:]
        # Remove trailing zeros
        while self.coeffs and self.coeffs[-1] == 0:
            self.coeffs.pop()

    def evaluate(self, x: int) -> int:
        """Evaluate polynomial at x."""
        result = 0
        power = 1
        for coeff in self.coeffs:
            result += coeff * power
            power *= x
        return result

    def add(self, other: "Polynomial") -> "Polynomial":
        """Add two polynomials."""
        max_len = max(len(self.coeffs), len(other.coeffs))
        result_coeffs = [0] * max_len
        for i in range(len(self.coeffs)):
            result_coeffs[i] += self.coeffs[i]
        for i in range(len(other.coeffs)):
            result_coeffs[i] += other.coeffs[i]
        return Polynomial(result_coeffs)

    def multiply(self, other: "Polynomial") -> "Polynomial":
        """Multiply two polynomials."""
        result_coeffs = [0] * (len(self.coeffs) + len(other.coeffs))
        for i, c1 in enumerate(self.coeffs):
            for j, c2 in enumerate(other.coeffs):
                result_coeffs[i + j] += c1 * c2
        return Polynomial(result_coeffs)

    def mod(self, m: int) -> "Polynomial":
        """Take coefficients mod m."""
        return Polynomial([c % m for c in self.coeffs])


def solve() -> int:
    """Solve Problem 833."""
    N = 10**35
    M = 136101521

    # Initialize Pell equation: D = a(a+1) = x(x+1)
    # Base solution: x = (1, 2), y = (2)
    D = Polynomial([0, 1, 1])  # x^2 + x
    base_x = Polynomial([1, 2])  # 2x + 1
    base_y = Polynomial([2])  # 2

    x = base_x
    y = base_y
    ys: List[Polynomial] = []

    # Generate solutions until y(1) >= N
    while y.evaluate(1) < N:
        ys.append(y)
        # Next solution: (x', y') = (x*base_x + D*y*base_y, x*base_y + y*base_x)
        new_x = x.multiply(base_x).add(
            D.multiply(y).multiply(base_y)
        )
        new_y = x.multiply(base_y).add(y.multiply(base_x))
        x = new_x
        y = new_y

    ans = 0
    for i in range(len(ys)):
        for j in range(i + 1, len(ys)):
            if gcd(i + 1, j + 1) == 1:
                # Compute product polynomial
                prod = D.multiply(ys[i]).multiply(ys[j])

                # Binary search for maximum a
                low = 0
                high = 2**60
                while low + 1 < high:
                    mid = (low + high) // 2
                    if prod.evaluate(mid) // 8 <= N:
                        low = mid
                    else:
                        high = mid

                # Sum polynomial coefficients
                prod_mod = prod.mod(M)
                for e, coeff in enumerate(prod_mod.coeffs):
                    if coeff != 0:
                        ans = (ans + sum_powers(low, e, M) * coeff) % M

    ans = (ans * mod_inverse(8, M)) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
