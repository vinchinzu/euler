"""Project Euler Problem 258: A lagged Fibonacci sequence.

Find g_N (mod M), where g_k = 1 for 0 ≤ k < K and
g_k = g_(k-K) + g(k-K+1) for k ≥ K.
"""

from __future__ import annotations

from typing import List


class Polynomial:
    """Polynomial with integer coefficients."""

    def __init__(self, coeffs: List[int]) -> None:
        """Initialize polynomial with coefficients."""
        self.coeffs = coeffs[:]
        while len(self.coeffs) > 1 and self.coeffs[-1] == 0:
            self.coeffs.pop()

    def shift_up(self, n: int) -> "Polynomial":
        """Shift polynomial up by n degrees."""
        return Polynomial([0] * n + self.coeffs)

    def subtract(self, other: "Polynomial", mod: int) -> "Polynomial":
        """Subtract another polynomial modulo mod."""
        max_len = max(len(self.coeffs), len(other.coeffs))
        result = [0] * max_len
        for i in range(len(self.coeffs)):
            result[i] = self.coeffs[i] % mod
        for i in range(len(other.coeffs)):
            result[i] = (result[i] - other.coeffs[i]) % mod
        return Polynomial(result)

    def pow_mod(self, n: int, mod_poly: "Polynomial", mod: int) -> "Polynomial":
        """Compute polynomial power modulo mod_poly and mod."""
        if n == 0:
            return Polynomial([1])
        if n == 1:
            return self.mod_reduce(mod_poly, mod)
        half = self.pow_mod(n // 2, mod_poly, mod)
        result = half.multiply(half, mod).mod_reduce(mod_poly, mod)
        if n % 2 == 1:
            result = result.multiply(self, mod).mod_reduce(mod_poly, mod)
        return result

    def multiply(self, other: "Polynomial", mod: int) -> "Polynomial":
        """Multiply by another polynomial modulo mod."""
        result = [0] * (len(self.coeffs) + len(other.coeffs) - 1)
        for i, c1 in enumerate(self.coeffs):
            for j, c2 in enumerate(other.coeffs):
                result[i + j] = (result[i + j] + c1 * c2) % mod
        return Polynomial(result)

    def mod_reduce(self, mod_poly: "Polynomial", mod: int) -> "Polynomial":
        """Reduce polynomial modulo mod_poly."""
        if len(self.coeffs) < len(mod_poly.coeffs):
            return self
        result = Polynomial(self.coeffs[:])
        while len(result.coeffs) >= len(mod_poly.coeffs):
            if result.coeffs[-1] == 0:
                result.coeffs.pop()
                continue
            factor = result.coeffs[-1]
            shift = len(result.coeffs) - len(mod_poly.coeffs)
            for i, c in enumerate(mod_poly.coeffs):
                idx = shift + i
                if idx < len(result.coeffs):
                    result.coeffs[idx] = (result.coeffs[idx] - factor * c) % mod
            result.coeffs.pop()
        return result

    def evaluate(self, x: int, mod: int) -> int:
        """Evaluate polynomial at x modulo mod."""
        result = 0
        power = 1
        for coeff in self.coeffs:
            result = (result + coeff * power) % mod
            power = (power * x) % mod
        return result


def solve() -> int:
    """Solve Problem 258."""
    N = 10**18
    K = 2000
    M = 20092010

    # x^K - x - 1
    mod_poly = Polynomial([1]).shift_up(K).subtract(Polynomial([1, 1]), M)

    # x^N mod (x^K - x - 1)
    result_poly = Polynomial([0, 1]).pow_mod(N, mod_poly, M)

    # Evaluate at x=1
    return result_poly.evaluate(1, M)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
