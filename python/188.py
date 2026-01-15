"""Project Euler Problem 188: The hyperexponentiation of a number."""

from typing import Dict
import math

MOD = 100_000_000
BASE = 1777
HEIGHT = 1855

PHI_CACHE: Dict[int, int] = {}


def phi(n: int) -> int:
    """Euler's totient function."""
    if n == 1:
        return 1
    if n in PHI_CACHE:
        return PHI_CACHE[n]
    result = n
    temp = n
    i = 2
    while i * i <= temp:
        if temp % i == 0:
            while temp % i == 0:
                temp //= i
            result -= result // i
        i += 1
    if temp > 1:
        result -= result // temp
    PHI_CACHE[n] = result
    return result


def mod_pow(base: int, exponent: int, mod: int) -> int:
    """Modular exponentiation."""
    if mod == 1:
        return 0
    result = 1 % mod
    base %= mod
    while exponent > 0:
        if exponent % 2 == 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exponent >>= 1
    return result


def tetration(a: int, height: int, mod: int) -> int:
    """Compute tetration a^(a^(...)) mod mod."""
    if mod == 1:
        return 1 % mod
    if height == 1:
        return a % mod

    phi_mod = phi(mod)
    exponent = tetration(a, height - 1, phi_mod)
    if exponent < phi_mod and math.gcd(a, mod) != 1:
        exponent += phi_mod
    return mod_pow(a, exponent, mod)


def main() -> int:
    """Main function."""
    return tetration(BASE, HEIGHT, MOD)


if __name__ == "__main__":
    print(main())
