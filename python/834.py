"""Project Euler Problem 834: Add and Divide.

Define a sequence starting with a positive integer n and incrementing by
(n+m) at step m. Let T(n) be the sum of all indices m such that term m is
divisible by n+m. Find Σ_{n=3}^N T(n).

The closed formula of the sequence f is f(m) = n(m+1) + m(m+1)/2, which
can be written as

f(m) = 1/2 ((n+m)(n+m+1) - n(n-1)),

so n+m | n(n-1) is a necessary condition for term m to be divisible by m,
so we iterate only over divisors d of n(n-1). We can then add m=d-n only
if (n+m+1) - n(n-1)/(n+m) is even.
"""

from __future__ import annotations

from math import isqrt
from typing import List, Set


def sieve_spf(limit: int) -> List[int]:
    """Smallest prime factor sieve."""
    spf = list(range(limit + 1))
    spf[0] = spf[1] = 0
    for i in range(2, isqrt(limit) + 1):
        if spf[i] == i:
            for j in range(i * i, limit + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf


def get_prime_factors(n: int, spf: List[int]) -> Set[int]:
    """Get prime factors of n."""
    factors: Set[int] = set()
    while n > 1:
        p = spf[n]
        factors.add(p)
        while n % p == 0:
            n //= p
    return factors


def get_all_divisors(n: int, prime_factors: Set[int]) -> List[int]:
    """Get all divisors of n."""
    if n == 1:
        return [1]
    divisors = [1]
    temp = n
    for p in prime_factors:
        if temp % p == 0:
            count = 0
            while temp % p == 0:
                temp //= p
                count += 1
            new_divs = []
            power = 1
            for _ in range(count + 1):
                for d in divisors:
                    new_divs.append(d * power)
                power *= p
            divisors = new_divs
    return sorted(divisors)


def solve() -> int:
    """Solve Problem 834."""
    N = 1234567

    spf = sieve_spf(N)
    ans = 0

    for n in range(3, N + 1):
        factors_n = get_prime_factors(n, spf)
        factors_n_minus_1 = get_prime_factors(n - 1, spf)
        all_factors = factors_n | factors_n_minus_1
        divisors = get_all_divisors(n * (n - 1), all_factors)
        for d in divisors:
            if d > n:
                m = d - n
                if (d + 1 - (n * (n - 1)) // d) % 2 == 0:
                    ans += m

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
