"""Project Euler Problem 622: Riffle Shuffles.

Find the sum of all even n such that a deck of n cards will be restored to its
original configuration after N perfect riffle shuffles.

A perfect riffle shuffle maps the last card n-1 to itself, and every other card
k to 2k (mod n-1). This means the deck is restored after S(n) shuffles, then
S(n) is the order of 2 (mod n-1). This means that the only candidates for n-1
are the divisors of 2^60 - 1.
"""

from __future__ import annotations

from sympy import factorint, divisors


def order(base: int, n: int) -> int:
    """Compute the order of base modulo n."""
    if n == 1:
        return 1
    phi_n = n
    factors = factorint(n)
    for p in factors:
        phi_n = phi_n // p * (p - 1)

    result = phi_n
    factors_phi = factorint(phi_n)
    for p in factors_phi:
        exp = factors_phi[p]
        for _ in range(exp):
            if pow(base, result // p, n) == 1:
                result //= p
            else:
                break
    return result


def solve() -> int:
    """Solve Problem 622."""
    N = 60
    L = 2**N - 1

    all_divs = divisors(L)
    ans = 0
    for divisor in all_divs:
        if divisor > 1 and order(2, divisor) == N:
            ans += divisor + 1
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
