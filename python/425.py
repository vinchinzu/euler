"""Project Euler Problem 425: Prime connection.

A and B are connected if they have the same length and differ by exactly one
digit, or adding one digit to the left of A makes B (or vice versa). A prime P
is 2's relative if there exists a chain of connected primes from 2 to P, none
of which exceed P. Find the sum of all primes ≤ N that are not 2's relative.

We iterate over all primes from 2 to N in increasing order, maintaining a union
find data structure. For each prime p, we can union it with all primes that are
connected to it. Then it is 2's relative if at that point, it is in the same
union-find component as 2.

To find all primes connected to p, we go through each digit d, and subtract all
possible numbers from 1 to d (multiplied by the appropriate power of ten). The
one exception is that if we subtract d from the first digit, we must make sure
that the second digit is not zero. This can be verified by ensuring the result
is at least 10^{k-2} = 10^{k-1} / 10, where k is the number of digits in p.
"""

from __future__ import annotations

from math import isqrt
from typing import List


class UnionFind:
    """Union-Find data structure."""

    def __init__(self, n: int) -> None:
        """Initialize with n elements."""
        self.parent: List[int] = list(range(n))
        self.size: List[int] = [1] * n

    def find(self, x: int) -> int:
        """Find root with path compression."""
        while self.parent[x] != x:
            self.parent[x] = self.parent[self.parent[x]]
            x = self.parent[x]
        return x

    def union(self, a: int, b: int) -> None:
        """Union two sets."""
        root_a = self.find(a)
        root_b = self.find(b)
        if root_a == root_b:
            return

        if self.size[root_a] < self.size[root_b]:
            self.parent[root_a] = root_b
            self.size[root_b] += self.size[root_a]
        else:
            self.parent[root_b] = root_a
            self.size[root_a] += self.size[root_b]


def sieve(n: int) -> List[bool]:
    """Sieve of Eratosthenes."""
    is_prime = [True] * (n + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(n) + 1):
        if is_prime[i]:
            for j in range(i * i, n + 1, i):
                is_prime[j] = False
    return is_prime


def digits(n: int) -> List[int]:
    """Get digits of n."""
    result: List[int] = []
    while n > 0:
        result.append(n % 10)
        n //= 10
    return result[::-1]


def pow_10(n: int) -> int:
    """Compute 10^n."""
    result = 1
    for _ in range(n):
        result *= 10
    return result


def solve() -> int:
    """Solve Problem 425."""
    N = 10**7
    B = 10

    is_prime = sieve(N)
    primes = [i for i in range(2, N + 1) if is_prime[i]]

    pow_bs = [pow_10(i) for i in range(len(digits(N)) + 1)]

    uf = UnionFind(N + 1)

    ans = 0
    for p in primes:
        p_digits = digits(p)
        for i in range(len(p_digits)):
            for d in range(1, p_digits[i] + 1):
                relative = p - d * pow_bs[len(p_digits) - 1 - i]
                if relative >= pow_bs[len(p_digits) - 1] // B and is_prime[relative]:
                    uf.union(p, relative)

        if uf.find(2) != uf.find(p):
            ans += p

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
