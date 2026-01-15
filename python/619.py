"""Project Euler Problem 619: Perfect subsets.

Find the number of non-empty subsets of {A, A+1, ... B} where the product of
the elements is a perfect square.

For each prime p, the subset must contain an even number of factors of p, which
gives an equation in mod 2 for every prime that divides at least one of the
values. If there are P such primes, then these equations can be expressed as
M ["A" "A+1" ... "B"]ᵀ = [0 0 ... 0]ᵀ, where M is a P x (B-A+1) matrix. For
large enough values of B-A, all rows of M are independent. This means that M
has rank P, and there are 2^{B-A+1 - P} solutions. Subtracting one for the
empty subset gives the answer.
"""

from __future__ import annotations

from sympy import primerange


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def solve() -> int:
    """Solve Problem 619."""
    A = 1_000_000
    B = 1_234_567
    M = 10**9 + 7

    rank = 0
    primes = list(primerange(2, B + 1))
    for p in primes:
        if (A - 1) // p != B // p:
            rank += 1

    ans = pow_mod(2, B - A + 1 - rank, M) - 1
    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
