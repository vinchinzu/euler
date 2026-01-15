"""Project Euler Problem 466: Distinct products.

Find the number of distinct products m*n for 1 ≤ m ≤ K and 1 ≤ n ≤ N.
"""

from __future__ import annotations

from math import gcd
from typing import List


def preff(limit: int) -> List[int]:
    """Precompute largest prime factor."""
    ff = [0] * (limit + 1)
    for i in range(2, limit + 1):
        if ff[i] == 0:
            for j in range(i, limit + 1, i):
                ff[j] = i
    return ff


def solve() -> int:
    """Solve Problem 466."""
    N = 10**16
    K = 64
    ff = preff(K)

    def num_not_divisible_by(n: int, factors: List[int]) -> int:
        """Count numbers not divisible by any factor."""
        # Remove factors of 1
        factors = [f for f in factors if f != 1]
        if not factors:
            return n

        # Find common prime factor
        for i in range(len(factors)):
            for j in range(i + 1, len(factors)):
                g = gcd(factors[i], factors[j])
                if g > 1:
                    p = ff[g]
                    if p > 1:
                        new_factors1 = []
                        new_factors2 = [p]
                        for factor in factors:
                            if factor % p == 0:
                                new_factors1.append(factor // p)
                            else:
                                new_factors1.append(factor)
                                new_factors2.append(factor)
                        return num_not_divisible_by(
                            n // p, new_factors1
                        ) + num_not_divisible_by(n, new_factors2)

        # Inclusion-exclusion
        result = 0
        for subset in range(1 << len(factors)):
            count = n
            sign = 1
            for i, factor in enumerate(factors):
                if subset & (1 << i):
                    count //= factor
                    sign = -sign
            result += sign * count
        return result

    ans = 0
    for m in range(1, K + 1):
        factors = []
        for i in range(m + 1, K + 1):
            factors.append(i // gcd(i, m))
        ans += num_not_divisible_by(N, factors)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
