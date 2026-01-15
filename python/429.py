"""Project Euler Problem 429: Unitary divisors.

A unitary divisor d is a divisor of n such that GCD(d, n/d) = 1. Find
S(N), the sum of the squares of the unitary divisors of N!.

For every prime divisor p of N!, we can compute c(N, p), the number of
times p divides N!. A unitary divisor must have either none of the factors
p, or all c(N, p) of them. We can then factor the sum of the squares of
these 2^k divisors:

S(N) = Π_p (1 + p^{2c(N, p)}).
"""

from __future__ import annotations

from sympy import primerange


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Compute base^exp mod mod."""
    result = 1
    base %= mod
    while exp:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def num_factors_in_factorial(n: int, p: int) -> int:
    """Count how many times prime p divides n!."""
    count = 0
    power = p
    while power <= n:
        count += n // power
        power *= p
    return count


def solve() -> int:
    """Solve Problem 429."""
    N = 10**8
    M = 10**9 + 9

    ans = 1
    for p in primerange(2, N + 1):
        exp = 2 * num_factors_in_factorial(N, p)
        ans = (ans * (1 + pow_mod(p, exp, M))) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
