"""Project Euler Problem 464: Möbius function and balanced pairs.

Find the number of pairs 1 ≤ a ≤ b ≤ n such that the P(a, b), the number of
integers n in [a, b] where μ(n) = 1, and N(a, b), the number of integers where
μ(n) = -1, satisfy 99 N(a,b) ≤ 100 P(a,b) and 99 P(a,b) ≤ 100 N(a,b).
"""

from __future__ import annotations

from math import isqrt
from typing import List


class BIT:
    """Fenwick tree."""

    def __init__(self, size: int, max_val: int) -> None:
        """Initialize BIT."""
        self.size = size
        self.tree = [0] * (size + 1)
        self.max_val = max_val

    def add(self, idx: int, val: int) -> None:
        """Add value at index."""
        idx += 1
        while idx <= self.size:
            self.tree[idx] += val
            idx += idx & -idx

    def sum(self, idx: int) -> int:
        """Prefix sum up to index."""
        idx += 1
        result = 0
        while idx > 0:
            result += self.tree[idx]
            idx -= idx & -idx
        return result


def pre_mobius(limit: int) -> List[int]:
    """Precompute Möbius function."""
    mu = [1] * (limit + 1)
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False

    for i in range(2, limit + 1):
        if is_prime[i]:
            for j in range(i, limit + 1, i):
                is_prime[j] = False
                if j % (i * i) == 0:
                    mu[j] = 0
                else:
                    mu[j] = -mu[j]
    return mu


def tr(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def solve() -> int:
    """Solve Problem 464."""
    N = 20_000_000
    K = 100
    L = K * isqrt(N)
    mobius = pre_mobius(N)

    ans = tr(N)
    for sign in [1, -1]:
        f = 0
        bit = BIT(N + L, 2**63 - 1)
        for b in range(1, N + 1):
            bit.add(f + L, 1)
            if mobius[b] == sign:
                f += K
            elif mobius[b] == -sign:
                f -= K - 1
            ans -= bit.sum(N + L) - bit.sum(f + L)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
