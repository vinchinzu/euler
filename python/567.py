"""Project Euler Problem 567: Reciprocal games I.

Let J_A(n) be the expected value in a game where k is randomly chosen from 1
to n, and if a random binary sequence has k 1s it scores 1/k. Let J_B(n) be
the expected value in a game where two random binary sequence of k 1s are the
same scores 1/k. Find Σ_{n=1}^N (J_A(n) + J_B(n)).

We have J_A(n) = nCr(n,k)/(k*2^n) = 1/2^n Σ_{k=1}^n (2^k - 1) / k and
J_B(n) = 1/(k nCr(n,k)) = 1/2^n Σ_{k=1}^n 2^k / k. So

Σ_n (J_A + J_B) = Σ_n (1/2^n) Σ_k (2^{k+1} - 1) / k
                = Σ_k ( ((2^{k+1} - 1) / k) Σ_n 1/2^n )
                = Σ_k ( (2^{k+1} - 1) / (k 2^{k-1}) )
                = Σ_k ( 4/k - 1/(k 2^{k-1}) ).

The first term sums to 4H_n, and the second term sums to approximately ln 4
for large n.
"""

from __future__ import annotations

import math


def harmonic(n: int) -> float:
    """Compute the n-th harmonic number H_n = 1 + 1/2 + ... + 1/n."""
    if n <= 0:
        return 0.0
    return sum(1.0 / i for i in range(1, n + 1))


def solve() -> float:
    """Solve Problem 567."""
    N = 123456789

    ans = 4 * harmonic(N - 1) - math.log(4)
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.8f}")
    return result


if __name__ == "__main__":
    main()
