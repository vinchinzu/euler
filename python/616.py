"""Project Euler Problem 616: Creative numbers.

Find the sum of all numbers n ≤ N which can be transformed into any other
integer m with the transformations (a, b) => a^b and a^b => (a, b).

For any integer a^{b*c} or (a*b)^c with a,b,c ≥ 2 and one integer strictly
greater than 2, we can do the following: (a, b, c) => (a^b)^{b^{c-1}} =>
(a, b, b, c-1), and continue decrementing values until we obtain (2, 2, 3).
We can then add more 2s via: (2, 2, 3) => 2^9 = 8^3 => 3^8 = 9^4 =>
(2, 2, 2, 3). This allows us to generate any 2^m, from which we can obtain
any integer m.
"""

from __future__ import annotations

from math import isqrt

from sympy import isprime, primerange


def pow_int(base: int, exp: int) -> int:
    """Integer power."""
    result = 1
    for _ in range(exp):
        result *= base
    return result


def solve() -> int:
    """Solve Problem 606."""
    N = 10**12
    primes_set = set(primerange(2, isqrt(N) + 1))

    nums = set()
    a = 2
    while a * a <= N:
        b = 2
        while pow_int(a, b) <= N:
            if not (isprime(a) and isprime(b)):
                nums.add(pow_int(a, b))
            b += 1
        a += 1

    nums.discard(16)
    return sum(nums)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
