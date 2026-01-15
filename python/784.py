"""Project Euler Problem 784: Reciprocal Pairs.

A pair (p, q) is a reciprocal pair if there exists an integer r < p such that
r ≡ p⁻¹ (mod q) and r ≡ q⁻¹ (mod p). Find the sum of p+q for all reciprocal
pairs (p,q) with p ≤ N.

There exist integers a,b such that rp = 1 + qa and rq = 1 + pb, so
rp - qa = rq - pb => p(r + b) = q(r + a). Since (p,q)=1, we must have
q|r+b => r+b=kq for some integer k => rq = 1 + p(kq-r) => r(p+q) = pqk+1.
Since r < p, the left hand side is at most p²+pq < 2pq, so k=1 and
r(p+q) = pq => (p-r)(q-r) = r²-1. So we can iterate over all r, and
enumerate possible (p,q) from the factors of r²-1.
"""

from __future__ import annotations

from typing import List, Set

from sympy import factorint, primerange


def pre_ff(limit: int) -> List[int]:
    """Precompute smallest prime factor."""
    ff = [0] * (limit + 1)
    for i in range(2, limit + 1):
        if ff[i] == 0:
            ff[i] = i
            for j in range(2 * i, limit + 1, i):
                if ff[j] == 0:
                    ff[j] = i
    return ff


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def prime_factor(n: int) -> Set[int]:
    """Return set of prime factors of n."""
    factors = set()
    temp = n
    for p in primerange(2, int(n**0.5) + 1):
        if temp % p == 0:
            factors.add(p)
            while temp % p == 0:
                temp //= p
    if temp > 1:
        factors.add(temp)
    return factors


def all_divisors(n: int, exclude_primes: Set[int] = None) -> List[int]:
    """Return all divisors of n, optionally excluding those divisible by exclude_primes."""
    if exclude_primes is None:
        exclude_primes = set()

    divisors = []
    for i in range(1, int(n**0.5) + 1):
        if n % i == 0:
            # Check if divisible by excluded primes
            include_i = True
            for p in exclude_primes:
                if i % p == 0:
                    include_i = False
                    break
            if include_i:
                divisors.append(i)

            j = n // i
            if j != i:
                include_j = True
                for p in exclude_primes:
                    if j % p == 0:
                        include_j = False
                        break
                if include_j:
                    divisors.append(j)

    return sorted(divisors)


def solve() -> int:
    """Solve Problem 784."""
    N = 2 * 10**6
    ff = pre_ff(N + 1)

    ans = 0
    for r in range(2, N + 1):
        n = sq(r) - 1
        exclude_primes = prime_factor(r - 1).union(prime_factor(r + 1))
        for d in all_divisors(n, exclude_primes):
            p = d + r
            q = n // d + r
            if p < q and p <= N:
                ans += p + q

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
