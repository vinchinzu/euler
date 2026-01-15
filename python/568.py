"""Project Euler Problem 568: Reciprocal games II.

Let J_A(n) be the expected value in a game where k is randomly chosen from 1
to n, and if a random binary sequence has k 1s it scores 1/k. Let J_B(n) be
the expected value in a game where two random binary sequence of k 1s are the
same scores 1/k. Find J_B(N) - J_A(N).

Similarly to problem 567 we have J_A(n) = nCr(n,k)/(k*2^n) =
1/2^n Σ_{k=1}^n (2^k - 1) / k and J_B(n) = 1/(k nCr(n,k)) =
1/2^n Σ_{k=1}^n 2^k / k. So

J_B - J_A = (1/2^N) Σ_k 1/k
          = H_N / 2^N.
"""

from __future__ import annotations

import math


def harmonic(n: int) -> float:
    """Compute the n-th harmonic number H_n = 1 + 1/2 + ... + 1/n."""
    if n <= 0:
        return 0.0
    return sum(1.0 / i for i in range(1, n + 1))


def solve() -> str:
    """Solve Problem 568."""
    N = 123456789

    h_n = harmonic(N)
    log10_h_n = math.log10(h_n)
    log10_2_n = N * math.log10(2)
    ans = math.pow(10, log10_h_n - log10_2_n)
    # Return fractional part after decimal point
    fractional = ans - int(ans)
    return f"{fractional:.7f}".replace("0.", "")


def main() -> str:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
