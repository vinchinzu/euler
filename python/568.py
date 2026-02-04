"""Project Euler Problem 568: Reciprocal Games II.

D(n) = J_B(n) - J_A(n) = H_n / 2^n, where H_n is the n-th harmonic number.

We need the 7 most significant digits of D(123456789) after removing leading zeros.

Approach: compute log10(D(n)) = log10(H_n) - n*log10(2) using high-precision
arithmetic (mpmath), then extract the significant digits from the fractional part.
"""

from __future__ import annotations

import math
from mpmath import mp, mpf, log10, harmonic


def solve() -> str:
    """Solve Problem 568."""
    mp.dps = 50  # 50 decimal digits of precision
    N = 123456789

    # H_N via Euler-Maclaurin summation (fast for large N in mpmath)
    H_N = harmonic(N)

    # log10(D(N)) = log10(H_N) - N * log10(2)
    log10_H_N = float(log10(H_N))
    N_log10_2 = float(N * log10(mpf(2)))
    log10_D = log10_H_N - N_log10_2

    # Extract 7 most significant digits from the fractional part of log10
    frac = log10_D - math.floor(log10_D)
    significant = 10**frac
    return str(significant).replace(".", "")[:7]


def main() -> str:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
