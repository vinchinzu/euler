"""Project Euler Problem 805: Shifted Multiple.

Let N(r) be the smallest positive integer n such that s(n) = r*n, where
s(n) is obtained by moving the leftmost digit to the rightmost position.
Find Σ N(u³/v³) over all co-prime u,v not exceeding N.

Let the integer n have k+1 digits, the first one is a and the remainder
is b. Then we have

u³/v³ (a 10^k + b) = 10b + a
=> (u³10^k-v³)a = (10v³-u³)b = cb,

where c = 10v³-u³. Firstly, we need b ≤ 10^k, so u³a ≤ c. Then, for each
a, we need to find k such that a(u³10^k-v³) is divisible by c. We can see
that if k=-1, then this value is equal to a(c/10) ≡ 0 (mod c), so the
smallest positive k is T-1 where T is the order of k (mod c / gcd(a,c)).
For each k we can compute the value of b directly.

We iterate over all digits a to find the one with minimum k. For efficiency,
we don't recompute orders if we've already seen a mod.
"""

from __future__ import annotations

import math
from typing import Dict


def gcd(a: int, b: int) -> int:
    """Greatest common divisor."""
    while b:
        a, b = b, a % b
    return a


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def mod_inverse(a: int, m: int) -> int:
    """Modular inverse using extended Euclidean algorithm."""
    if math.gcd(a, m) != 1:
        raise ValueError("Inverse does not exist")
    return pow(a, -1, m)


def order(base: int, mod: int) -> int:
    """Find the order of base modulo mod."""
    if math.gcd(base, mod) != 1:
        raise ValueError("Base and mod must be coprime")
    phi = mod
    # Factorize phi to find order
    factors = []
    temp = phi
    for i in range(2, int(temp**0.5) + 1):
        if temp % i == 0:
            count = 0
            while temp % i == 0:
                temp //= i
                count += 1
            factors.append((i, count))
    if temp > 1:
        factors.append((temp, 1))

    ord_val = phi
    for p, _ in factors:
        while ord_val % p == 0:
            if pow_mod(base, ord_val // p, mod) == 1:
                ord_val //= p
            else:
                break

    return ord_val


def solve() -> int:
    """Solve Problem 805."""
    N = 200
    M = 10**9 + 7
    B = 10

    ans = 0
    order_cache: Dict[int, int] = {}

    for v in range(1, N + 1):
        for u in range(1, N + 1):
            if gcd(u, v) != 1:
                continue

            u3 = u * u * u
            v3 = v * v * v
            min_k = float("inf")
            min_mod_val = float("inf")
            n = 0

            for a in range(1, B):
                c = B * v3 - u3
                if u3 * a > c:
                    continue

                mod_val = c // gcd(a, c)
                if mod_val >= min_mod_val:
                    continue

                min_mod_val = mod_val

                if mod_val not in order_cache:
                    order_cache[mod_val] = order(B, mod_val)
                k = order_cache[mod_val] - 1

                if k < min_k:
                    min_k = k
                    term1 = (a * pow_mod(B, k, M)) % M
                    term2 = (
                        (u3 * pow_mod(B, k, M) - v3)
                        * a
                        % M
                        * mod_inverse(c, M)
                        % M
                    )
                    n = (term1 + term2) % M

            ans = (ans + n) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
