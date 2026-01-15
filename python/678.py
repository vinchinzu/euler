"""Project Euler Problem 678: Fermat-like Equations.

Find the number of positive integer tuples (a,b,c,e,f) such that a^e + b^e = c^f
for some a<b, e≥2, f≥3, and c^f≤N.

We can iterate over all possible c^f; there are O(³√N) of them.

For e=2, we can use the standard algorithm to find the number of ways that c^f
can be written as the sum of two squares.

For e=3, we write c^f = a³+b³ = (a+b)(a²-ab+b²). For each divisor d of c^f,
if a+b=d and a²-ab+b²=c^f/d, then solving for a gives

a = (1/2) (d - √((4/3) c^f/d - (1/3) d²)).

This means we have a solution if the discriminant is a perfect square, and the
square root is smaller than d.

For e=4, we can filter the ways that c^f can be written as the sum of two squares.

For e≥5, we precompute at the beginning all the ways that each integer can be
written as a^e + b^e. There are only O(N^(1/5)) such a^e and b^e, so this part
takes O(N^(2/5)) total.
"""

from __future__ import annotations

from collections import Counter, defaultdict
from dataclasses import dataclass
from math import isqrt
from typing import Set


@dataclass(frozen=True)
class Point:
    """Integer point."""

    x: int
    y: int


def is_sq(n: int) -> bool:
    """Check if n is a perfect square."""
    if n < 0:
        return False
    root = isqrt(n)
    return root * root == n


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def pow_mod(base: int, exp: int, mod: int | None = None) -> int:
    """Modular exponentiation."""
    result = 1
    while exp > 0:
        if exp & 1:
            result = result * base
        base = base * base
        exp >>= 1
    return result


def sums_of_two_squares(prime_factors: list[int]) -> Set[Point]:
    """Find all representations of n as sum of two squares.

    Given prime factorization, returns set of (x,y) with x^2+y^2=n and x>0, y>0.
    """
    # Simple implementation: iterate over possible x values
    n = 1
    for p in prime_factors:
        n *= p

    result: Set[Point] = set()
    for x in range(1, isqrt(n) + 1):
        rem = n - x * x
        if rem < 0:
            break
        y = isqrt(rem)
        if y * y == rem and y > 0:
            result.add(Point(min(x, y), max(x, y)))
    return result


def all_divisors(n: int, prime_factors: list[int]) -> list[int]:
    """Get all divisors of n given prime factorization."""
    # Use prime factors to generate divisors
    divisors = [1]
    factor_counts = Counter(prime_factors)
    
    for prime, count in factor_counts.items():
        new_divisors = []
        for d in divisors:
            power = 1
            for _ in range(count + 1):
                new_divisors.append(d * power)
                power *= prime
        divisors = new_divisors
    
    return sorted(set(divisors))


def solve() -> int:
    """Solve Problem 678."""
    N = 10**18
    
    # Precompute smallest prime factors up to cube root of N
    limit = int(N**(1/3)) + 1
    ff = list(range(limit + 1))
    for i in range(2, int(limit**0.5) + 1):
        if ff[i] == i:
            for j in range(i * i, limit + 1, i):
                if ff[j] == j:
                    ff[j] = i

    # Precompute for e≥5
    counts: dict[int, Counter[int]] = defaultdict(Counter)
    for e in range(5, 100):  # Reasonable upper bound
        if 1 << e >= N:
            break
        pows: list[int] = []
        a = 1
        while True:
            power = pow(a, e)
            if power >= N:
                break
            pows.append(power)
            a += 1
        
        for i in range(len(pows)):
            for j in range(i + 1, len(pows)):
                cf = pows[i] + pows[j]
                if cf <= N:
                    counts[e][cf] += 1

    ans = 0
    
    for f in range(3, 100):  # Reasonable upper bound
        if pow(2, f) > N:
            break
        c = 2
        while True:
            cf = pow(c, f)
            if cf > N:
                break

            # Get prime factors
            unsorted_prime_factors: list[int] = []
            cc = c
            while cc > 1:
                prime = ff[cc]
                for _ in range(f):
                    unsorted_prime_factors.append(prime)
                while cc % prime == 0:
                    cc //= prime

            # e=2: sums of two squares
            sums = sums_of_two_squares(unsorted_prime_factors)
            for p in sums:
                if p.x > 0 and p.x < p.y:
                    ans += 1

            # e=3: cubic sums
            divisors = all_divisors(cf, unsorted_prime_factors)
            for d in divisors:
                if d * d * d >= 4 * cf:
                    continue
                disc = 4 * cf // d - sq(d)
                if disc < 3 * sq(d) and is_sq(3 * disc):
                    ans += 1

            # e=4: sums of two fourth powers (filter sums of two squares)
            for p in sums:
                if p.x > 0 and p.x < p.y and is_sq(p.x) and is_sq(p.y):
                    ans += 1

            # e≥5: use precomputed counts
            for e in range(5, 100):
                if pow(2, e) >= cf:
                    break
                if e in counts:
                    ans += counts[e][cf]

            c += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
