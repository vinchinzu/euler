"""Project Euler Problem 722: Slowly converging series.

Find Σ_{n=1}^∞ σ_K(n) Q^n, where σ_K(n) is the sum of the Kth powers of all
divisors of n.

We have
S = Σ_{n=1}^∞ σ_K(n) Q^n
  = Σ_{n=1}^∞ n^K (Q^n + Q^{2n} + Q^{3n} + ...)
  = Σ_{t=1}^∞ Li_{-K}(Q^t),

where Li_{-K} is the poly-logarithm, which can be computed by starting with
the (natural) logarithm and applying f(z) -> zf'(z) K times. The series
converges quickly, so we can stop once the sum stops changing.
"""

from __future__ import annotations

from typing import List


class FPolynomial:
    """Floating-point polynomial."""

    def __init__(self, *coeffs: float) -> None:
        """Initialize polynomial with coefficients."""
        if len(coeffs) == 1 and isinstance(coeffs[0], (int, float)):
            self.coefficients = [float(coeffs[0])]
        else:
            self.coefficients = [float(c) for c in coeffs]
        # Remove trailing zeros
        while self.coefficients and abs(self.coefficients[-1]) < 1e-15:
            self.coefficients.pop()
        if not self.coefficients:
            self.coefficients = [0.0]

    def derivative(self) -> FPolynomial:
        """Compute derivative."""
        if len(self.coefficients) <= 1:
            return FPolynomial(0.0)
        result = [0.0] * (len(self.coefficients) - 1)
        for i in range(1, len(self.coefficients)):
            result[i - 1] = self.coefficients[i] * i
        return FPolynomial(*result)

    def multiply(self, other: FPolynomial) -> FPolynomial:
        """Multiply two polynomials."""
        result = [0.0] * (len(self.coefficients) + len(other.coefficients) - 1)
        for i, c1 in enumerate(self.coefficients):
            for j, c2 in enumerate(other.coefficients):
                result[i + j] += c1 * c2
        return FPolynomial(*result)

    def add(self, other: FPolynomial) -> FPolynomial:
        """Add two polynomials."""
        max_len = max(len(self.coefficients), len(other.coefficients))
        result = [0.0] * max_len
        for i in range(len(self.coefficients)):
            result[i] += self.coefficients[i]
        for i in range(len(other.coefficients)):
            result[i] += other.coefficients[i]
        return FPolynomial(*result)

    def shift_up(self, n: int) -> FPolynomial:
        """Shift polynomial up by n degrees."""
        return FPolynomial(*([0.0] * n + self.coefficients))

    def evaluate(self, x: float) -> float:
        """Evaluate polynomial at x."""
        result = 0.0
        power = 1.0
        for coeff in self.coefficients:
            result += coeff * power
            power *= x
        return result


def solve() -> float:
    """Solve Problem 722."""
    Q = 1 - 0.5**25
    K = 15

    # Start with num = x, den = 1 - x
    num = FPolynomial(0.0, 1.0)
    den = FPolynomial(1.0, -1.0)

    # Apply f(z) -> zf'(z) K times
    for i in range(1, K + 1):
        # num = num.derivative().multiply(den).add(num.multiply(i)).shiftUp(1)
        num_deriv = num.derivative()
        num_deriv_times_den = num_deriv.multiply(den)
        num_times_i = FPolynomial(*[c * i for c in num.coefficients])
        num = num_deriv_times_den.add(num_times_i).shift_up(1)

    ans = 0.0
    prev_ans = -1.0
    i = 1
    while True:
        z = Q**i
        num_val = num.evaluate(z)
        den_val = (1 - z) ** (K + 1)
        res = num_val / den_val
        ans += res
        # Check convergence
        if abs(ans - prev_ans) < 1e-10:
            break
        prev_ans = ans
        i += 1
        if i > 1000:  # Safety limit
            break

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(f"{result:.12e}")
    return 0


if __name__ == "__main__":
    main()
