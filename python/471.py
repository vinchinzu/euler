"""Project Euler Problem 471: Triangle inscribed in ellipse.

Let r(a, b) be the radius of a circle centered at (2b, 0) such that it is the
in-circle of a triangle with one vertex at (a/2, √3/2 b) and all vertices on
the ellipse x²/a² + y²/b² = 1. Find G(N) = Σ_{a=3}^N Σ_{b=1}^⌊(a-1)/2⌋ r(a, b).
"""

from __future__ import annotations
from decimal import Decimal, getcontext
import math

# Set precision high enough for accurate calculation
getcontext().prec = 100


def harmonic(n: int) -> Decimal:
    """Harmonic number using asymptotic approximation for large n.

    For large n: H(n) ≈ ln(n) + γ + 1/(2n) - 1/(12n²) + 1/(120n⁴)
    where γ ≈ 0.5772156649015329 (Euler-Mascheroni constant)
    """
    if n <= 0:
        return Decimal(0)

    n_dec = Decimal(n)

    # Euler-Mascheroni constant with high precision
    gamma = Decimal('0.5772156649015328606065120900824024310421593359399235988057672348848677267776646709369470632917467495')

    # Natural logarithm using Python's math.log, then convert to Decimal
    ln_n = Decimal(str(math.log(n)))

    # Higher order terms for better accuracy
    correction = (Decimal(1) / (Decimal(2) * n_dec)
                  - Decimal(1) / (Decimal(12) * n_dec * n_dec)
                  + Decimal(1) / (Decimal(120) * n_dec**4))

    return ln_n + gamma + correction


def solve() -> int:
    """Solve Problem 471."""
    N = Decimal('100000000000')  # 10^11

    # Calculate G(N) = N(2N-1)(3N+4)/24 - N(N+1)(2N+1)[H(N)-H(N/2)]/6
    term1 = N * (2 * N - 1) * (3 * N + 4) / 24

    h_diff = harmonic(int(N)) - harmonic(int(N / 2))
    term2 = N * (N + 1) * (2 * N + 1) * h_diff / 6

    ans = term1 - term2

    # Round to nearest integer for final answer
    return round(ans)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
