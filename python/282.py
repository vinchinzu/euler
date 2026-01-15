"""Project Euler Problem 282: Ackermann Function.

Let A(m, n) = n + 1 if m=0, A(m-1, 1) if n=0, and A(m-1, A(m, n-1))
otherwise. Find sum_{n=0}^6 A(n, n).

We compute A(n, n) for n≤3 by brute force. It turns out that A(4, 4) = (2 ^^
7) - 3 (https://en.wikipedia.org/wiki/Ackermann_function), where x ^^ y =
x^(x^(x^(...))) is Knuth's double-arrow notation, or exponentiation of x by
itself y times. This can be computed using Euler's identity b^e (mod M) ≡
b^(e (mod ϕ(M))).

Similarly, A(5, 5) = (2 ^^^ 8) - 3, where x ^^^ y = (x^^(x^^(...))) is
applying the double arrow operation y times. We can see that 2 ^^^ 8 = 2 ^^
(a big number), and the exact number does not matter, because applying the
phi operation to the modulus when using Euler's identity will bring the
modulus down to 2 in much fewer steps. At that point, the result must be 0
regardless of how many more double arrow operations need to be applied.

For larger values of n, A(n, n) is 2 ^^ (a big number) for even larger
numbers, so they are all constant (mod M).
"""

from __future__ import annotations

from math import gcd
from typing import Callable


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    if mod == 1:
        return 0
    result = 1
    base %= mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        exp //= 2
        base = (base * base) % mod
    return result


def totient(n: int) -> int:
    """Euler's totient function."""
    result = n
    p = 2
    while p * p <= n:
        if n % p == 0:
            while n % p == 0:
                n //= p
            result -= result // p
        p += 1
    if n > 1:
        result -= result // n
    return result


def ackermann_brute(m: int, n: int) -> int:
    """Brute force Ackermann function for small values."""
    if m == 0:
        return n + 1
    if n == 0:
        return ackermann_brute(m - 1, 1)
    return ackermann_brute(m - 1, ackermann_brute(m, n - 1))


def f(n: int, mod: int) -> int:
    """Compute 2 ^^ n mod mod (tower of 2s)."""
    if n == 0:
        return 1
    if mod == 2:
        return 0
    phi_mod = totient(mod)
    return pow_mod(2, f(n - 1, phi_mod), mod)


def ackermann_mod(n: int, mod: int) -> int:
    """Compute A(n, n) mod mod."""
    if n <= 3:
        return ackermann_brute(n, n) % mod
    if n == 4:
        return (f(7, mod) - 3) % mod
    # For n >= 5, the result is constant mod M
    # We use a large number to represent the tower
    return (f(2**63, mod) - 3) % mod


def solve() -> int:
    """Solve Problem 282."""
    N = 6
    M = 14**8

    ans = 0
    for n in range(N + 1):
        ans = (ans + ackermann_mod(n, M)) % M

    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    print(result)


if __name__ == "__main__":
    main()
