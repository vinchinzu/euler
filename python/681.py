"""Project Euler Problem 681: Maximal Area.

Find the sum a+b+c+d of all integer quadruples a≤b≤c≤d such that the maximum
area of a quadrilateral with sides a,b,c,d is an integer at most N.

The maximum area of a quadrilateral with sides a,b,c,d is that of a cyclic
quadrilateral, whose area is given by Brahmagupta's Formula:
K = √(s-a)(s-b)(s-c)(s-d). We can iterate over each value of K, and find four
values s-d ≤ s-c ≤ s-b ≤ s-a that multiply to K². We do this by first iterating
over divisors (s-d, s-c), and then noting the following restrictions on s-a.

First, s-a must be at least large enough such that s-b ≤ s-a =>
s-a ≥ √K²/((s-c)(s-d)).
Second, s-a must not be so large that s-b = K²/((s-a)(s-c)(s-d)) becomes
smaller than s-c.
Finally, s-a must not be so large that the quadrilateral no longer satisfies
the "triangle" inequality, s-a ≤ (s-b) + (s-c) + (s-d).

Finally, we have the parity constraint, so (s-a) + (s-b) + (s-c) + (s-d) must
be even. Given these requirements, it is straightforward to sum a+b+c+d over all
such quadrilaterals.
"""

from __future__ import annotations

from collections import Counter
from math import ceil, sqrt


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def build_spf(limit: int) -> list[int]:
    """Build smallest prime factor array."""
    spf = list(range(limit + 1))
    for i in range(2, int(limit**0.5) + 1):
        if spf[i] == i:
            for j in range(i * i, limit + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf


def prime_factor(n: int, spf: list[int]) -> dict[int, int]:
    """Get prime factorization."""
    factors: dict[int, int] = {}
    while n > 1:
        p = spf[n]
        count = 0
        while n % p == 0:
            n //= p
            count += 1
        factors[p] = count
    return factors


def all_divisors(n: int, prime_factors: set[int]) -> list[int]:
    """Get all divisors given prime factors."""
    factor_counts = Counter(prime_factors)
    divisors = [1]
    for prime, count in factor_counts.items():
        new_divisors = []
        for d in divisors:
            power = 1
            for _ in range(count + 1):
                new_divisors.append(d * power)
                power *= prime
        divisors = new_divisors
    return sorted(set(divisors))


def fcb(n: int) -> int:
    """Cube of n."""
    return n * n * n


def solve() -> int:
    """Solve Problem 681."""
    N = 1000000
    spf = build_spf(N)
    ans = 0

    for K in range(1, N + 1):
        k_sq = sq(K)
        factors = prime_factor(k_sq, spf)
        divisors = all_divisors(k_sq, set(factors.keys()))
        divisors.sort()

        for di, d in enumerate(divisors):
            if d**4 > k_sq:
                break
            for ci in range(di, len(divisors)):
                c = divisors[ci]
                if d * fcb(c) > k_sq:
                    break
                ab = k_sq // (c * d)
                if ab * c * d != k_sq:
                    continue
                start_val = ceil(sqrt(ab))
                start_ai = 0
                for i, div in enumerate(divisors):
                    if div >= start_val:
                        start_ai = i
                        break
                for ai in range(start_ai, len(divisors)):
                    a = divisors[ai]
                    if ab % a != 0:
                        continue
                    b = ab // a
                    total_sum = a + b + c + d
                    if b < c or total_sum <= 2 * a:
                        break
                    if a * b == ab and total_sum % 2 == 0:
                        ans += total_sum

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
