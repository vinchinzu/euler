"""Project Euler Problem 768: Chandelier Balance.

Find the number of ways to place K candles around a chandelier with N evenly
spaced candle spaces, such that the chandelier is balanced.

For small values of N we can brute force by finding all subsets of candle spaces
that, when represented as complex numbers, sum to zero. We can avoid floating
point arithmetic by working in mod p, where p≡1 (mod N) such that there is a
Nth root of unity ω and the candle spaces can be represented by powers ω^e.

For efficiency, we can compute the sums of subsets of half of the candle spaces.
The other half have the same sums by symmetry, so we only need to consider two
half-configurations with the same sum.

Finally, for the large value of N, the chandelier consists of N/rad(N)
independent rings of rad(N) candle spaces. So we can compute the number of
configurations for rad(N) spaces, and use those values to compute the number of
configurations for N spaces.
"""

from __future__ import annotations

import random
from typing import List

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


def generator(p: int) -> int:
    """Find a generator modulo p."""
    phi = p - 1
    factors = []
    temp = phi
    for i in range(2, int(temp**0.5) + 1):
        if temp % i == 0:
            factors.append(i)
            while temp % i == 0:
                temp //= i
    if temp > 1:
        factors.append(temp)

    for g in range(2, p):
        is_generator = True
        for factor in factors:
            if pow_mod(g, phi // factor, p) == 1:
                is_generator = False
                break
        if is_generator:
            return g
    return 2


def pows(base: int, n: int, mod: int) -> List[int]:
    """Compute powers of base: [base^0, base^1, ..., base^(n-1)]."""
    result = [1]
    for _ in range(1, n):
        result.append((result[-1] * base) % mod)
    return result


def rad(n: int) -> int:
    """Radical of n (product of distinct prime factors)."""
    result = 1
    for p in primerange(2, n + 1):
        if n % p == 0:
            result *= p
    return result


class LPolynomial:
    """Laurent polynomial."""

    def __init__(self, coeffs: List[int]) -> None:
        """Initialize with coefficients."""
        self.coefficients = coeffs[:]

    def pow(self, exp: int, mod_poly: "LPolynomial", mod: int) -> "LPolynomial":
        """Raise polynomial to exp-th power modulo mod_poly."""
        if exp == 0:
            return LPolynomial([1])
        if exp == 1:
            return self
        half = self.pow(exp // 2, mod_poly, mod)
        result = self._multiply(half, half, mod)
        if exp % 2 == 1:
            result = self._multiply(result, self, mod)
        # Cap to mod_poly degree
        max_deg = len(mod_poly.coefficients) - 1
        if len(result.coefficients) > max_deg:
            result.coefficients = result.coefficients[: max_deg + 1]
        return result

    def _multiply(self, a: "LPolynomial", b: "LPolynomial", mod: int) -> "LPolynomial":
        """Multiply two polynomials."""
        n = len(a.coefficients)
        m = len(b.coefficients)
        result = [0] * (n + m - 1)
        for i in range(n):
            for j in range(m):
                result[i + j] = (result[i + j] + a.coefficients[i] * b.coefficients[j]) % mod
        return LPolynomial(result)

    def shift_up(self, n: int) -> "LPolynomial":
        """Shift polynomial up by n degrees."""
        return LPolynomial([0] * n + self.coefficients)


def solve() -> int:
    """Solve Problem 768."""
    N = 360
    K = 20
    L = rad(N)

    # Find prime p such that p ≡ 1 (mod L)
    random.seed(0)
    while True:
        # Generate a probable prime around 2^31
        p = random.randint(2**30, 2**31 - 1)
        # Simple primality test (in practice use better test)
        is_prime = True
        for i in range(2, int(p**0.5) + 1):
            if p % i == 0:
                is_prime = False
                break
        if is_prime and p % L == 1:
            break

    g = generator(p)
    ws = pows(pow_mod(g, (p - 1) // L, p), L, p)

    # Count subsets of first half
    all_counts: dict[int, List[int]] = {}
    for subset in range(1 << (L // 2)):
        weight = 0
        for i in range(L // 2):
            if subset & (1 << i):
                weight = (weight + ws[i]) % p
        bit_count = bin(subset).count("1")
        if weight not in all_counts:
            all_counts[weight] = [0] * (L + 1)
        all_counts[weight][bit_count] += 1

    # Count balanced configurations
    num_balanced = [0] * (K + 1)
    for counts in all_counts.values():
        for num_candles1 in range(K + 1):
            for num_candles2 in range(K + 1 - num_candles1):
                num_balanced[num_candles1 + num_candles2] += (
                    counts[num_candles1] * counts[num_candles2]
                )

    # Compute polynomial and raise to N/L power
    poly = LPolynomial(num_balanced)
    mod_poly = LPolynomial([1] + [0] * K)
    result_poly = poly.pow(N // L, mod_poly.shift_up(K + 1), 2**63 - 1)

    return result_poly.coefficients[K] if K < len(result_poly.coefficients) else 0


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
