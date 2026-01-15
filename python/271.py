"""Project Euler Problem 271: Modular Cubes, part 1.

Find the sum of all cube roots of 1 (mod N), other than 1 itself.
"""

from __future__ import annotations

from itertools import product
from math import isqrt
from typing import List, Tuple


def ext_gcd(a: int, b: int) -> Tuple[int, int, int]:
    """Extended Euclidean algorithm."""
    if b == 0:
        return (a, 1, 0)
    g, x1, y1 = ext_gcd(b, a % b)
    return (g, y1, x1 - (a // b) * y1)


def crt(remainders: List[int], moduli: List[int]) -> int:
    """Chinese Remainder Theorem."""
    if not remainders:
        return 0

    result = remainders[0]
    mod = moduli[0]

    for r, m in zip(remainders[1:], moduli[1:]):
        g, x, _ = ext_gcd(mod, m)
        if (r - result) % g != 0:
            return 0
        result += mod * x * ((r - result) // g)
        mod = mod * m // g
        result %= mod

    return result


def prime_factors(n: int) -> List[int]:
    """Get distinct prime factors."""
    factors = []
    d = 2
    while d * d <= n:
        if n % d == 0:
            factors.append(d)
            while n % d == 0:
                n //= d
        d += 1
    if n > 1:
        factors.append(n)
    return factors


def solve() -> int:
    """Solve Problem 271."""
    N = 13082761331670030

    mods = prime_factors(N)
    cube_roots_list: List[List[int]] = []

    for mod in mods:
        cube_roots = []
        for i in range(mod):
            if (i * i * i) % mod == 1:
                cube_roots.append(i)
        cube_roots_list.append(cube_roots)

    ans = 0
    for cube_roots in product(*cube_roots_list):
        ans += crt(list(cube_roots), mods)

    return ans - 1  # Exclude 1 itself


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
