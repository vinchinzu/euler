"""Project Euler Problem 470: Super Ramvok.

In the game of Ramvok, a player first chooses a positive integer t and pays
c*t. She can then roll a d-sided die (with sides from 1 to d) up to t times,
choosing to stop at any time to earn the value of the face-up side. In the
game of Super-Ramvok, the player repeatedly plays Ramvok, but after each round,
the number on a random side is either erased or added back. Compute
Σ_{4≤d≤N} Σ_{0≤c≤n} S(d, c).
"""

from __future__ import annotations

from typing import List


def ilog2(n: int) -> int:
    """Integer logarithm base 2."""
    return n.bit_length() - 1


def tridiagonal_system(
    A: List[float], B: List[float], C: List[float], D: List[float]
) -> List[float]:
    """Solve tridiagonal system."""
    n = len(B)
    x = [0.0] * n
    # Thomas algorithm
    c_prime = [0.0] * n
    d_prime = [0.0] * n

    c_prime[0] = C[0] / B[0]
    d_prime[0] = D[0] / B[0]

    for i in range(1, n):
        denom = B[i] - A[i] * c_prime[i - 1]
        c_prime[i] = C[i] / denom if i < n - 1 else 0.0
        d_prime[i] = (D[i] - A[i] * d_prime[i - 1]) / denom

    x[n - 1] = d_prime[n - 1]
    for i in range(n - 2, -1, -1):
        x[i] = d_prime[i] - c_prime[i] * x[i + 1]

    return x


def R(subset: int, c: float, N: int) -> float:
    """Expected earnings in Ramvok."""
    if c == 0:
        return 1 + ilog2(subset)

    # Extract values from subset
    vals: List[float] = []
    for i in range(N):
        if subset & (1 << i):
            vals.append(float(i + 1))

    if not vals:
        return 0.0

    best_expected = 0.0
    for t in range(1, 1000):  # Reasonable limit
        mean = sum(vals) / len(vals)
        threshold = best_expected
        expected = sum(max(v, threshold) for v in vals) / len(vals)
        profit = expected - c * t
        if profit < best_expected - c * (t - 1):
            break
        best_expected = profit

    return max(0.0, best_expected)


def solve() -> int:
    """Solve Problem 470."""
    N = 20
    ans = 0.0

    for c in range(N + 1):
        R_vals = [0.0] * (1 << N)
        for subset in range(1, 1 << N):
            R_vals[subset] = R(subset, float(c), N)

        for d in range(4, N + 1):
            A = [0.0] * (d + 1)
            B = [1.0] * (d + 1)
            C = [0.0] * (d + 1)
            D = [0.0] * (d + 1)

            for i in range(1, d + 1):
                A[i] = -(d - i + 1) / d
            for i in range(1, d):
                C[i] = -(i + 1) / d

            for subset in range(1, 1 << d):
                bit_count = bin(subset).count("1")
                D[bit_count] += R_vals[subset]

            x = tridiagonal_system(A, B, C, D)
            ans += x[d]

    return int(round(ans))


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
